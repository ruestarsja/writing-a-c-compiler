use super::aast::{ Function, Instruction };

use super::instruction;


pub(super) fn traverse(
    aast_function: Function,
    allocation_size: u32
) -> Function {
    let mut instructions: Vec<Instruction> = vec![
        Instruction::AllocateStack {
            bytes: allocation_size
        }
    ];
    for aast_instruction in aast_function.instructions {
        let mut more_instructions: Vec<Instruction> = instruction::traverse(aast_instruction);
        instructions.append(
            &mut more_instructions
        )
    }
    Function {
        name: aast_function.name,
        instructions
    }
}