use super::cast;
use super::tast;

use super::block;
use super::for_init;
use super::label_list;
use super::identifier;
use super::expression;
use super::id;


pub(super) fn generate_instructions(
    cast_statement: cast::Statement
) -> Vec<tast::Instruction> {
    let mut instructions: Vec<tast::Instruction>;
    let mut statement_instructions: Vec<tast::Instruction>;

    match cast_statement {

        cast::Statement::Return {
            labels: cast_labels,
            exp: cast_exp
        } => {
            let val: tast::Value;
            instructions = label_list::generate_instructions(cast_labels);
            (statement_instructions, val) = expression::generate_instructions_and_value(cast_exp);
            instructions.append(&mut statement_instructions);
            instructions.push(
                tast::Instruction::Return {
                    val
                }
            );
        },

        cast::Statement::Expression {
            labels: cast_labels,
            exp: cast_exp
        } => {
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = expression::generate_instructions(cast_exp);
            instructions.append(
                &mut statement_instructions
            );
        },

        cast::Statement::If {
            labels: cast_labels,
            condition: cast_condition,
            then_statement: cast_then,
            else_statement: cast_else
        } => {
            let else_label: String = id::generate(id::IDType::ElseLabel);
            let end_label: String = id::generate(id::IDType::EndIfLabel);
            let val: tast::Value;
            instructions = label_list::generate_instructions(cast_labels);
            (statement_instructions, val) = expression::generate_instructions_and_value(cast_condition);
            instructions.append(
                &mut statement_instructions
            );
            instructions.push(
                tast::Instruction::JumpIfZero {
                    condition: val,
                    target: tast::Identifier {
                        id: match cast_else {
                            Some(_) => else_label.clone(),
                            None => end_label.clone()
                        }
                    }
                }
            );
            let mut then_instructions: Vec<tast::Instruction> = generate_instructions(*cast_then);
            instructions.append(&mut then_instructions);
            match cast_else {
                Some(cast_else) => {
                    instructions.push(
                        tast::Instruction::Jump {
                            target: tast::Identifier {
                                id: end_label.clone()
                            }
                        }
                    );
                    instructions.push(
                        tast::Instruction::Label {
                            id: tast::Identifier {
                                id: else_label
                            }
                        }
                    );
                    let mut else_instructions: Vec<tast::Instruction> = generate_instructions(*cast_else);
                    instructions.append(&mut else_instructions);
                },
                None => {}
            }
            instructions.push(
                tast::Instruction::Label {
                    id: tast::Identifier {
                        id: end_label
                    }
                }
            );
        },

        cast::Statement::Goto {
            labels: cast_labels,
            target: cast_target
        } => {
            instructions = label_list::generate_instructions(cast_labels);
            instructions.push(
                tast::Instruction::Jump {
                    target: identifier::generate(cast_target.id)
                }
            );
        },

        cast::Statement::Compound {
            labels: cast_labels,
            block: cast_block
        } => {
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = block::generate_instructions(cast_block);
            instructions.append(
                &mut statement_instructions
            );
        },

        cast::Statement::Break {
            labels: cast_labels,
            loop_id: cast_loop_id
        } => {
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = vec![
                tast::Instruction::Jump {
                    target: tast::Identifier {
                        id: id::generate_loop_id(
                            id::LoopLabelIDType::BreakLabel,
                            cast_loop_id.expect("internal error: break statement has null loop id during tacky generation")
                        )
                    }
                }
            ];
            instructions.append(
                &mut statement_instructions
            );
        },

        cast::Statement::Continue {
            labels: cast_labels,
            loop_id: cast_loop_id
        } => {
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = vec![
                tast::Instruction::Jump {
                    target: tast::Identifier {
                        id: id::generate_loop_id(
                            id::LoopLabelIDType::ContinueLabel,
                            cast_loop_id.expect("internal error: continue statement has null loop id during tacky generation")
                        )
                    }
                }
            ];
            instructions.append(
                &mut statement_instructions
            );
        },

        cast::Statement::While {
            labels: cast_labels,
            condition: cast_condition,
            body: cast_body,
            loop_id: cast_loop_id
        } => {
            let loop_id: String = cast_loop_id.expect("internal error: while statement has null loop id during tacky generation");
            let continue_label_id: tast::Identifier = tast::Identifier {
                id: id::generate_loop_id(
                    id::LoopLabelIDType::ContinueLabel,
                    loop_id.clone()
                )
            };
            let break_label_id: tast::Identifier = tast::Identifier {
                id: id::generate_loop_id(
                    id::LoopLabelIDType::BreakLabel,
                    loop_id
                )
            };
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = vec![
                tast::Instruction::Label {
                    id: continue_label_id.clone()
                }
            ];
            let cond: tast::Value;
            let mut cond_instructions: Vec<tast::Instruction>;
            (cond_instructions, cond) = expression::generate_instructions_and_value(cast_condition);
            statement_instructions.append(
                &mut cond_instructions
            );
            statement_instructions.push(
                tast::Instruction::JumpIfZero {
                    condition: cond,
                    target: break_label_id.clone()
                }
            );
            statement_instructions.append(
                &mut generate_instructions(*cast_body)
            );
            statement_instructions.push(
                tast::Instruction::Jump {
                    target: continue_label_id
                }
            );
            statement_instructions.push(
                tast::Instruction::Label {
                    id: break_label_id
                }
            );
            instructions.append(
                &mut statement_instructions
            );
        },

        cast::Statement::DoWhile {
            labels: cast_labels,
            body: cast_body,
            condition: cast_condition,
            loop_id: cast_loop_id
        } => {
            let loop_id: String = cast_loop_id.expect("internal error: do-while statement has null loop id during tacky generation");
            let start_label_id: tast::Identifier = tast::Identifier {
                id: id::generate_loop_id(
                    id::LoopLabelIDType::StartLabel,
                    loop_id.clone()
                )
            };
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = vec![
                tast::Instruction::Label {
                    id: start_label_id.clone()
                }
            ];
            statement_instructions.append(
                &mut generate_instructions(*cast_body)
            );
            statement_instructions.push(
                tast::Instruction::Label {
                    id: tast::Identifier {
                        id: id::generate_loop_id(
                            id::LoopLabelIDType::ContinueLabel,
                            loop_id.clone()
                        )
                    }
                }
            );
            let cond: tast::Value;
            let mut cond_instructions: Vec<tast::Instruction>;
            (cond_instructions, cond) = expression::generate_instructions_and_value(cast_condition);
            statement_instructions.append(
                &mut cond_instructions
            );
            statement_instructions.push(
                tast::Instruction::JumpIfNotZero {
                    condition: cond,
                    target: start_label_id
                }
            );
            statement_instructions.push(
                tast::Instruction::Label {
                    id: tast::Identifier {
                        id: id::generate_loop_id(
                            id::LoopLabelIDType::BreakLabel,
                            loop_id
                        )
                    }
                }
            );
            instructions.append(
                &mut statement_instructions
            );
        },

        cast::Statement::For {
            labels: cast_labels,
            init: cast_init,
            condition: cast_condition,
            post: cast_post,
            body: cast_body,
            loop_id: cast_loop_id
        } => {
            let loop_id: String = cast_loop_id.expect("internal error: for statement has null loop id during tacky generation");
            let start_label_id: tast::Identifier = tast::Identifier {
                id: id::generate_loop_id(
                    id::LoopLabelIDType::StartLabel,
                    loop_id.clone()
                )
            };
            let break_label_id: tast::Identifier = tast::Identifier {
                id: id::generate_loop_id(
                    id::LoopLabelIDType::BreakLabel,
                    loop_id.clone()
                )
            };
            instructions = label_list::generate_instructions(cast_labels);
            statement_instructions = for_init::generate_instructions(cast_init);
            statement_instructions.push(
                tast::Instruction::Label {
                    id: start_label_id.clone()
                }
            );
            match cast_condition {
                Some(exp) => {
                    let cond: tast::Value;
                    let mut cond_instructions: Vec<tast::Instruction>;
                    (cond_instructions, cond) = expression::generate_instructions_and_value(exp);
                    statement_instructions.append(
                        &mut cond_instructions
                    );
                    statement_instructions.push(
                        tast::Instruction::JumpIfZero {
                            condition: cond,
                            target: break_label_id.clone()
                        }
                    );
                },
                None => {}
            }
            statement_instructions.append(
                &mut generate_instructions(*cast_body)
            );
            statement_instructions.push(
                tast::Instruction::Label {
                    id: tast::Identifier {
                        id: id::generate_loop_id(
                            id::LoopLabelIDType::ContinueLabel,
                            loop_id
                        )
                    }
                }
            );
            match cast_post {
                Some(exp) => statement_instructions.append(
                    &mut expression::generate_instructions(exp)
                ),
                None => {}
            }
            statement_instructions.push(
                tast::Instruction::Jump {
                    target: start_label_id
                }
            );
            statement_instructions.push(
                tast::Instruction::Label {
                    id: break_label_id
                }
            );
            instructions.append(
                &mut statement_instructions
            );
        }

        cast::Statement::Null {
            labels: cast_labels,
        } => {
            instructions = label_list::generate_instructions(cast_labels);
        }

    }

    instructions
}
