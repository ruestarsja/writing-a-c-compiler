use crate::compiler::aast::Operand;

use super::super::obj::{tast,aast};

// FUNCTIONS

pub(super) fn main(tacky_ast: tast::Program) -> aast::Program {
    generate_program(tacky_ast)
}

fn generate_program(tast_program: tast::Program) -> aast::Program {
    aast::Program {
        function_definition: generate_function(tast_program.function_definition)
    }
}

fn generate_function(tast_function_definition: tast::Function) -> aast::Function {
    let mut instructions: Vec<aast::Instruction> = vec![];
    for tast_instruction in tast_function_definition.body {
        instructions.append(
            &mut generate_instructions(tast_instruction)
        );
    }
    aast::Function {
        name: generate_identifier(tast_function_definition.name),
        instructions
    }
}

fn generate_identifier(tast_identifier: tast::Identifier) -> aast::Identifier {
    aast::Identifier {
        id: tast_identifier.id
    }
}

// fn generate_instructions(ast_statement: tast::Statement) -> Vec<aast::Instruction> {
//     match ast_statement {
//         tast::Statement::Return {
//             exp: ast_exp
//         } => {
//             vec![
//                 aast::Instruction::Mov {
//                     exp: generate_expression(ast_exp),
//                     reg: aast::Register {}
//                 },
//                 aast::Instruction::Ret
//             ]
//         }
//     }
// }

fn generate_instructions(tast_instruction: tast::Instruction) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Return {
            val: tast_value
        } => {
            vec![
                aast::Instruction::Mov {
                    src: generate_operand_from_val(tast_value),
                    dst: aast::Operand::Reg {
                        reg: aast::Register::AX
                    }
                },
                aast::Instruction::Ret
            ]
        },
        tast::Instruction::Unary {
            operator: tast_unary_operator,
            src: tast_source,
            dst: tast_destination
        } => {
            match tast_unary_operator {
                tast::UnaryOperator::Complement |
                tast::UnaryOperator::Negate => {
                    let dst: aast::Operand = generate_operand_from_val(tast_destination);
                    vec![
                        aast::Instruction::Mov {
                            src: generate_operand_from_val(tast_source),
                            dst: dst.clone()
                        },
                        aast::Instruction::Unary {
                            operator: generate_unary_operator(tast_unary_operator),
                            operand: dst
                        }
                    ]
                },
                tast::UnaryOperator::Not => {
                    let zero: aast::Operand = aast::Operand::Imm { int: 0 };
                    let dst: aast::Operand = generate_operand_from_val(tast_destination);
                    vec![
                        aast::Instruction::Cmp {
                            loperand: zero.clone(),
                            roperand: generate_operand_from_val(tast_source)
                        },
                        aast::Instruction::Mov {
                            src: zero,
                            dst: dst.clone()
                        },
                        aast::Instruction::SetCC {
                            condition: aast::ConditionCode::E,
                            operand: dst
                        }
                    ]
                }
            }
        },
        tast::Instruction::Binary {
            operator: tast_binary_operator,
            lsrc: tast_left_source,
            rsrc: tast_right_source,
            dst: tast_destination
        } => {
            match tast_binary_operator {
                tast::BinaryOperator::Add |
                tast::BinaryOperator::Subtract |
                tast::BinaryOperator::Multiply |
                tast::BinaryOperator::BitwiseAnd |
                tast::BinaryOperator::BitwiseOr |
                tast::BinaryOperator::BitwiseXor |
                tast::BinaryOperator::BitwiseLeftShift |
                tast::BinaryOperator::BitwiseRightShift => {
                    let dst: aast::Operand = generate_operand_from_val(tast_destination);
                    vec![
                        aast::Instruction::Mov {
                            src: generate_operand_from_val(tast_left_source),
                            dst: dst.clone()
                        },
                        aast::Instruction::Binary {
                            operator: generate_binary_operator(tast_binary_operator),
                            src: generate_operand_from_val(tast_right_source),
                            dst: dst
                        }
                    ]
                },
                tast::BinaryOperator::Divide => {
                    vec![
                        aast::Instruction::Mov {
                            src: generate_operand_from_val(tast_left_source),
                            dst: aast::Operand::Reg {
                                reg: aast::Register::AX
                            }
                        },
                        aast::Instruction::Cdq,
                        aast::Instruction::Idiv {
                            operand: generate_operand_from_val(tast_right_source)
                        },
                        aast::Instruction::Mov {
                            src: aast::Operand::Reg {
                                reg: aast::Register::AX
                            },
                            dst: generate_operand_from_val(tast_destination)
                        }
                    ]
                },
                tast::BinaryOperator::Remainder => {
                    vec![
                        aast::Instruction::Mov {
                            src: generate_operand_from_val(tast_left_source),
                            dst: aast::Operand::Reg {
                                reg: aast::Register::AX
                            }
                        },
                        aast::Instruction::Cdq,
                        aast::Instruction::Idiv {
                            operand: generate_operand_from_val(tast_right_source)
                        },
                        aast::Instruction::Mov {
                            src: aast::Operand::Reg {
                                reg: aast::Register::DX
                            },
                            dst: generate_operand_from_val(tast_destination)
                        }
                    ]
                },
                tast::BinaryOperator::Equal |
                tast::BinaryOperator::NotEqual |
                tast::BinaryOperator::LessThan |
                tast::BinaryOperator::LessThanOrEqual |
                tast::BinaryOperator::GreaterThan |
                tast::BinaryOperator::GreaterThanOrEqual => {
                    let aast_destination: Operand = generate_operand_from_val(tast_destination);
                    vec![
                        aast::Instruction::Cmp {
                            loperand: generate_operand_from_val(tast_right_source),
                            roperand: generate_operand_from_val(tast_left_source)
                        },
                        aast::Instruction::Mov {
                            src: aast::Operand::Imm {
                                int: 0
                            },
                            dst: aast_destination.clone()
                        },
                        aast::Instruction::SetCC {
                            condition: generate_condition_code_from_operator(tast_binary_operator),
                            operand: aast_destination
                        }
                    ]
                }
            }
        },
        tast::Instruction::Copy {
            src: tast_source,
            dst: tast_destination
        } => {
            vec![
                aast::Instruction::Mov {
                    src: generate_operand_from_val(tast_source),
                    dst: generate_operand_from_val(tast_destination)
                }
            ]
        },
        tast::Instruction::Jump {
            target: tast_target
        } => {
            vec![
                aast::Instruction::Jmp {
                    target: generate_identifier(tast_target)
                }
            ]
        },
        tast::Instruction::JumpIfZero {
            condition: tast_condition,
            target: tast_target
        } => {
            vec![
                aast::Instruction::Cmp {
                    loperand: aast::Operand::Imm {
                        int: 0
                    },
                    roperand: generate_operand_from_val(tast_condition)
                },
                aast::Instruction::JmpCC {
                    condition: aast::ConditionCode::E,
                    target: generate_identifier(tast_target)
                }
            ]
        },
        tast::Instruction::JumpIfNotZero {
            condition: tast_condition,
            target: tast_target
        } => {
            vec![
                aast::Instruction::Cmp {
                    loperand: aast::Operand::Imm {
                        int: 0
                    },
                    roperand: generate_operand_from_val(tast_condition)
                },
                aast::Instruction::JmpCC {
                    condition: aast::ConditionCode::NE,
                    target: generate_identifier(tast_target)
                }
            ]
        },
        tast::Instruction::Label {
            id: tast_id
        } => {
            vec![
                aast::Instruction::Label {
                    id: generate_identifier(tast_id)
                }
            ]
        }
    }
}

