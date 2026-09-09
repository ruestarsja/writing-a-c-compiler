use super::obj::{cast,tast};

static mut VAR_COUNTER: u32 = 0;

static mut AND_LABEL_COUNTER: u32 = 0;
static mut OR_LABEL_COUNTER: u32 = 0;
static mut END_BOOLOP_LABEL_COUNTER: u32 = 0;

static mut ELSE_LABEL_COUNTER: u32 = 0;
static mut END_IF_LABEL_COUNTER: u32 = 0;

// FUNCTIONS

pub(super) fn main(ast: cast::Program) -> tast::Program {
    generate_program(ast)
}

fn generate_program(ast_program: cast::Program) -> tast::Program {
    tast::Program {
        function_definition: generate_function(ast_program.function_definition)
    }
}

fn generate_function(ast_func_def: cast::Function) -> tast::Function {
    tast::Function {
        name: generate_identifier(ast_func_def.name),
        body: generate_instructions(ast_func_def.body)
    }
}

fn generate_identifier(ast_identifier: cast::Identifier) -> tast::Identifier {
    tast::Identifier {
        id: ast_identifier.id
    }
}

fn generate_instructions(ast_block: Vec<cast::BlockItem>) -> Vec<tast::Instruction> {
    let mut instructions: Vec<tast::Instruction> = vec![];
    for ast_block_item in ast_block {
        match ast_block_item {
            cast::BlockItem::Declaration {
                declaration: ast_declaration
            } => {
                match ast_declaration.init {
                    Some(exp) => {
                        let mut more_instructions: Vec<tast::Instruction>;
                        let val: tast::Value;
                        (more_instructions, val) = generate_instructions_and_value(exp);
                        more_instructions.push(tast::Instruction::Copy {
                            src: val,
                            dst: tast::Value::Variable {
                                name: generate_identifier(ast_declaration.name)
                            }
                        });
                        instructions.append(&mut more_instructions);
                    },
                    None => {}
                }
            },
            cast::BlockItem::Statement {
                statement: ast_statement
            } => {
                match ast_statement {
                    cast::Statement::Return {
                        exp: ast_exp
                    } => {
                        let mut more_instructions: Vec<tast::Instruction>;
                        let val: tast::Value;
                        (more_instructions, val) = generate_instructions_and_value(ast_exp);
                        more_instructions.push(
                            tast::Instruction::Return { val }
                        );
                        instructions.append(&mut more_instructions);
                    },
                    cast::Statement::Expression {
                        exp: ast_exp
                    } => {
                        let mut more_instructions: Vec<tast::Instruction>;
                        (more_instructions, _) = generate_instructions_and_value(ast_exp);
                        instructions.append(&mut more_instructions);
                    },
                    cast::Statement::If {
                        condition: ast_condition,
                        then_statement: ast_then,
                        else_statement: ast_else
                    } => {
                        match ast_else {
                            Some(else_box) => {
                                let else_label: String = make_if_else_label_name(true);
                                let end_label: String = make_if_else_label_name(false);
                                let mut conditional_instructions: Vec<tast::Instruction>;
                                let conditional_val: tast::Value;
                                (conditional_instructions, conditional_val) = generate_instructions_and_value(ast_condition);
                                instructions.append(&mut conditional_instructions);
                                instructions.push(tast::Instruction::JumpIfZero {
                                    condition: conditional_val,
                                    target: tast::Identifier {
                                        id: else_label
                                    }
                                });
                                let mut then_instructions: Vec<tast::Instruction>;
                                match *ast_then {
                                    cast::Statement::Return {
                                        exp: ast_exp
                                    } => {
                                        let val: tast::Value;
                                        (then_instructions, val) = generate_instructions_and_value(ast_exp);
                                        instructions.append(&mut then_instructions);
                                        instructions.push(
                                            tast::Instruction::Return {
                                                val
                                            }
                                        );
                                    },
                                    cast::Statement::Expression {
                                        exp: ast_exp
                                    } => {
                                        (then_instructions, _) = generate_instructions_and_value(ast_exp);
                                        instructions.append(&mut then_instructions);
                                    },
                                    cast::Statement::If {
                                        condition: _,
                                        then_statement: _,
                                        else_statement: _
                                    } => {
                                        todo!("not yet implemented: code must be refactored to handle nested ifs");
                                    },
                                    cast::Statement::Null => {}
                                }
                                instructions.push(tast::Instruction::Jump {
                                    target: tast::Identifier {
                                        id: end_label
                                    }
                                });
                                let mut else_instructions: Vec<tast::Instruction>;
                                match *else_box {
                                    cast::Statement::Return {
                                        exp: ast_exp
                                    } => {
                                        let val: tast::Value;
                                        (else_instructions, val) = generate_instructions_and_value(ast_exp);
                                        instructions.append(&mut else_instructions);
                                        instructions.push(
                                            tast::Instruction::Return {
                                                val
                                            }
                                        );
                                    },
                                    cast::Statement::Expression {
                                        exp: ast_exp
                                    } => {
                                        (else_instructions, _) = generate_instructions_and_value(ast_exp);
                                        instructions.append(&mut else_instructions);
                                    },
                                    cast::Statement::If {
                                        condition: _,
                                        then_statement: _,
                                        else_statement: _
                                    } => {
                                        todo!("not yet implemented: code must be refactored to handle nested ifs");
                                    },
                                    cast::Statement::Null => {}
                                }
                            },
                            None => {
                                let end_label: String = make_if_else_label_name(false);
                                let mut conditional_instructions: Vec<tast::Instruction>;
                                let conditional_val: tast::Value;
                                (conditional_instructions, conditional_val) = generate_instructions_and_value(ast_condition);
                                instructions.append(&mut conditional_instructions);
                                instructions.push(tast::Instruction::JumpIfZero {
                                    condition: conditional_val,
                                    target: tast::Identifier {
                                        id: end_label
                                    }
                                });
                                let mut then_instructions: Vec<tast::Instruction>;
                                match *ast_then {
                                    cast::Statement::Return {
                                        exp: ast_exp
                                    } => {
                                        let val: tast::Value;
                                        (then_instructions, val) = generate_instructions_and_value(ast_exp);
                                        instructions.append(&mut then_instructions);
                                        instructions.push(
                                            tast::Instruction::Return {
                                                val
                                            }
                                        );
                                    },
                                    cast::Statement::Expression {
                                        exp: ast_exp
                                    } => {
                                        (then_instructions, _) = generate_instructions_and_value(ast_exp);
                                        instructions.append(&mut then_instructions);
                                    },
                                    cast::Statement::If {
                                        condition: _,
                                        then_statement: _,
                                        else_statement: _
                                    } => {
                                        todo!("not yet implemented: code must be refactored to handle nested ifs");
                                    },
                                    cast::Statement::Null => {}
                                }
                            }
                        }
                    },
                    cast::Statement::Null => {}
                }
            }
        }
    }
    instructions.push(tast::Instruction::Return {
        val: tast::Value::Constant {
            int: 0
        }
    });
    return instructions;
}

