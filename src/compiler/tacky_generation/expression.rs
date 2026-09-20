use super::cast;
use super::tast;

use super::id;
use super::identifier;
use super::operators;


pub(super) fn generate_instructions(
    cast_exp: cast::Expression
) -> Vec<tast::Instruction> {
    let instructions: Vec<tast::Instruction>;
    (instructions, _) = generate_instructions_and_value(cast_exp);
    instructions
}


pub(super) fn generate_instructions_and_value(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {

        cast::Expression::Constant {
            int: _
        } => {
            generate_from_constant(cast_exp)
        },

        cast::Expression::Var {
            id: _
        } => {
            generate_from_var(cast_exp)
        },

        cast::Expression::Unary {
            operator: _,
            factor: _
        } => {
            generate_from_unary(cast_exp)
        },

        cast::Expression::Binary {
            operator: _,
            lfactor: _,
            rfactor: _
        } => {
            generate_from_binary(cast_exp)
        },

        cast::Expression::Assignment {
            lvalue: _,
            exp: _
        } => {
            generate_from_assignment(cast_exp)
        },

        cast::Expression::Conditional {
            condition: _,
            exp_if_true: _,
            exp_if_false: _
        } => {
            generate_from_conditional(cast_exp)
        }
    }
}


fn generate_from_constant(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {
        cast::Expression::Constant {
            int: cast_int
        } => {
            let val: tast::Value = tast::Value::Constant {
                int: cast_int
            };
            (vec![], val)
        },
        _ => {
            panic!("internal error: `generate_from_constant()` called with non-constant expression");
        }
    }
}


fn generate_from_var(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {
        cast::Expression::Var {
            id: cast_id
        } => {
            let val: tast::Value = tast::Value::Variable {
                name: identifier::generate(cast_id)
            };
            (vec![], val)
        },
        _ => {
            panic!("internal error: `generate_from_var()` called with non-var expression");
        }
    }
}


fn generate_from_unary(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {
        cast::Expression::Unary {
            operator: cast_unary_operator,
            factor: cast_inner_exp
        } => {
            let mut instructions: Vec<tast::Instruction>;
            let src: tast::Value;
            (instructions, src) = generate_instructions_and_value(*cast_inner_exp);
            let val: tast::Value;
            match cast_unary_operator {
                cast::UnaryOperator::PreIncrement |
                cast::UnaryOperator::PreDecrement => {
                    instructions.push(
                        tast::Instruction::Binary {
                            operator: operators::binary::generate_from_unary(cast_unary_operator),
                            lsrc: src.clone(),
                            rsrc: tast::Value::Constant {
                                int: 1
                            },
                            dst: src.clone()
                        }
                    );
                    val = src;
                },
                cast::UnaryOperator::PostIncrement |
                cast::UnaryOperator::PostDecrement => {
                    let dst: tast::Value = tast::Value::Variable {
                        name: tast::Identifier {
                            id: id::generate(id::IDType::TemporaryVariableName)
                        }
                    };
                    instructions.push(
                        tast::Instruction::Copy {
                            src: src.clone(),
                            dst: dst.clone()
                        }
                    );
                    instructions.push(
                        tast::Instruction::Binary {
                            operator: operators::binary::generate_from_unary(cast_unary_operator),
                            lsrc: src.clone(),
                            rsrc: tast::Value::Constant {
                                int: 1
                            },
                            dst: src
                        }
                    );
                    val = dst;
                },
                _ => {
                    let dst: tast::Value = tast::Value::Variable {
                        name: tast::Identifier {
                            id: id::generate(id::IDType::TemporaryVariableName)
                        }
                    };
                    instructions.push(
                        tast::Instruction::Unary {
                            operator: operators::unary::generate(cast_unary_operator),
                            src,
                            dst: dst.clone()
                        }
                    );
                    val = dst;
                }
            }
            (instructions, val)
        },
        _ => {
            panic!("internal error: `generate_from_unary()` called with non-unary expression");
        }
    }
}


