use std::collections::HashMap;

use super::super::obj::aast;

static mut ABS_STACK_OFFSET: u32 = 0;

// FUNCTIONS

pub(super) fn main(assembly_ast: aast::Program) -> (aast::Program, u32) {
    let mut pseudo_stack_cache: HashMap<String, i32> = HashMap::new();
    let program: aast::Program = traverse_program(assembly_ast, &mut pseudo_stack_cache);
    unsafe {
        return (program, ABS_STACK_OFFSET);
    }
}

fn traverse_program(
    aast_program: aast::Program,
    pseudo_stack_cache: &mut HashMap<String, i32>
) -> aast::Program {
    let function_definition: aast::Function = traverse_function(aast_program.function_definition, pseudo_stack_cache);
    return aast::Program {
        function_definition
    };
}

fn traverse_function(
    aast_function: aast::Function,
    pseudo_stack_cache: &mut HashMap<String, i32>
) -> aast::Function {
    let mut instructions: Vec<aast::Instruction> = vec![];
    for aast_instruction in aast_function.instructions {
        instructions.push(
            traverse_instruction(aast_instruction, pseudo_stack_cache)
        );
    }
    return aast::Function {
        name: aast_function.name,
        instructions
    };
}

fn traverse_instruction(
    aast_instruction: aast::Instruction,
    pseudo_stack_cache: &mut HashMap<String, i32>
) -> aast::Instruction {
    match aast_instruction {
        aast::Instruction::Mov { 
            src: aast_src,
            dst: aast_dst
        } => {
            let src: aast::Operand = traverse_operand(aast_src, pseudo_stack_cache);
            let dst: aast::Operand = traverse_operand(aast_dst, pseudo_stack_cache);
            return aast::Instruction::Mov {
                src,
                dst
            }
        },
        aast::Instruction::Unary {
            operator: aast_operator,
            operand: aast_operand
        } => {
            let operand: aast::Operand = traverse_operand(aast_operand, pseudo_stack_cache);
            return aast::Instruction::Unary {
                operator: aast_operator,
                operand
            }
        },
        aast::Instruction::Binary {
            operator: aast_operator,
            src: aast_left_operand,
            dst: aast_right_operand
        } => {
            let loperand: aast::Operand = traverse_operand(aast_left_operand, pseudo_stack_cache);
            let roperand: aast::Operand = traverse_operand(aast_right_operand, pseudo_stack_cache);
            return aast::Instruction::Binary {
                operator: aast_operator,
                src: loperand,
                dst: roperand
            };
        },
        aast::Instruction::Cmp {
            loperand: aast_left_operand,
            roperand: aast_right_operand
        } => {
            let loperand: aast::Operand = traverse_operand(aast_left_operand, pseudo_stack_cache);
            let roperand: aast::Operand = traverse_operand(aast_right_operand, pseudo_stack_cache);
            return aast::Instruction::Cmp {
                loperand,
                roperand
            }
        },
        aast::Instruction::Idiv {
            operand: aast_operand
        } => {
            let operand: aast::Operand = traverse_operand(aast_operand, pseudo_stack_cache);
            return aast::Instruction::Idiv {
                operand
            };
        },
        aast::Instruction::SetCC {
            condition: aast_condition,
            operand: aast_operand
        } => {
            let operand: aast::Operand = traverse_operand(aast_operand, pseudo_stack_cache);
            return aast::Instruction::SetCC {
                condition: aast_condition,
                operand
            }
        },
        aast::Instruction::Cdq |
        aast::Instruction::Jmp { target: _ } |
        aast::Instruction::JmpCC { condition: _, target: _ } |
        aast::Instruction::Label { id: _ } |
        aast::Instruction::AllocateStack { bytes: _ } |
        aast::Instruction::Ret => {
            aast_instruction
        },
    }
}

fn traverse_operand(
    aast_operand: aast::Operand,
    pseudo_stack_cache: &mut HashMap<String, i32>
) -> aast::Operand {
    // dbg!(&aast_operand);
    // dbg!(&pseudo_stack_cache);
    match aast_operand {
        aast::Operand::Imm { int: _ } => { aast_operand },
        aast::Operand::Reg { reg: _ } => { aast_operand },
        aast::Operand::Pseudo {
            name
        } => {
            let cache_opt: Option<&i32> = pseudo_stack_cache.get(&name.id);
            // dbg!(&cache_opt);
            match cache_opt {
                Some(offset) => {
                    // dbg!(&offset);
                    aast::Operand::Stack {
                        offset: offset.clone()
                    }
                },
                None => {
                    let offset: i32 = get_new_stack_offset();
                    // dbg!(&offset);
                    pseudo_stack_cache.insert(name.id, offset);
                    aast::Operand::Stack {
                        offset
                    }
                }
            }
        },
        aast::Operand::Stack { offset: _ } => { aast_operand }
    }
}

fn get_new_stack_offset() -> i32 {
    unsafe {
        ABS_STACK_OFFSET += 4;
        return -1 * ABS_STACK_OFFSET.cast_signed();
    }
}
