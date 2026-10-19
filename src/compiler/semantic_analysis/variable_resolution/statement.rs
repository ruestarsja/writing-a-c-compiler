use super::cast::Statement;
use super::VariableMap;

use super::expression;
use super::block;


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