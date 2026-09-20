use super::super::aast::{
    Instruction,
    Operand,
    Register
};


pub(super) fn traverse(
    aast_instruction: Instruction
) -> Vec<Instruction> {
    match aast_instruction {
        Instruction::Cmp {
            loperand: aast_loperand,
            roperand: aast_roperand
        } => {

            match (&aast_loperand, &aast_roperand) {
                (Operand::Stack { offset: _ }, Operand::Stack { offset: _ }) => {
                    let temp_reg: Operand = Operand::Reg {
                        reg: Register::R10
                    };
                    vec![
                        Instruction::Mov {
                            src: aast_loperand,
                            dst: temp_reg.clone()
                        },
                        Instruction::Cmp {
                            loperand: temp_reg,
                            roperand: aast_roperand
                        }
                    ]
                },
                (_, Operand::Imm { int: _ }) => {
                    let temp_reg: Operand = Operand::Reg {
                        reg: Register::R11
                    };
                    vec![
                        Instruction::Mov {
                            src: aast_roperand,
                            dst: temp_reg.clone()
                        },
                        Instruction::Cmp {
                            loperand: aast_loperand,
                            roperand: temp_reg
                        }
                    ]
                },
                _ => vec![
                    Instruction::Cmp {
                        loperand: aast_loperand,
                        roperand: aast_roperand
                    }
                ]
            }

        },
        _ => panic!("internal error: called cmp::traverse() for non-cmp instruction")
    }
}