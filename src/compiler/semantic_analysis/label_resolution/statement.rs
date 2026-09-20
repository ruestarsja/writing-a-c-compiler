use super::cast::Statement;
use super::LabelMap;

use super::statement_label_list;
use super::target_label;


pub(super) fn traverse(
    statement: Statement,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Statement {
    match statement {

        Statement::Return {
            labels,
            exp
        } => {
            Statement::Return {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                exp
            }
        },

        Statement::Expression {
            labels,
            exp
        } => {
            Statement::Expression {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                exp
            }
        },

        Statement::If {
            labels,
            condition,
            then_statement,
            else_statement
        } => {
            Statement::If {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                condition,
                then_statement: Box::new(
                    traverse(
                        *then_statement,
                        label_map,
                        undefined_label_map
                    )
                ),
                else_statement: match else_statement {
                    Some(else_box) => {
                        Some(
                            Box::new(
                                traverse(
                                    *else_box,
                                    label_map,
                                    undefined_label_map
                                )
                            )
                        )
                    },
                    None => None
                }
            }
        },

        Statement::Goto {
            labels,
            target
        } => {
            Statement::Goto {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                target: target_label::traverse(
                    target,
                    label_map,
                    undefined_label_map
                )
            }
        },

        Statement::Null {
            labels
        } => {
            Statement::Null {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                )
            }
        }

    }
}