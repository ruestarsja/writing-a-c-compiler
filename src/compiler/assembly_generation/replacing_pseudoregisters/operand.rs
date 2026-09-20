use super::aast::Operand;
use super::PseudoStackCache;

use super::stack;


pub(super) fn traverse(
    aast_operand: Operand,
    pseudo_stack_cashe: &mut PseudoStackCache
) -> Operand {
    match aast_operand {
        
        Operand::Pseudo {
            name
        } => {
            let cache_opt: Option<&i32> = pseudo_stack_cashe.get(&name.id);
            match cache_opt {
                Some(offset) => {
                    Operand::Stack {
                        offset: offset.clone()
                    }
                },
                None => {
                    let offset: i32 = stack::get_new_offset();
                    pseudo_stack_cashe.insert(name.id, offset);
                    Operand::Stack {
                        offset
                    }
                }
            }
        }
        
        Operand::Imm { int: _ } |
        Operand::Reg { reg: _ } |
        Operand::Stack { offset: _ } => {
            aast_operand
        },

    }
}