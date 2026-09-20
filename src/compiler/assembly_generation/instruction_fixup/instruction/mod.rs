mod mov;
mod binary;
mod cmp;
mod idiv;

use super::aast::Instruction;


pub(super) fn traverse(
    aast_instruction: Instruction
) -> Vec<Instruction> {
    match aast_instruction {

        Instruction::Mov { src: _, dst: _ } => {
            mov::traverse(aast_instruction)
        },

        Instruction::Binary { operator: _, src: _, dst: _ } => {
            binary::traverse(aast_instruction)
        },

        Instruction::Cmp { loperand: _, roperand: _ } => {
            cmp::traverse(aast_instruction)
        },

        Instruction::Idiv { operand: _ } => {
            idiv::traverse(aast_instruction)
        },

        Instruction::Unary { operator: _, operand: _ } |
        Instruction::Cdq |
        Instruction::Jmp { target: _ } |
        Instruction::JmpCC { condition: _, target: _ } |
        Instruction::SetCC { condition: _, operand: _ } |
        Instruction::Label { id: _ } |
        Instruction::AllocateStack { bytes: _ } |
        Instruction::Ret => {
            vec![ aast_instruction ]
        }

    }
}