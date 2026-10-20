use super::cast::Statement;
use super::LabelMap;

use super::block;
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

        Statement::Compound {
            labels,
            block
        } => {
            Statement::Compound {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                block: block::traverse(
                    block,
                    label_map,
                    undefined_label_map
                )
            }
        },

        Statement::Break {
            labels,
            loop_id
        } => {
            Statement::Break {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                loop_id
            }
        },

        Statement::Continue {
            labels,
            loop_id
        } => {
            Statement::Continue {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
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
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                condition,
                body: Box::new(
                    traverse(
                        *body,
                        label_map,
                        undefined_label_map
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
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                body: Box::new(
                    traverse(
                        *body,
                        label_map,
                        undefined_label_map
                    )
                ),
                condition,
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
            Statement::For {
                labels: statement_label_list::traverse(
                    labels,
                    label_map,
                    undefined_label_map
                ),
                init,
                condition,
                post,
                body: Box::new(
                    traverse(
                        *body,
                        label_map,
                        undefined_label_map
                    )
                ),
                loop_id
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