mod program;
mod function;
mod instruction;

use super::aast;


pub(super) fn main(
    assembly_ast: aast::Program,
    allocation_size: u32
) -> aast::Program {
    program::traverse(assembly_ast, allocation_size)
}