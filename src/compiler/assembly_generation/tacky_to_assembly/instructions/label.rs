use super::tast;
use super::aast;

use super::super::identifier;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Label {
            id: tast_id
        } => {
            vec![

                aast::Instruction::Label {
                    id: identifier::generate(tast_id)
                }

            ]
        },
        _ => panic!("internal error: called `label::generate()` with non-label instruction")
    }
}