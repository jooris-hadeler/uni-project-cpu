use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::once,
    path::PathBuf,
};

use crate::{
    lexer::{self, LexError},
    parser::{self, ParseError, ast},
    util::{StringId, intern, resolve},
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

    /// Raised when encountering a parameter outside of a macro.
    ParameterOutsideOfMacro { span: (usize, usize) },

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
    macro_labels: HashMap<StringId, HashSet<StringId>>,
    definitions: HashMap<StringId, ast::Argument>,
    macro_dependency: HashMap<StringId, HashSet<StringId>>,
    includes: HashSet<StringId>,
    macro_counter: usize,
}

impl Expander {
    pub fn new(self_name: StringId) -> Self {
        Self {
            macros: HashMap::new(),
            macro_labels: HashMap::new(),
            definitions: HashMap::new(),
            macro_dependency: HashMap::new(),
            includes: {
                let mut set = HashSet::new();
                set.insert(self_name);
                set
            },
            macro_counter: 0,
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
                        let ast::Instruction {
                            mnemonic,
                            mnemonic_span,
                            args,
                        } = instruction;

                        let mut new_args = Vec::new();

                        for arg in args {
                            match arg.kind {
                                ast::ArgumentKind::Identifier(ident) => {
                                    let Some(value) = self.definitions.get(&ident) else {
                                        new_args.push(arg);
                                        continue;
                                    };

                                    new_args.push(*value);
                                }
                                _ => new_args.push(arg),
                            }
                        }

                        new_nodes.push(ast::Content::Instruction(ast::Instruction {
                            mnemonic,
                            mnemonic_span,
                            args: new_args,
                        }));

                        continue;
                    };

                    // add replacement
                    for repl in &macro_.replacement {
                        new_nodes.push(self.create_replacement(repl, &instruction.args, macro_)?);
                    }

                    self.macro_counter += 1;

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
                ast::Content::Definition(def) => {
                    // check for duplicated definitions
                    if let Some(prev_definition) = self.definitions.get(&def.name) {
                        return Err(ExpandError::DuplicatedDefinition {
                            name: def.name,
                            first_span: prev_definition.span,
                            current_span: def.name_span,
                        });
                    }

                    // validate argument kind
                    let ast::ArgumentKind::Parameter(_) = def.value.kind else {
                        return Err(ExpandError::ParameterOutsideOfMacro {
                            span: def.value.span,
                        });
                    };

                    // insert definition into lookup table
                    self.definitions.insert(def.name, def.value);

                    changed = true;
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

                    let entry = self.macro_labels.entry(macro_.name).or_default();

                    // validate replacement
                    for repl in &macro_.replacement {
                        match repl {
                            ast::Content::Instruction(instr) => {
                                for arg in &instr.args {
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
                            ast::Content::Label(label) => {
                                entry.insert(label.name);
                            }
                            _ => unreachable!(),
                        }
                    }

                    // insert macro dependecies
                    let entry = self.macro_dependency.entry(macro_.name).or_default();

                    for repl in &macro_.replacement {
                        let ast::Content::Instruction(instr) = repl else {
                            continue;
                        };

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

    fn create_replacement(
        &self,
        repl: &ast::Content,
        args: &[ast::Argument],
        macro_: &ast::Macro,
    ) -> Result<ast::Content, ExpandError> {
        match repl {
            ast::Content::Instruction(instr) => {
                let mut new_args = Vec::new();

                // resolve arguments
                for arg in &instr.args {
                    match arg.kind {
                        ast::ArgumentKind::Parameter(index) => {
                            // we don't need to check the index here since
                            // the validation was done when the macro was first found

                            new_args.push(
                                args.get(index - 1)
                                    .copied()
                                    .expect("BUG: macro argument out of bounds"),
                            );
                        }
                        ast::ArgumentKind::Identifier(ident) => {
                            // check if we are referencing a local label
                            if !self
                                .macro_labels
                                .get(&macro_.name)
                                .unwrap()
                                .contains(&ident)
                            {
                                new_args.push(*arg);
                                continue;
                            }

                            let new_label_name =
                                intern(format!("macro_{}_{}", self.macro_counter, resolve(ident),));

                            new_args.push(ast::Argument {
                                kind: ast::ArgumentKind::Identifier(new_label_name),
                                span: arg.span,
                            });
                        }
                        ast::ArgumentKind::Register(_) | ast::ArgumentKind::Number(_) => {
                            new_args.push(*arg)
                        }
                    }
                }

                Ok(ast::Content::Instruction(ast::Instruction {
                    mnemonic: instr.mnemonic,
                    mnemonic_span: instr.mnemonic_span,
                    args: new_args,
                }))
            }
            ast::Content::Label(label) => {
                let new_label_name = intern(format!(
                    "macro_{}_{}",
                    self.macro_counter,
                    resolve(label.name),
                ));

                Ok(ast::Content::Label(ast::Label {
                    name: new_label_name,
                    name_span: label.name_span,
                }))
            }
            _ => unreachable!(),
        }
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
