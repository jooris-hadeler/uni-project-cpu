use std::{collections::HashMap, mem};

use crate::{
    lexer::Span,
    parser::ast::{self, MacroNode},
    strings::StringId,
};

#[derive(Debug)]
pub enum ExpansionError {
    DuplicateMacroDefinition {
        name: StringId,
        first_span: Span,
        second_span: Span,
    },

    UnknownSymbol {
        name: StringId,
        span: Span,
    },

    InvalidArithType {
        value_span: Span,
        op_span: Span,
    },
}

pub struct Expander {
    macros: HashMap<StringId, MacroNode>,

    name: StringId,
    content: Vec<ast::Node>,
}

impl Expander {
    pub fn new(module: ast::Module) -> Self {
        let ast::Module { name, content } = module;

        Self {
            macros: HashMap::new(),
            name,
            content,
        }
    }

    pub fn expand(mut self) -> Result<ast::Module, ExpansionError> {
        self.expand_node_list(&mut self.content)?;

        Ok(ast::Module {
            name: self.name,
            content: self.content,
        })
    }

    fn expand_node_list(&mut self, list: &mut Vec<ast::Node>) -> Result<bool, ExpansionError> {
        let mut has_changed = false;

        for node_ref in list.iter_mut() {
            match node_ref {
                ast::Node::Macro(_) => {
                    // replace the macro node with an empty node and register the macro definition
                    let ast::Node::Macro(definition) = mem::replace(node_ref, ast::Node::Empty)
                    else {
                        unreachable!()
                    };

                    if let Some(previous_definition) = self.macros.get(&definition.name) {
                        return Err(ExpansionError::DuplicateMacroDefinition {
                            name: definition.name,
                            first_span: previous_definition.name_span,
                            second_span: definition.name_span,
                        });
                    }

                    self.macros.insert(definition.name, definition);
                    has_changed = true;
                }

                ast::Node::Instruction(instr) => {
                    // if there is no macro with the given name we do nothing
                    let Some(macro_def) = self.macros.get(&instr.mnemonic) else {
                        continue;
                    };

                    let invaktion_name = instr.mnemonic;
                    let invokation_span = instr.mnemonic_span;

                    // check parameter types

                    // build parameter map

                    // build replacement content

                    let mut content = Vec::new();

                    // replace
                    *node_ref = ast::Node::Expansion(ast::ExpansionNode {
                        invaktion_name,
                        invokation_span,
                        content,
                    });
                }

                ast::Node::Expansion(inner) => {
                    // if we encounter an already expanded block,
                    // ensure it's content is check for possible expansions again
                    has_changed |= self.expand_node_list(&mut inner.content)?;
                }

                ast::Node::Label(_) | ast::Node::Empty => continue,
            }
        }

        Ok(has_changed)
    }

    fn eval_expression(
        &mut self,
        expr: ast::Expression,
    ) -> Result<ast::Expression, ExpansionError> {
        match expr {
            ast::Expression::Unary { op, op_span, expr } => {
                let expr = self.eval_expression(*expr)?;

                // ensure we are handling a number type here
                let ast::Expression::Number { value, span } = expr else {
                    return Err(ExpansionError::InvalidArithType {
                        op_span,
                        value_span: expr.span(),
                    });
                };

                let span = op_span.join(span);
                let value = match op {
                    ast::UnaryOp::Negate => (-(value as i32)) as u32,
                };

                Ok(ast::Expression::Number { value, span })
            }
            ast::Expression::Binary {
                op,
                op_span,
                left,
                right,
            } => {
                let left_expr = self.eval_expression(*left)?;
                let right_expr = self.eval_expression(*right)?;

                // ensure we are handling a number type here
                let ast::Expression::Number {
                    value: left_value,
                    span: left_span,
                } = left_expr
                else {
                    return Err(ExpansionError::InvalidArithType {
                        op_span,
                        value_span: left_expr.span(),
                    });
                };

                let ast::Expression::Number {
                    value: right_value,
                    span: right_span,
                } = right_expr
                else {
                    return Err(ExpansionError::InvalidArithType {
                        op_span,
                        value_span: right_expr.span(),
                    });
                };

                let span = left_span.join(right_span);
                let value = match op {
                    ast::BinaryOp::Add => left_value.wrapping_add(right_value),
                    ast::BinaryOp::Subtract => left_value.wrapping_sub(right_value),
                    ast::BinaryOp::Multiply => left_value.wrapping_mul(right_value),
                    ast::BinaryOp::Divide => left_value.wrapping_div(right_value),
                    ast::BinaryOp::And => left_value & right_value,
                    ast::BinaryOp::Or => left_value | right_value,
                    ast::BinaryOp::Xor => left_value ^ right_value,
                    ast::BinaryOp::ShiftLeft => left_value >> right_value,
                    ast::BinaryOp::ShiftRight => left_value << right_value,
                };

                Ok(ast::Expression::Number { value, span })
            }
            ast::Expression::Identifier { name, span } => {
                let Some(value) = self.resolve_identifier(name) else {
                    return Err(ExpansionError::UnknownSymbol { name, span });
                };

                Ok(value)
            }
            ast::Expression::Number { .. } => Ok(expr),
            ast::Expression::Register { .. } => Ok(expr),
        }
    }

    fn resolve_identifier(&self, name: StringId) -> Option<ast::Expression> {
        todo!()
    }
}