fn generate_instructions_and_value(ast_exp: cast::Expression) -> (Vec<tast::Instruction>, tast::Value) {
    match ast_exp {
        cast::Expression::Constant {
            int: ast_int
        } => {
            let instructions: Vec<tast::Instruction> = vec![];
            let val: tast::Value = tast::Value::Constant{ int: ast_int };
            return (instructions, val);
        },
        cast::Expression::Var {
            id: ast_id
        } => {
            let instructions: Vec<tast::Instruction> = vec![];
            let val: tast::Value = tast::Value::Variable {
                name: generate_identifier(ast_id)
            };
            return (instructions, val);
        },
        cast::Expression::Unary {
            operator: ast_unary_operator,
            factor: ast_inner_exp
        } => {
            let mut instructions: Vec<tast::Instruction>;
            let src: tast::Value;
            (instructions, src) = generate_instructions_and_value(*ast_inner_exp);
            match ast_unary_operator {
                cast::UnaryOperator::Complement |
                cast::UnaryOperator::Negate |
                cast::UnaryOperator::Not => {
                    let dst: tast::Value = tast::Value::Variable {
                        name: tast::Identifier {
                            id: make_temporary_variable_name()
                        }
                    };
                    instructions.push(
                        tast::Instruction::Unary {
                            operator: generate_unary_operator(ast_unary_operator),
                            src,
                            dst: dst.clone()
                        }
                    );
                    return (instructions, dst);
                },
                cast::UnaryOperator::PreIncrement |
                cast::UnaryOperator::PreDecrement => {
                    instructions.push(
                        tast::Instruction::Binary {
                            operator: match ast_unary_operator {
                                cast::UnaryOperator::PreIncrement => tast::BinaryOperator::Add,
                                cast::UnaryOperator::PreDecrement => tast::BinaryOperator::Subtract,
                                _ => panic!("this arm should be unreachable")
                            },
                            lsrc: src.clone(),
                            rsrc: tast::Value::Constant {
                                int: 1
                            },
                            dst: src.clone()
                        }
                    );
                    return (instructions, src);
                },
                cast::UnaryOperator::PostIncrement |
                cast::UnaryOperator::PostDecrement => {
                    let dst: tast::Value = tast::Value::Variable {
                        name: tast::Identifier {
                            id: make_temporary_variable_name()
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
                            operator: match ast_unary_operator {
                                cast::UnaryOperator::PostIncrement => tast::BinaryOperator::Add,
                                cast::UnaryOperator::PostDecrement => tast::BinaryOperator::Subtract,
                                _ => panic!("this arm should be unreachable")
                            },
                            lsrc: src.clone(),
                            rsrc: tast::Value::Constant {
                                int: 1
                            },
                            dst: src
                        }
                    );
                    return (instructions, dst);
                }
            }
        },
        cast::Expression::Binary {
            operator: ast_binary_operator,
            lfactor,
            rfactor
        } => {
            let mut instructions: Vec<tast::Instruction>;
            let dst: tast::Value = tast::Value::Variable {
                name: tast::Identifier {
                    id: make_temporary_variable_name()
                }
            };
            match ast_binary_operator {
                // short circuit operators
                cast::BinaryOperator::And |
                cast::BinaryOperator::Or => {
                    let lsrc: tast::Value;
                    (instructions, lsrc) = generate_instructions_and_value(*lfactor);
                    let short_circuit_label: tast::Identifier = tast::Identifier {
                        id: make_and_or_label_name(Some(&ast_binary_operator))
                    };
                    instructions.push(
                        match ast_binary_operator {
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
                                panic!("this arm should be unreachable")
                            }
                        }
                    );
                    let mut rinstructions: Vec<tast::Instruction>;
                    let rsrc: tast::Value;
                    (rinstructions, rsrc) = generate_instructions_and_value(*rfactor);
                    instructions.append(&mut rinstructions);
                    instructions.push(
                        match ast_binary_operator {
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
                                panic!("this arm should be unreachable")
                            }
                        }
                    );
                    instructions.push(tast::Instruction::Copy {
                        src: tast::Value::Constant {
                            int: match ast_binary_operator {
                                cast::BinaryOperator::And => 1,
                                cast::BinaryOperator::Or => 0,
                                _ => panic!("this arm should be unreachable")
                            }
                        },
                        dst: dst.clone()
                    });
                    let end_label: tast::Identifier = tast::Identifier {
                        id: make_and_or_label_name(None)
                    };
                    instructions.push(tast::Instruction::Jump {
                        target: end_label.clone()
                    });
                    instructions.push(tast::Instruction::Label {
                        id: short_circuit_label
                    });
                    instructions.push(tast::Instruction::Copy {
                        src: tast::Value::Constant {
                            int: match ast_binary_operator {
                                cast::BinaryOperator::And => 0,
                                cast::BinaryOperator::Or => 1,
                                _ => panic!("this arm should be unreachable")
                            }
                        },
                        dst: dst.clone()
                    });
                    instructions.push(tast::Instruction::Label {
                        id: end_label
                    });
                },
                // everything else
                ast_binary_operator => {
                    let lsrc: tast::Value;
                    (instructions, lsrc) = generate_instructions_and_value(*lfactor);
                    let mut rinstructions: Vec<tast::Instruction>;
                    let rsrc: tast::Value;
                    (rinstructions, rsrc) = generate_instructions_and_value(*rfactor);
                    instructions.append(&mut rinstructions);
                    let operator = generate_binary_operator(ast_binary_operator);
                    instructions.push(
                        tast::Instruction::Binary {
                            operator,
                            lsrc,
                            rsrc,
                            dst: dst.clone()
                        }
                    );
                }
            }
            return (instructions, dst);
        },
        cast::Expression::Assignment {
            lvalue: ast_left_value,
            exp: ast_expression
        } => {
            let mut instructions: Vec<tast::Instruction>;
            let src: tast::Value;
            (instructions, src) = generate_instructions_and_value(*ast_expression);
            let val: tast::Value;
            (_, val) = generate_instructions_and_value(*ast_left_value);
            instructions.push(tast::Instruction::Copy {
                src,
                dst: val.clone()
            });
            return (instructions, val);
        },
        cast::Expression::Conditional { condition: _, then_expression: _, else_expression: _ } => {
            todo!("not yet implemented: generate tacky for conditional statements")
        }
    }
}

