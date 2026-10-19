use super::cast;
use super::tast;

use super::block;
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

        cast::Statement::Null {
            labels: cast_labels,
        } => {
            instructions = label_list::generate_instructions(cast_labels);
        }

    }

    instructions
}
