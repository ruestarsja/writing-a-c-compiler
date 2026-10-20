use super::cast::Statement;
use super::VariableMap;

use super::expression;
use super::block;
use super::for_init;
use super::variable_map;


pub(super) fn traverse(
    statement: Statement,
    variable_map: &mut VariableMap
) -> Statement {
    match statement {

        Statement::Return {
            labels,
            exp
        } => {
            Statement::Return {
                labels,
                exp: expression::traverse(
                    exp,
                    variable_map
                )
            }
        },

        Statement::Expression {
            labels,
            exp
        } => {
            Statement::Expression {
                labels,
                exp: expression::traverse(
                    exp,
                    variable_map
                )
            }
        },

        Statement::If {
            labels,
            condition,
            then_statement,
            else_statement
        } => {
            Statement::If {
                labels,
                condition: expression::traverse(
                    condition,
                    variable_map
                ),
                then_statement: Box::new(
                    traverse(
                        *then_statement,
                        variable_map
                    )
                ),
                else_statement: match else_statement {
                    Some(statement) => Some(
                        Box::new(
                            traverse(
                                *statement,
                                variable_map
                            )
                        )
                    ),
                    None => None
                }
            }
        },

        Statement::Goto {
            labels,
            target
        } => {
            Statement::Goto {
                labels,
                target
            }
        },

        Statement::Compound {
            labels,
            block
        } => {
            Statement::Compound {
                labels,
                block: block::traverse(
                    block,
                    variable_map
                )
            }
        },

        Statement::Break {
            labels,
            loop_id
        } => {
            Statement::Break {
                labels,
                loop_id
            }
        },

        Statement::Continue {
            labels,
            loop_id
        } => {
            Statement::Continue {
                labels,
                loop_id
            }
        },

        Statement::While {
            labels,
            condition,
            body,
            loop_id
        } => {
            Statement::While {
                labels,
                condition: expression::traverse(
                    condition,
                    variable_map
                ),
                body: Box::new(
                    traverse(
                        *body,
                        variable_map
                    )
                ),
                loop_id
            }
        },

        Statement::DoWhile {
            labels,
            body,
            condition,
            loop_id
        } => {
            Statement::DoWhile {
                labels,
                body: Box::new(
                    traverse(
                        *body,
                        variable_map
                    )
                ),
                condition: expression::traverse(
                    condition,
                    variable_map
                ),
                loop_id
            }
        },

        Statement::For {
            labels,
            init,
            condition,
            post,
            body,
            loop_id
        } => {
            let mut inner_variable_map: VariableMap = variable_map::copy_for_new_block(variable_map);
            Statement::For {
                labels,
                init: for_init::traverse(
                    init,
                    &mut inner_variable_map
                ),
                condition: match condition {
                    Some(exp) => Some(
                        expression::traverse(
                            exp,
                            &mut inner_variable_map
                        )
                    ),
                    None => None
                },
                post: match post {
                    Some(exp) => Some(
                        expression::traverse(
                            exp,
                            &mut inner_variable_map
                        )
                    ),
                    None => None
                },
                body: Box::new(
                    traverse(
                        *body,
                        &mut inner_variable_map
                    )
                ),
                loop_id
            }
        }
        
        Statement::Null {
            labels
        } => {
            Statement::Null {
                labels
            }
        }
        
    }
}