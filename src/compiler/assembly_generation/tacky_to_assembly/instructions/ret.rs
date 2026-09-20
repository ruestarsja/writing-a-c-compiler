use super::tast;
use super::aast;

use super::super::operand;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Return {
            val: tast_value
        } => {
            vec![

                aast::Instruction::Mov {
                    src: operand::generate(tast_value),
                    dst: aast::Operand::Reg {
                        reg: aast::Register::AX
                    }
                },
                
                aast::Instruction::Ret
                
            ]
        },
        _ => panic!("internal error: called `ret::generate()` with non-return instruction")
    }
}