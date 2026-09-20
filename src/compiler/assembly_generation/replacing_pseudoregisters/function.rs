use super::aast::{ Function, Instruction };
use super::PseudoStackCache;

use super::instruction;


pub(super) fn traverse(
    aast_function: Function,
    pseudo_stack_cache: &mut PseudoStackCache
) -> Function {
    let mut instructions: Vec<Instruction> = vec![];
    for aast_instruction in aast_function.instructions {
        instructions.push(
            instruction::traverse(aast_instruction, pseudo_stack_cache)
        );
    }

    Function {
        name: aast_function.name,
        instructions
    }
}