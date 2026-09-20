use super::super::aast::{
    Instruction,
    Operand,
    Register
};


pub(super) fn traverse(
    aast_instruction: Instruction
) -> Vec<Instruction> {
    match aast_instruction {
        Instruction::Mov {
            src: aast_src,
            dst: aast_dst
        } => {

            match (&aast_src, &aast_dst) {
                (Operand::Stack { offset: _ }, Operand::Stack { offset: _ }) => {
                    let temp_reg: Operand = Operand::Reg {
                        reg: Register::R10
                    };
                    vec![
                        Instruction::Mov {
                            src: aast_src,
                            dst: temp_reg.clone()
                        },
                        Instruction::Mov {
                            src: temp_reg,
                            dst: aast_dst
                        }
                    ]
                },

                _ => vec![
                    Instruction::Mov {
                        src: aast_src,
                        dst: aast_dst
                    }
                ]
            }

        },
        _ => panic!("internal error: mov::traverse() called for non-mov instruction")
    }
}