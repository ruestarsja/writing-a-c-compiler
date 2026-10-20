use super::cast::Statement;

use super::block;
use super::id;


pub(super) fn traverse(
    statement: Statement,
    current_loop_id: Option<String>
) -> Statement {
    match statement {

        Statement::Return {
            labels,
            exp
        } => {
            Statement::Return {
                labels,
                exp
            }
        },

        Statement::Expression {
            labels,
            exp
        } => {
            Statement::Expression {
                labels,
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
                labels,
                condition,
                then_statement: Box::new(
                    traverse(
                        *then_statement,
                        current_loop_id.clone()
                    )
                ),
                else_statement: match else_statement {
                    Some(state) => Some(
                        Box::new(
                            traverse(
                                *state,
                                current_loop_id
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
                    current_loop_id
                )
            }
        },

        Statement::Break {
            labels,
            loop_id: _
        } => {
            if current_loop_id.is_none() {
                panic!("semantic error: break outside of loop body");
            }
            Statement::Break {
                labels,
                loop_id: current_loop_id
            }
        },

        Statement::Continue {
            labels,
            loop_id: _
        } => {
            if current_loop_id.is_none() {
                panic!("semantic error: continue outside of loop body");
            }
            Statement::Continue {
                labels,
                loop_id: current_loop_id
            }
        },

        Statement::While {
            labels,
            condition,
            body,
            loop_id: _
        } => {
            let new_loop_id: Option<String> = Some(
                id::generate_loop_id(id::LoopType::While)
            );
            Statement::While {
                labels,
                condition,
                body: Box::new(
                    traverse(
                        *body,
                        new_loop_id.clone()
                    )
                ),
                loop_id: new_loop_id
            }
        },

        Statement::DoWhile {
            labels,
            body,
            condition,
            loop_id: _
        } => {
            let new_loop_id: Option<String> = Some(
                id::generate_loop_id(id::LoopType::DoWhile)
            );
            Statement::DoWhile {
                labels,
                body: Box::new(
                    traverse(
                        *body,
                        new_loop_id.clone()
                    )
                ),
                condition,
                loop_id: new_loop_id
            }
        },

        Statement::For {
            labels,
            init,
            condition,
            post,
            body,
            loop_id: _
        } => {
            let new_loop_id: Option<String> = Some(
                id::generate_loop_id(id::LoopType::For)
            );
            Statement::For {
                labels,
                init,
                condition,
                post,
                body: Box::new(
                    traverse(
                        *body,
                        new_loop_id.clone()
                    )
                ),
                loop_id: new_loop_id
            }
        },

        Statement::Null {
            labels
        } => {
            Statement::Null {
                labels
            }
        }

    }
}