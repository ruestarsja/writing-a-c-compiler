use super::aast::{ Instruction, Operand };
use super::PseudoStackCache;

use super::operand;


pub(super) fn traverse(
    aast_instruction: Instruction,
    pseudo_stack_cache: &mut PseudoStackCache
) -> Instruction {
    match aast_instruction {

        Instruction::Mov {
            src: aast_src,
            dst: aast_dst
        } => {
            let src: Operand = operand::traverse(aast_src, pseudo_stack_cache);
            let dst: Operand = operand::traverse(aast_dst, pseudo_stack_cache);
            Instruction::Mov {
                src,
                dst
            }
        },

        Instruction::Unary {
            operator: aast_operator,
            operand: aast_operand
        } => {
            let operand: Operand = operand::traverse(aast_operand, pseudo_stack_cache);
            Instruction::Unary {
                operator: aast_operator,
                operand
            }
        },

        Instruction::Binary {
            operator: aast_operator,
            src: aast_src,
            dst: aast_dst
        } => {
            let src: Operand = operand::traverse(aast_src, pseudo_stack_cache);
            let dst: Operand = operand::traverse(aast_dst, pseudo_stack_cache);
            Instruction::Binary {
                operator: aast_operator,
                src,
                dst
            }
        },

        Instruction::Cmp {
            loperand: aast_loperand,
            roperand: aast_roperand
        } => {
            let loperand: Operand = operand::traverse(aast_loperand, pseudo_stack_cache);
            let roperand: Operand = operand::traverse(aast_roperand, pseudo_stack_cache);
            Instruction::Cmp {
                loperand,
                roperand
            }
        },

        Instruction::Idiv {
            operand: aast_operand
        } => {
            let operand: Operand = operand::traverse(aast_operand, pseudo_stack_cache);
            Instruction::Idiv {
                operand
            }
        },

        Instruction::SetCC {
            condition: aast_condition,
            operand: aast_operand
        } => {
            let operand: Operand = operand::traverse(aast_operand, pseudo_stack_cache);
            Instruction::SetCC {
                condition: aast_condition,
                operand
            }
        },

        Instruction::Cdq |
        Instruction::Jmp { target: _ } |
        Instruction::JmpCC { condition: _, target: _ } |
        Instruction::Label { id: _ } |
        Instruction::AllocateStack { bytes: _ } |
        Instruction::Ret => {
            aast_instruction
        }

    }
}