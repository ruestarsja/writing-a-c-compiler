use super::super::aast::{
    BinaryOperator,
    Instruction,
    Operand,
    Register
};


pub(super) fn traverse(
    aast_instruction: Instruction
) -> Vec<Instruction> {
    match aast_instruction {
        Instruction::Binary {
            operator: aast_binary_operator,
            src: aast_src,
            dst: aast_dst
        } => {

            match aast_binary_operator {

                BinaryOperator::Add |
                BinaryOperator::Sub |
                BinaryOperator::BitwiseAnd |
                BinaryOperator::BitwiseOr |
                BinaryOperator::BitwiseXor => {
                    match (&aast_src, &aast_dst) {
                        (Operand::Stack { offset: _ }, Operand::Stack { offset: _ }) => {
                            let temp_reg: Operand = Operand::Reg {
                                reg: Register::R10
                            };
                            vec![
                                Instruction::Mov {
                                    src: aast_src,
                                    dst: temp_reg.clone()
                                },
                                Instruction::Binary {
                                    operator: aast_binary_operator,
                                    src: temp_reg,
                                    dst: aast_dst
                                }
                            ]
                        },
                        _ => vec![
                            Instruction::Binary {
                                operator: aast_binary_operator,
                                src: aast_src,
                                dst: aast_dst
                            }
                        ]
                    }
                },

                BinaryOperator::Mult => {
                    match aast_dst {
                        Operand::Stack { offset: _ } => {
                            let temp_reg: Operand = Operand::Reg {
                                reg: Register::R11
                            };
                            vec![
                                Instruction::Mov {
                                    src: aast_dst.clone(),
                                    dst: temp_reg.clone()
                                },
                                Instruction::Binary {
                                    operator: aast_binary_operator,
                                    src: aast_src,
                                    dst: temp_reg.clone()
                                },
                                Instruction::Mov {
                                    src: temp_reg,
                                    dst: aast_dst
                                }
                            ]
                        },
                        _ => vec![
                            Instruction::Binary {
                                operator: aast_binary_operator,
                                src: aast_src,
                                dst: aast_dst
                            }
                        ]
                    }
                },

                BinaryOperator::BitwiseLeftShift |
                BinaryOperator::BitwiseRightShift => {
                    match aast_src {
                        Operand::Reg { reg: _ } |
                        Operand::Pseudo { name: _ } |
                        Operand::Stack { offset: _ } => {
                            vec![
                                Instruction::Mov {
                                    src: aast_src,
                                    dst: Operand::Reg {
                                        reg: Register::CX
                                    }
                                },
                                Instruction::Binary {
                                    operator: aast_binary_operator,
                                    src: Operand::Reg {
                                        reg: Register::CL
                                    },
                                    dst: aast_dst
                                }
                            ]
                        },
                        Operand::Imm { int: _ } => vec![
                            Instruction::Binary {
                                operator: aast_binary_operator,
                                src: aast_src,
                                dst: aast_dst
                            }
                        ]
                    }
                }

            }

        },
        _ => panic!("internal error: called binary::traverse() with non-binary instruction")
    }
}