fn generate_unary_operator(ast_unary_operator: cast::UnaryOperator) -> tast::UnaryOperator {
    match ast_unary_operator {
        cast::UnaryOperator::Complement => {
            tast::UnaryOperator::Complement
        },
        cast::UnaryOperator::Negate => {
            tast::UnaryOperator::Negate
        },
        cast::UnaryOperator::Not => {
            tast::UnaryOperator::Not
        },
        cast::UnaryOperator::PreIncrement |
        cast::UnaryOperator::PreDecrement |
        cast::UnaryOperator::PostIncrement |
        cast::UnaryOperator::PostDecrement => {
            panic!("internal error: do not use generate_unary_operator for increment and decrement operators, since ordering is relavent for tast generation");
        }
    }
}

fn generate_binary_operator(ast_binary_operator: cast::BinaryOperator) -> tast::BinaryOperator {
    match ast_binary_operator {
        cast::BinaryOperator::Add => {
            tast::BinaryOperator::Add
        },
        cast::BinaryOperator::Subtract => {
            tast::BinaryOperator::Subtract
        },
        cast::BinaryOperator::Multiply => {
            tast::BinaryOperator::Multiply
        },
        cast::BinaryOperator::Divide => {
            tast::BinaryOperator::Divide
        },
        cast::BinaryOperator::Remainder => {
            tast::BinaryOperator::Remainder
        },
        cast::BinaryOperator::BitwiseAnd => {
            tast::BinaryOperator::BitwiseAnd
        },
        cast::BinaryOperator::BitwiseOr => {
            tast::BinaryOperator::BitwiseOr
        },
        cast::BinaryOperator::BitwiseXor => {
            tast::BinaryOperator::BitwiseXor
        },
        cast::BinaryOperator::BitwiseLeftShift => {
            tast::BinaryOperator::BitwiseLeftShift
        },
        cast::BinaryOperator::BitwiseRightShift => {
            tast::BinaryOperator::BitwiseRightShift
        },
        cast::BinaryOperator::And |
        cast::BinaryOperator::Or => {
            panic!("internal error: do not call generate_binary_operator for boolean `and` and `or` (they require custom tacky generation)");
        },
        cast::BinaryOperator::Equal => {
            tast::BinaryOperator::Equal
        },
        cast::BinaryOperator::NotEqual => {
            tast::BinaryOperator::NotEqual
        },
        cast::BinaryOperator::LessThan => {
            tast::BinaryOperator::LessThan
        },
        cast::BinaryOperator::LessThanOrEqual => {
            tast::BinaryOperator::LessThanOrEqual
        },
        cast::BinaryOperator::GreaterThan => {
            tast::BinaryOperator::GreaterThan
        },
        cast::BinaryOperator::GreaterThanOrEqual => {
            tast::BinaryOperator::GreaterThanOrEqual
        }
    }
}

