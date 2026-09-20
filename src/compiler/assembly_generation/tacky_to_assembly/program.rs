use super::tast;
use super::aast;

use super::function;


pub(super) fn generate(
    tast_program: tast::Program
) -> aast::Program {

    let function_definition: aast::Function = function::generate(
        tast_program.function_definition
    );
    
    aast::Program {
        function_definition
    }
}