fn generate_operand_from_val(tast_value: tast::Value) -> aast::Operand {
    match tast_value {
        tast::Value::Constant {
            int: tast_int
        } => {
            aast::Operand::Imm {
                int: tast_int
            }
        },
        tast::Value::Variable {
            name: tast_name
        } => {
            aast::Operand::Pseudo {
                name: generate_identifier(tast_name)
            }
        }
    }
}

fn generate_unary_operator(tast_unary_operator: tast::UnaryOperator) -> aast::UnaryOperator {
    match tast_unary_operator {
        tast::UnaryOperator::Complement => {
            aast::UnaryOperator::Not
        },
        tast::UnaryOperator::Negate => {
            aast::UnaryOperator::Neg
        },
        tast::UnaryOperator::Not => {
            panic!("internal error: no aast equivalent for not operator (use cmp, mov, setcc instead)");
        }
    }
}

fn generate_binary_operator(tast_binary_operator: tast::BinaryOperator) -> aast::BinaryOperator {
    match tast_binary_operator {
        tast::BinaryOperator::Add => {
            aast::BinaryOperator::Add
        },
        tast::BinaryOperator::Subtract => {
            aast::BinaryOperator::Sub
        },
        tast::BinaryOperator::Multiply => {
            aast::BinaryOperator::Mult
        },
        tast::BinaryOperator::BitwiseAnd => {
            aast::BinaryOperator::BitwiseAnd
        },
        tast::BinaryOperator::BitwiseOr => {
            aast::BinaryOperator::BitwiseOr
        },
        tast::BinaryOperator::BitwiseXor => {
            aast::BinaryOperator::BitwiseXor
        },
        tast::BinaryOperator::BitwiseLeftShift => {
            aast::BinaryOperator::BitwiseLeftShift
        },
        tast::BinaryOperator::BitwiseRightShift => {
            aast::BinaryOperator::BitwiseRightShift
        }
        tast::BinaryOperator::Divide |
        tast::BinaryOperator::Remainder => {
            panic!("internal error: no aast equivalent for division or remainder operators (use idiv instruction instead)");
        },
        _ => { todo!("not yet implemented: generate assembly for missing binary operators") }
    }
}

fn generate_condition_code_from_operator(tast_binary_operator: tast::BinaryOperator) -> aast::ConditionCode {
    match tast_binary_operator {
        tast::BinaryOperator::Add |
        tast::BinaryOperator::Subtract |
        tast::BinaryOperator::Multiply |
        tast::BinaryOperator::Divide |
        tast::BinaryOperator::Remainder |
        tast::BinaryOperator::BitwiseAnd |
        tast::BinaryOperator::BitwiseOr |
        tast::BinaryOperator::BitwiseXor |
        tast::BinaryOperator::BitwiseLeftShift |
        tast::BinaryOperator::BitwiseRightShift => {
            panic!("internal error: no condition code for most binary operators (use only for comparative operations)");
        },
        tast::BinaryOperator::Equal => {
            aast::ConditionCode::E
        },
        tast::BinaryOperator::NotEqual => {
            aast::ConditionCode::NE
        },
        tast::BinaryOperator::LessThan => {
            aast::ConditionCode::L
        },
        tast::BinaryOperator::LessThanOrEqual => {
            aast::ConditionCode::LE
        },
        tast::BinaryOperator::GreaterThan => {
            aast::ConditionCode::G
        },
        tast::BinaryOperator::GreaterThanOrEqual => {
            aast::ConditionCode::GE
        }
    }
}
