use super::tast;
use super::aast;

use super::super::operand;
use super::super::operators;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
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
                    let dst: aast::Operand = operand::generate(tast_destination);
                    vec![

                        aast::Instruction::Mov {
                            src: operand::generate(tast_left_source),
                            dst: dst.clone()
                        },

                        aast::Instruction::Binary {
                            operator: operators::binary::generate(tast_binary_operator),
                            src: operand::generate(tast_right_source),
                            dst: dst
                        }

                    ]
                },

                tast::BinaryOperator::Divide => {
                    vec![

                        aast::Instruction::Mov {
                            src: operand::generate(tast_left_source),
                            dst: aast::Operand::Reg {
                                reg: aast::Register::AX
                            }
                        },

                        aast::Instruction::Cdq,

                        aast::Instruction::Idiv {
                            operand: operand::generate(tast_right_source)
                        },

                        aast::Instruction::Mov {
                            src: aast::Operand::Reg {
                                reg: aast::Register::AX
                            },
                            dst: operand::generate(tast_destination)
                        }

                    ]
                },

                tast::BinaryOperator::Remainder =>  {
                    vec![

                        aast::Instruction::Mov {
                            src: operand::generate(tast_left_source),
                            dst: aast::Operand::Reg {
                                reg: aast::Register::AX
                            }
                        },

                        aast::Instruction::Cdq,

                        aast::Instruction::Idiv {
                            operand: operand::generate(tast_right_source)
                        },

                        aast::Instruction::Mov {
                            src: aast::Operand::Reg {
                                reg: aast::Register::DX
                            },
                            dst: operand::generate(tast_destination)
                        }

                    ]
                },

                tast::BinaryOperator::Equal |
                tast::BinaryOperator::NotEqual |
                tast::BinaryOperator::LessThan |
                tast::BinaryOperator::LessThanOrEqual |
                tast::BinaryOperator::GreaterThan |
                tast::BinaryOperator::GreaterThanOrEqual => {
                    let dst: aast::Operand = operand::generate(tast_destination);
                    vec![

                        aast::Instruction::Cmp {
                            loperand: operand::generate(tast_right_source),
                            roperand: operand::generate(tast_left_source)
                        },

                        aast::Instruction::Mov {
                            src: aast::Operand::Imm {
                                int: 0
                            },
                            dst: dst.clone()
                        },

                        aast::Instruction::SetCC {
                            condition: operators::binary::generate_condition_code(tast_binary_operator),
                            operand: dst
                        }

                    ]
                }

            }
        },
        _ => panic!("internal error: called `binary::generate()` with non-binary instruction")
    }
}