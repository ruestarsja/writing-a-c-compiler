use super::aast::{ Function, Program };

use super::function;


pub(super) fn traverse(
    aast_program: Program,
    allocation_size: u32
) -> Program {
    let function_definition: Function = function::traverse(
        aast_program.function_definition,
        allocation_size
    );
    
    Program {
        function_definition
    }
}