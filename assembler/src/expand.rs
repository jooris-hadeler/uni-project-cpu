use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::once,
    path::PathBuf,
};

use crate::{
    lexer::{self, LexError},
    parser::{self, ParseError, ast},
    util::{StringId, resolve},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandError {
    /// Raised when trying to invoke a macro with too many or too few arguments.
    WrongArgumentCount {
        macro_name: StringId,
        macro_span: (usize, usize),
        invokation_span: (usize, usize),
        expected: usize,
        got: usize,
    },

    /// Raised when trying to access parameter that is out of bounds.
    OutOfBounds {
        arg_span: (usize, usize),
        index: usize,
        length: usize,
    },

    /// Raised when encountering a macro with an invalid parameter count.
    InvalidParameterCount {
        macro_name: StringId,
        macro_span: (usize, usize),
    },

    /// Raised when there are multiple definitions with the same name.
    DuplicatedDefinition {
        name: StringId,
        first_span: (usize, usize),
        current_span: (usize, usize),
    },

    /// Raised when a macro is recursive.
    RecursiveMacro {
        macro_name: StringId,
        macro_span: (usize, usize),
        cycle: Vec<(StringId, (usize, usize))>,
    },

    /// Raised when an included file cannot be found.
    CantFindInclude {
        path: StringId,
        path_span: (usize, usize),
    },

    /// Raised when an error occured while parsing an include.
    Parse {
        path: PathBuf,
        content: String,
        err: ParseError,
    },

    /// Raised when an error occured while lexing an include.
    Lex {
        path: PathBuf,
        content: String,
        err: Vec<(usize, LexError, usize)>,
    },
}

/// Expand an [ast::Program] until there are no more expansions.
pub fn expand(program: ast::Program, self_name: StringId) -> Result<ast::Program, ExpandError> {
    Expander::new(self_name).expand(program)
}

struct Expander {
    macros: HashMap<StringId, ast::Macro>,
    macro_dependency: HashMap<StringId, HashSet<StringId>>,
    includes: HashSet<StringId>,
}

impl Expander {
    pub fn new(self_name: StringId) -> Self {
        Self {
            macros: HashMap::new(),
            macro_dependency: HashMap::new(),
            includes: {
                let mut set = HashSet::new();
                set.insert(self_name);
                set
            },
        }
    }

    fn expand(&mut self, program: ast::Program) -> Result<ast::Program, ExpandError> {
        let ast::Program { mut content } = program;

        loop {
            let (new_content, changed) = self.expand_once(content)?;

            content = new_content;

            if !changed {
                break;
            }
        }

        Ok(ast::Program { content })
    }

    fn expand_once(
        &mut self,
        content: Vec<ast::Content>,
    ) -> Result<(Vec<ast::Content>, bool), ExpandError> {
        let mut changed = false;
        let mut new_nodes = Vec::new();

        // TODO: ensure labels work inside macros and will not clash with multiple invokations

        for node in content {
            match node {
                ast::Content::Instruction(instruction) => {
                    // check if this is a macro invokation
                    // if not just append the instruction and continue
                    let Some(macro_) = self.macros.get(&instruction.mnemonic) else {
                        new_nodes.push(ast::Content::Instruction(instruction));
                        continue;
                    };

                    // add replacement
                    for repl in &macro_.replacement {
                        let mut args = Vec::new();

                        // resolve arguments
                        for arg in &repl.args {
                            match arg.kind {
                                ast::ArgumentKind::Parameter(index) => {
                                    // we don't need to check the index here since
                                    // the validation was done when the macro was first found

                                    args.push(
                                        instruction
                                            .args
                                            .get(index - 1)
                                            .copied()
                                            .expect("BUG: macro argument out of bounds"),
                                    );
                                }
                                ast::ArgumentKind::Identifier(_)
                                | ast::ArgumentKind::Register(_)
                                | ast::ArgumentKind::Number(_) => args.push(*arg),
                            }
                        }

                        new_nodes.push(ast::Content::Instruction(ast::Instruction {
                            mnemonic: repl.mnemonic,
                            mnemonic_span: repl.mnemonic_span,
                            args,
                        }));
                    }

                    // set changed to true to indicate that we need atleast one more iteration
                    changed = true;
                }
                ast::Content::Label(_) => new_nodes.push(node),
                ast::Content::Include(include) => {
                    new_nodes.extend(self.include_file(include)?);

                    // set changed to true to indicate that we need atleast one more iteration
                    changed = true;
                }
                ast::Content::Expanded(expanded) => {
                    let ast::Expanded {
                        invokation_span,
                        content,
                    } = expanded;

                    // expand the inner content
                    let (content, has_changed) = self.expand_once(content)?;
                    changed |= has_changed;

                    new_nodes.push(ast::Content::Expanded(ast::Expanded {
                        invokation_span,
                        content,
                    }));
                }
                ast::Content::Macro(macro_) => {
                    // check for duplicated macros
                    if let Some(prev_macro) = self.macros.get(&macro_.name) {
                        return Err(ExpandError::DuplicatedDefinition {
                            name: macro_.name,
                            first_span: prev_macro.name_span,
                            current_span: macro_.name_span,
                        });
                    }

                    // validate parameter count
                    if macro_.num_args < 0 {
                        return Err(ExpandError::InvalidParameterCount {
                            macro_name: macro_.name,
                            macro_span: macro_.name_span,
                        });
                    }

                    let num_args = macro_.num_args as usize;

                    // validate replacement
                    for repl in &macro_.replacement {
                        for arg in &repl.args {
                            let &ast::Argument {
                                kind: ast::ArgumentKind::Parameter(index),
                                span,
                            } = arg
                            else {
                                continue;
                            };

                            if index > num_args || index == 0 {
                                return Err(ExpandError::OutOfBounds {
                                    arg_span: span,
                                    index,
                                    length: num_args,
                                });
                            }
                        }
                    }

                    // insert macro dependecies
                    let entry = self.macro_dependency.entry(macro_.name).or_default();

                    for instr in &macro_.replacement {
                        entry.insert(instr.mnemonic);
                    }

                    // add the macro to the macros map
                    self.macros.insert(macro_.name, macro_);

                    // check for recursive macros
                    self.check_for_recursive_macros()?;

                    // set changed to true to indicate that we need atleast one more iteration
                    changed = true;
                }
            }
        }

        Ok((new_nodes, changed))
    }

    /// Lex and parse file then return its content
    fn include_file(&self, include: ast::Include) -> Result<Vec<ast::Content>, ExpandError> {
        // check for duplicated includes if found skip
        if self.includes.contains(&include.path) {
            return Ok(Vec::new());
        }

        let path = PathBuf::from(resolve(include.path));

        // try to read file content
        let Ok(content) = fs::read_to_string(&path) else {
            return Err(ExpandError::CantFindInclude {
                path: include.path,
                path_span: include.path_span,
            });
        };

        // tokenize the included file
        let tokens = match lexer::lex(&content) {
            Ok(tokens) => tokens,
            Err(err) => return Err(ExpandError::Lex { path, content, err }),
        };

        // parse the included file
        let program = match parser::parse(tokens) {
            Ok(prog) => prog,
            Err(err) => return Err(ExpandError::Parse { path, content, err }),
        };

        Ok(program.content)
    }

    /// This uses DFS to ensure that no cycles (recusive macros) exist in the macro_dependecy graph.
    fn check_for_recursive_macros(&self) -> Result<(), ExpandError> {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut found = false;

        fn dfs(
            node: StringId,
            graph: &HashMap<StringId, HashSet<StringId>>,
            macros: &HashMap<StringId, ast::Macro>,
            visiting: &mut HashSet<StringId>,
            visited: &mut HashSet<StringId>,
            path: &mut Vec<StringId>,
            found: &mut bool,
        ) -> Result<(), ExpandError> {
            if visiting.contains(&node) {
                // Found a cycle — raise an error
                if let Some(start_idx) = path.iter().position(|&n| n == node) {
                    let cycle: Vec<_> = path[start_idx..]
                        .iter()
                        .copied()
                        .chain(once(node))
                        .map(|id| (id, macros.get(&id).unwrap().name_span))
                        .collect();

                    let node_span = macros.get(&node).unwrap().name_span;

                    return Err(ExpandError::RecursiveMacro {
                        macro_name: node,
                        macro_span: node_span,
                        cycle,
                    });
                }
                *found = true;
                return Ok(());
            }

            if visited.contains(&node) {
                return Ok(());
            }

            visiting.insert(node);
            path.push(node);

            if let Some(neighbors) = graph.get(&node) {
                for &next in neighbors {
                    dfs(next, graph, macros, visiting, visited, path, found)?;
                }
            }

            visiting.remove(&node);
            visited.insert(node);
            path.pop();

            Ok(())
        }

        for &node in self.macro_dependency.keys() {
            if !visited.contains(&node) {
                dfs(
                    node,
                    &self.macro_dependency,
                    &self.macros,
                    &mut visiting,
                    &mut visited,
                    &mut path,
                    &mut found,
                )?;
            }
        }

        Ok(())
    }
}
