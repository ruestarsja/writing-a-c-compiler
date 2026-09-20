use super::tast;
use super::aast;

use super::super::identifier;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Jump {
            target: tast_target
        } => {
            vec![

                aast::Instruction::Jmp {
                    target: identifier::generate(tast_target)
                }

            ]
        },
        _ => panic!("internal error: called `jump::generate()` with non-jump instruction")
    }
}