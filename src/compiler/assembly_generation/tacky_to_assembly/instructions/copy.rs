use super::tast;
use super::aast;

use super::super::operand;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Copy {
            src: tast_source,
            dst: tast_destination
        } => {
            vec![

                aast::Instruction::Mov {
                    src: operand::generate(tast_source),
                    dst: operand::generate(tast_destination)
                }

            ]
        },
        _ => panic!("internal error: called `copy::generate()` with non-copy instruction")
    }
}