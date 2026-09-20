use super::tast;
use super::aast;

use super::super::identifier;
use super::super::operand;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::JumpIfNotZero {
            condition: tast_condition,
            target: tast_target
        } => {
            vec![

                aast::Instruction::Cmp {
                    loperand: aast::Operand::Imm {
                        int: 0
                    },
                    roperand: operand::generate(tast_condition)
                },

                aast::Instruction::JmpCC {
                    condition: aast::ConditionCode::NE,
                    target: identifier::generate(tast_target)
                }

            ]
        },
        _ => panic!("internal error: called `jumpifnotzero::generate()` with non-jump-if-not-zero instruction")
    }
}