fn generate_from_binary(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {
        cast::Expression::Binary {
            operator: cast_binary_operator,
            lfactor: cast_lfactor,
            rfactor: cast_rfactor
        } => {
            let mut instructions: Vec<tast::Instruction> = vec![];
            let mut linstructions: Vec<tast::Instruction>;
            let mut rinstructions: Vec<tast::Instruction>;

            let lsrc: tast::Value;
            let rsrc: tast::Value;
            let dst: tast::Value = tast::Value::Variable {
                name: tast::Identifier {
                    id: id::generate(id::IDType::TemporaryVariableName)
                }
            };

            (linstructions, lsrc) = generate_instructions_and_value(*cast_lfactor);
            (rinstructions, rsrc) = generate_instructions_and_value(*cast_rfactor);

            instructions.append(&mut linstructions);
            match cast_binary_operator {
                // short-circuit operators
                cast::BinaryOperator::And |
                cast::BinaryOperator::Or => {
                    let short_circuit_label: tast::Identifier = tast::Identifier {
                        id: id::generate(
                            match &cast_binary_operator {
                                cast::BinaryOperator::And => id::IDType::AndLabel,
                                cast::BinaryOperator::Or => id::IDType::OrLabel,
                                _ => panic!("internal error: this arm should be unreachable")
                            }
                        )
                    };
                    instructions.push(
                        match cast_binary_operator {
                            cast::BinaryOperator::And => {
                                tast::Instruction::JumpIfZero {
                                    condition: lsrc,
                                    target: short_circuit_label.clone()
                                }
                            },
                            cast::BinaryOperator::Or => {
                                tast::Instruction::JumpIfNotZero {
                                    condition: lsrc,
                                    target: short_circuit_label.clone()
                                }
                            },
                            _ => {
                                panic!("internal error: this arm should be unreachable");
                            }
                        }
                    );
                    instructions.append(&mut rinstructions);
                    instructions.push(
                        match cast_binary_operator {
                            cast::BinaryOperator::And => {
                                tast::Instruction::JumpIfZero {
                                    condition: rsrc,
                                    target: short_circuit_label.clone()
                                }
                            },
                            cast::BinaryOperator::Or => {
                                tast::Instruction::JumpIfNotZero {
                                    condition: rsrc,
                                    target: short_circuit_label.clone()
                                }
                            },
                            _ => {
                                panic!("internal error: this arm should be unreachable");
                            }
                        }
                    );
                    instructions.push(
                        tast::Instruction::Copy {
                            src: tast::Value::Constant {
                                int: match cast_binary_operator {
                                    cast::BinaryOperator::And => 1,
                                    cast::BinaryOperator::Or => 0,
                                    _ => panic!("internal error: this arm should be unreachable")
                                }
                            },
                            dst: dst.clone()
                        }
                    );
                    let end_label: tast::Identifier = tast::Identifier {
                        id: id::generate(id::IDType::EndBooleanOperatorLabel)
                    };
                    instructions.push(
                        tast::Instruction::Jump {
                            target: end_label.clone()
                        }
                    );
                    instructions.push(
                        tast::Instruction::Label {
                            id: short_circuit_label
                        }
                    );
                    instructions.push(
                        tast::Instruction::Copy {
                            src: tast::Value::Constant {
                                int: match cast_binary_operator {
                                    cast::BinaryOperator::And => 0,
                                    cast::BinaryOperator::Or => 1,
                                    _ => panic!("internal error: this arm should be unreachable")
                                }
                            },
                            dst: dst.clone()
                        }
                    );
                    instructions.push(
                        tast::Instruction::Label {
                            id: end_label
                        }
                    );
                },
                // everything else
                _ => {
                    instructions.append(&mut rinstructions);
                    let operator = operators::binary::generate(cast_binary_operator);
                    instructions.push(
                        tast::Instruction::Binary {
                            operator,
                            lsrc,
                            rsrc,
                            dst: dst.clone()
                        }
                    )
                }
            }
            (instructions, dst)
        },
        _ => {
            panic!("internal error: `generate_from_binary()` called with non-binary expression");
        }
    }
}


fn generate_from_assignment(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {
        cast::Expression::Assignment {
            lvalue: cast_lvalue,
            exp: cast_inner_exp
        } => {
            let mut instructions: Vec<tast::Instruction>;

            let src: tast::Value;
            let val: tast::Value;

            (instructions, src) = generate_instructions_and_value(*cast_inner_exp);
            (_, val) = generate_instructions_and_value(*cast_lvalue);

            instructions.push(
                tast::Instruction::Copy {
                    src,
                    dst: val.clone()
                }
            );

            (instructions, val)
        },
        _ => {
            panic!("internal error: `generate_from_assignment()` called with non-assignment expression");
        }
    }
}


fn generate_from_conditional(
    cast_exp: cast::Expression
) -> (Vec<tast::Instruction>, tast::Value) {
    match cast_exp {
        cast::Expression::Conditional {
            condition: cast_condition,
            exp_if_true: cast_expression_if_true,
            exp_if_false: cast_expression_if_false
        } => {
            let mut instructions: Vec<tast::Instruction> = vec![];
            let mut cond_instructions: Vec<tast::Instruction>;
            let mut exp_if_true_instructions: Vec<tast::Instruction>;
            let mut exp_if_false_instructions: Vec<tast::Instruction>;

            let cond_val: tast::Value;
            let exp_if_true_val: tast::Value;
            let exp_if_false_val: tast::Value;
            let val: tast::Value = tast::Value::Variable {
                name: tast::Identifier {
                    id: id::generate(id::IDType::TemporaryVariableName)
                }
            };

            (cond_instructions, cond_val) = generate_instructions_and_value(*cast_condition);
            (exp_if_true_instructions, exp_if_true_val) = generate_instructions_and_value(*cast_expression_if_true);
            (exp_if_false_instructions, exp_if_false_val) = generate_instructions_and_value(*cast_expression_if_false);
            
            let else_label_name: tast::Identifier = tast::Identifier {
                id: id::generate(id::IDType::ElseLabel)
            };
            let end_label_name: tast::Identifier = tast::Identifier {
                id: id::generate(id::IDType::EndIfLabel)
            };
            
            instructions.append(&mut cond_instructions);
            instructions.push(
                tast::Instruction::JumpIfZero {
                    condition: cond_val,
                    target: else_label_name.clone()
                }
            );
            instructions.append(&mut exp_if_true_instructions);
            instructions.append(
                &mut vec![
                    tast::Instruction::Copy {
                        src: exp_if_true_val,
                        dst: val.clone()
                    },
                    tast::Instruction::Jump {
                        target: end_label_name.clone()
                    },
                    tast::Instruction::Label {
                        id: else_label_name
                    },
                ]
            );
            instructions.append(&mut exp_if_false_instructions);
            instructions.append(
                &mut vec![
                    tast::Instruction::Copy {
                        src: exp_if_false_val,
                        dst: val.clone()
                    },
                    tast::Instruction::Label {
                        id: end_label_name
                    }
                ]
            );
            
            (instructions, val)
        },
        _ => {
            panic!("internal error: `generate_from_conditional()` called with non-conditional expression");
        }
    }
}