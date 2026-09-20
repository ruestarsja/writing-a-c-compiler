use super::super::aast::{
    Instruction,
    Operand,
    Register
};


pub(super) fn traverse(
    aast_instruction: Instruction
) -> Vec<Instruction> {
    match aast_instruction {
        Instruction::Idiv {
            operand: aast_operand
        } => {

            match aast_operand {
                Operand::Imm { int: _ } => {
                    let temp_reg: Operand = Operand::Reg {
                        reg: Register::R10
                    };
                    vec![
                        Instruction::Mov {
                            src: aast_operand,
                            dst: temp_reg.clone()
                        },
                        Instruction::Idiv {
                            operand: temp_reg
                        }
                    ]
                },
                _ => vec![
                    Instruction::Idiv {
                        operand: aast_operand
                    }
                ]
            }

        },
        _ => panic!("internal error: called idiv::traverse() with non-idiv instruction")
    }
}