use super::super::obj::aast;

// FUNCTIONS

pub(super) fn main(
    assembly_ast: aast::Program,
    allocation_size: u32
) -> aast::Program {
    traverse_program(assembly_ast, allocation_size)
}

fn traverse_program(
    aast_program: aast::Program,
    allocation_size: u32
) -> aast::Program {
    aast::Program {
        function_definition: traverse_function(aast_program.function_definition, allocation_size)
    }
}

fn traverse_function(
    aast_function: aast::Function,
    allocation_size: u32
) -> aast::Function {
    let mut instructions: Vec<aast::Instruction> = vec![
        aast::Instruction::AllocateStack { bytes: allocation_size }
    ];
    for aast_instruction in aast_function.instructions {
        instructions.append(
            &mut traverse_instruction(aast_instruction)
        );
    }
    aast::Function {
        name: aast_function.name,
        instructions
    }
}

fn traverse_instruction(aast_instruction: aast::Instruction) -> Vec<aast::Instruction> {
    match &aast_instruction {
        aast::Instruction::Mov {
            src: aast_source,
            dst: aast_destination
        } => {
            match (&aast_source, &aast_destination) {
                (
                    aast::Operand::Stack { offset: _ },
                    aast::Operand::Stack { offset: _ }
                ) => {
                    let tmp_reg_op: aast::Operand = aast::Operand::Reg {
                        reg: aast::Register::R10
                    };
                    vec![
                        aast::Instruction::Mov {
                            src: aast_source.to_owned(),
                            dst: tmp_reg_op.clone()
                        },
                        aast::Instruction::Mov {
                            src: tmp_reg_op,
                            dst: aast_destination.to_owned()
                        }
                    ]
                },
                _ => {
                    vec![ aast_instruction ]
                }
            }
        },
        aast::Instruction::Binary {
            operator: aast_binary_operator,
            src: aast_source,
            dst: aast_destination
        } => {
            match &aast_binary_operator {
                aast::BinaryOperator::Add |
                aast::BinaryOperator::Sub |
                aast::BinaryOperator::BitwiseAnd |
                aast::BinaryOperator::BitwiseOr |
                aast::BinaryOperator::BitwiseXor => {
                    match (&aast_source, &aast_destination) {
                        (
                            aast::Operand::Stack { offset: _ },
                            aast::Operand::Stack { offset: _ }
                        ) => {
                            let tmp_reg_op: aast::Operand = aast::Operand::Reg {
                                reg: aast::Register::R10
                            };
                            vec![
                                aast::Instruction::Mov {
                                    src: aast_source.to_owned(),
                                    dst: tmp_reg_op.clone()
                                },
                                aast::Instruction::Binary {
                                    operator: aast_binary_operator.to_owned(),
                                    src: tmp_reg_op,
                                    dst: aast_destination.to_owned()
                                }
                            ]
                        },
                        _ => {
                            vec![ aast_instruction ]
                        }
                    }
                },
                aast::BinaryOperator::Mult => {
                    match &aast_destination {
                        aast::Operand::Stack { offset: _ } => {
                            let tmp_reg_op: aast::Operand = aast::Operand::Reg {
                                reg: aast::Register::R11
                            };
                            vec![
                                aast::Instruction::Mov {
                                    src: aast_destination.to_owned(),
                                    dst: tmp_reg_op.clone()
                                },
                                aast::Instruction::Binary {
                                    operator: aast::BinaryOperator::Mult,
                                    src: aast_source.to_owned(),
                                    dst: tmp_reg_op.clone()
                                },
                                aast::Instruction::Mov {
                                    src: tmp_reg_op,
                                    dst: aast_destination.to_owned()
                                }
                            ]
                        },
                        _ => {
                            vec![ aast_instruction ]
                        }
                    }
                },
                aast::BinaryOperator::BitwiseLeftShift |
                aast::BinaryOperator::BitwiseRightShift => {
                    match &aast_source {
                        aast::Operand::Reg { reg: _ } |
                        aast::Operand::Pseudo { name: _ } |
                        aast::Operand::Stack { offset: _ } => {
                            vec![
                                aast::Instruction::Mov {
                                    src: aast_source.to_owned(),
                                    dst: aast::Operand::Reg {
                                        reg: aast::Register::CX
                                    }
                                },
                                aast::Instruction::Binary {
                                    operator: aast_binary_operator.to_owned(),
                                    src: aast::Operand::Reg {
                                        reg: aast::Register::CL
                                    },
                                    dst: aast_destination.to_owned()
                                }
                            ]
                        },
                        aast::Operand::Imm { int: _ } => {
                            vec![ aast_instruction ]
                        }
                    }
                }
            }
        },
        aast::Instruction::Cmp {
            loperand: aast_left_operand,
            roperand: aast_right_operand
        } => {
            match (&aast_left_operand, &aast_right_operand) {
                (
                    aast::Operand::Stack { offset: _ },
                    aast::Operand::Stack { offset: _ }
                ) => {
                    let tmp_reg_op: aast::Operand = aast::Operand::Reg {
                        reg: aast::Register::R10
                    };
                    vec![
                        aast::Instruction::Mov {
                            src: aast_left_operand.to_owned(),
                            dst: tmp_reg_op.clone()
                        },
                        aast::Instruction::Cmp {
                            loperand: tmp_reg_op,
                            roperand: aast_right_operand.to_owned()
                        }
                    ]
                },
                (
                    _,
                    aast::Operand::Imm { int: _ }
                ) => {
                    let tmp_reg_op: aast::Operand = aast::Operand::Reg {
                        reg: aast::Register::R11
                    };
                    vec![
                        aast::Instruction::Mov {
                            src: aast_right_operand.to_owned(),
                            dst: tmp_reg_op.clone()
                        },
                        aast::Instruction::Cmp {
                            loperand: aast_left_operand.to_owned(),
                            roperand: tmp_reg_op
                        }
                    ]
                },
                _ => {
                    vec![ aast_instruction ]
                }
            }
        }
        aast::Instruction::Idiv {
            operand: aast_operand
        } => {
            match &aast_operand {
                aast::Operand::Imm {
                    int: _
                } => {
                    let tmp_reg_op: aast::Operand = aast::Operand::Reg {
                        reg: aast::Register::R10
                    };
                    vec![
                        aast::Instruction::Mov {
                            src: aast_operand.to_owned(),
                            dst: tmp_reg_op.clone()
                        },
                        aast::Instruction::Idiv {
                            operand: tmp_reg_op
                        }
                    ]
                },
                _ => {
                    vec![ aast_instruction ]
                }
            }
        },
        aast::Instruction::Unary { operator: _, operand: _ } |
        aast::Instruction::Cdq |
        aast::Instruction::Jmp { target: _ } |
        aast::Instruction::JmpCC { condition: _, target: _ } |
        aast::Instruction::SetCC { condition: _, operand: _ } |
        aast::Instruction::Label { id: _ } |
        aast::Instruction::AllocateStack { bytes: _ } |
        aast::Instruction::Ret => {
            vec![ aast_instruction ]
        }
    }
}
