use super::aast::{ Function, Program };
use super::PseudoStackCache;

use super::function;


pub(super) fn traverse(
    aast_program: Program,
    pseudo_stack_cache: &mut PseudoStackCache
) -> Program {
    let function_definition: Function = function::traverse(
        aast_program.function_definition,
        pseudo_stack_cache
    );

    Program {
        function_definition
    }
}