fn make_temporary_variable_name() -> String {
    let count: u32;
    unsafe {
        count = VAR_COUNTER;
        VAR_COUNTER += 1;
    }
    return format!("$tmp{}", count);
}

fn make_and_or_label_name(operator: Option<&cast::BinaryOperator>) -> String {
    let count: u32;
    match operator {
        None => {
            unsafe {
                count = END_BOOLOP_LABEL_COUNTER;
                END_BOOLOP_LABEL_COUNTER += 1;
            }
            return format!("end_boolop_{}", count);
        },
        Some(cast::BinaryOperator::And) => {
            unsafe {
                count = AND_LABEL_COUNTER;
                AND_LABEL_COUNTER += 1;
            }
            return format!("and_false_{}", count);
        },
        Some(cast::BinaryOperator::Or) => {
            unsafe {
                count = OR_LABEL_COUNTER;
                OR_LABEL_COUNTER += 1;
            }
            return format!("or_true_{}", count);
        },
        Some(_) => {
            panic!("internal error: do not call `make_label_name` with a binary operator that doesn't short-circuit");
        }
    }
}

fn make_if_else_label_name(is_else: bool) -> String {
    let count: u32;
    if is_else {
        unsafe {
            count = ELSE_LABEL_COUNTER;
            ELSE_LABEL_COUNTER += 1;
        }
        return format!("else_{}", count);
    } else {
        unsafe {
            count = END_IF_LABEL_COUNTER;
            END_IF_LABEL_COUNTER += 1;
        }
        return format!("end_if_{}", count);
    }
}
