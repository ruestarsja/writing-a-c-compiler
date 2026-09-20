use super::tast;
use super::aast;

use super::super::operand;
use super::super::operators;


pub(super) fn generate(
    tast_instruction: tast::Instruction
) -> Vec<aast::Instruction> {
    match tast_instruction {
        tast::Instruction::Unary {
            operator: tast_unary_operator,
            src: tast_source,
            dst: tast_destination
        } => {
            match tast_unary_operator {

                tast::UnaryOperator::Complement |
                tast::UnaryOperator::Negate => {
                    let dst: aast::Operand = operand::generate(tast_destination);
                    vec![

                        aast::Instruction::Mov {
                            src: operand::generate(tast_source),
                            dst: dst.clone()
                        },

                        aast::Instruction::Unary {
                            operator: operators::unary::generate(tast_unary_operator),
                            operand: dst
                        }

                    ]
                },

                tast::UnaryOperator::Not => {
                    let zero: aast::Operand = aast::Operand::Imm { int: 0 };
                    let dst: aast::Operand = operand::generate(tast_destination);
                    vec![

                        aast::Instruction::Cmp {
                            loperand: zero.clone(),
                            roperand: operand::generate(tast_source)
                        },

                        aast::Instruction::Mov {
                            src: zero,
                            dst: dst.clone()
                        },

                        aast::Instruction::SetCC {
                            condition: aast::ConditionCode::E,
                            operand: dst
                        }

                    ]
                }
                
            }
        },
        _ => panic!("internal error: called `unary::generate()` with non-unary instruction")
    }
}