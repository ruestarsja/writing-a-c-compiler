use super::obj::aast;

// FUNCTIONS

pub(super) fn main(
    filename: &str,
    assembly_ast: aast::Program
) {
    let mut assembly_code: String = String::new();

    assembly_code += &emit_program(assembly_ast);

    std::fs::write(filename, assembly_code).expect("could not write file");
}

fn emit_program(ast_program: aast::Program) -> String {
    let mut program_code: String = String::new();

    program_code += &emit_function(ast_program.function_definition);
    program_code += "\t.section .note.GNU-stack,\"\",@progbits\n";

    return program_code;
}

fn emit_function(ast_function: aast::Function) -> String {
    let mut function_code: String = String::new();

    let name_code: String = emit_identifier(ast_function.name);

    function_code += &format!("\t.globl {}\n", name_code);
    function_code += &format!("{}:\n", name_code);
    function_code += "\tpushq\t%rbp\n";
    function_code += "\tmovq\t%rsp,\t%rbp\n";
    for instruction in ast_function.instructions {
        function_code += &emit_instruction(instruction);
    }

    return function_code;
}

fn emit_identifier(ast_identifier: aast::Identifier) -> String {
    return ast_identifier.id;
}

fn emit_instruction(ast_instruction: aast::Instruction) -> String {
    match ast_instruction {
        aast::Instruction::Mov {
            src,
            dst
        } => {
            return format!("\tmovl\t{},\t{}\n", emit_operand(src), emit_operand(dst));
        },
        aast::Instruction::Unary {
            operator,
            operand
        } => {
            return format!("\t{}\t{}\n", emit_unary_operator(operator), emit_operand(operand));
        },
        aast::Instruction::Binary {
            operator,
            src,
            dst
        } => {
            return format!(
                "\t{}\t{},\t{}\n",
                emit_binary_operator(operator),
                emit_operand(src),
                emit_operand(dst)
            );
        },
        aast::Instruction::Cmp {
            loperand,
            roperand
        } => {
            return format!(
                "\tcmpl\t{},\t{}\n",
                emit_operand(loperand),
                emit_operand(roperand)
            );
        },
        aast::Instruction::Idiv {
            operand
        } => {
            return format!(
                "\tidivl\t{}\n",
                emit_operand(operand)
            );
        },
        aast::Instruction::Cdq => {
            return "\tcdq\n".to_owned();
        },
        aast::Instruction::Jmp {
            target
        } => {
            return format!(
                "\tjmp\t.L{}\n",
                emit_identifier(target)
            );
        },
        aast::Instruction::JmpCC {
            condition,
            target
        } => {
            return format!(
                "\tj{}\t.L{}\n",
                emit_condition_code(condition),
                emit_identifier(target)
            );
        },
        aast::Instruction::SetCC {
            condition,
            operand
        } => {
            return format!(
                "\tset{}\t{}\n",
                emit_condition_code(condition),
                emit_setcc_operand(operand)
            );
        },
        aast::Instruction::Label {
            id
        } => {
            return format!(
                ".L{}:\n",
                emit_identifier(id)
            );
        }
        aast::Instruction::AllocateStack {
            bytes
        } => {
            return format!("\tsubq\t${},\t%rsp\n", bytes);
        },
        aast::Instruction::Ret => {
            let mut return_code: String = String::new();

            return_code += "\tmovq\t%rbp,\t%rsp\n";
            return_code += "\tpopq\t%rbp\n";
            return_code += "\tret\n";

            return return_code;
        }
    }
}

fn emit_unary_operator(ast_unary_operator: aast::UnaryOperator) -> String {
    match ast_unary_operator {
        aast::UnaryOperator::Neg => {
            return "negl".to_owned();
        },
        aast::UnaryOperator::Not => {
            return "notl".to_owned();
        }
    }
}

fn emit_binary_operator(ast_binary_operator: aast::BinaryOperator) -> String {
    match ast_binary_operator {
        aast::BinaryOperator::Add => {
            return "addl".to_owned();
        },
        aast::BinaryOperator::Sub => {
            return "subl".to_owned();
        },
        aast::BinaryOperator::Mult => {
            return "imull".to_owned();
        },
        aast::BinaryOperator::BitwiseAnd => {
            return "andl".to_owned();
        },
        aast::BinaryOperator::BitwiseOr => {
            return "orl".to_owned();
        },
        aast::BinaryOperator::BitwiseXor => {
            return "xorl".to_owned();
        },
        aast::BinaryOperator::BitwiseLeftShift => {
            return "sall".to_owned();
        },
        aast::BinaryOperator::BitwiseRightShift => {
            return "sarl".to_owned();
        }
    }
}

fn emit_operand(ast_operand: aast::Operand) -> String {
    match ast_operand {
        aast::Operand::Imm {
            int
        } => {
            return format!("${}", int);
        },
        aast::Operand::Reg {
            reg
        } => {
            match reg {
                aast::Register::AX => {
                    return "%eax".to_owned();
                },
                aast::Register::CX => {
                    return "%ecx".to_owned();
                },
                aast::Register::CL => {
                    return "%cl".to_owned();
                },
                aast::Register::DX => {
                    return "%edx".to_owned();
                },
                aast::Register::R10 => {
                    return "%r10d".to_owned();
                },
                aast::Register::R11 => {
                    return "%r11d".to_owned();
                }
            }
        },
        aast::Operand::Pseudo { name: _ } => {
            panic!("internal error: pseudoregister not replaced upon code emission");
        },
        aast::Operand::Stack {
            offset
        } => {
            return format!("{}(%rbp)", offset);
        }
    }
}

fn emit_setcc_operand(ast_operand: aast::Operand) -> String {
    match ast_operand {
        aast::Operand::Imm {
            int
        } => {
            return format!("${}", int);
        },
        aast::Operand::Reg {
            reg
        } => {
            match reg {
                aast::Register::AX => {
                    return "%al".to_owned();
                },
                aast::Register::CX => {
                    return "%cl".to_owned();
                },
                aast::Register::CL => {
                    return "%cl".to_owned();
                }
                aast::Register::DX => {
                    return "%dl".to_owned();
                },
                aast::Register::R10 => {
                    return "%r10b".to_owned();
                }
                aast::Register::R11 => {
                    return "%r11b".to_owned();
                }
            }
        },
        aast::Operand::Pseudo { name: _ } => {
            panic!("internal error: pseudoregister not replaced upon code emission")
        },
        aast::Operand::Stack {
            offset
        } => {
            return format!("{}(%rbp)", offset);
        }
    }
}

fn emit_condition_code(ast_condition_code: aast::ConditionCode) -> String {
    match ast_condition_code {
        aast::ConditionCode::E => {
            return "e".to_owned();
        },
        aast::ConditionCode::NE => {
            return "ne".to_owned();
        },
        aast::ConditionCode::L => {
            return "l".to_owned();
        },
        aast::ConditionCode::LE => {
            return "le".to_owned();
        },
        aast::ConditionCode::G => {
            return "g".to_owned();
        },
        aast::ConditionCode::GE => {
            return "ge".to_owned();
        }
    }
}

fn _emit_register(_ast_register: aast::Register) -> String {
    return "%eax".to_owned();
}
