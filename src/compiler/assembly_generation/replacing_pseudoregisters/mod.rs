mod pseudostackcache;
mod program;
mod function;
mod instruction;
mod operand;
mod stack;

use super::aast;

use pseudostackcache::PseudoStackCache;


pub(super) fn main(
    assembly_ast: aast::Program
) -> (aast::Program, u32) {
    let mut pseudo_stack_cache: PseudoStackCache = PseudoStackCache::new();
    let program: aast::Program = program::traverse(assembly_ast, &mut pseudo_stack_cache);
    let total_offset: u32 = stack::get_total_offset();
    
    (program, total_offset)
}