mod ret;
mod unary;
mod binary;
mod copy;
mod jump;
mod jumpifzero;
mod jumpifnotzero;
mod label;

use super::tast;
use super::aast;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Return { val: _ }
            => ret::generate(tast_instruction),
        tast::Instruction::Unary { operator: _, src: _, dst: _}
            => unary::generate(tast_instruction),
        tast::Instruction::Binary { operator: _, lsrc: _, rsrc: _, dst: _ }
            => binary::generate(tast_instruction),
        tast::Instruction::Copy { src: _, dst: _ }
            => copy::generate(tast_instruction),
        tast::Instruction::Jump { target: _ }
            => jump::generate(tast_instruction),
        tast::Instruction::JumpIfZero { condition: _, target: _ }
            => jumpifzero::generate(tast_instruction),
        tast::Instruction::JumpIfNotZero { condition: _, target: _ }
            => jumpifnotzero::generate(tast_instruction),
        tast::Instruction::Label { id: _ }
            => label::generate(tast_instruction)
    }
}