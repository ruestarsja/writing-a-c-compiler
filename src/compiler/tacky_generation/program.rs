use super::cast;
use super::tast;

use super::function;


pub(super) fn generate(
    cast_program: cast::Program
) -> tast::Program {
    tast::Program {
        function_definition: function::generate(
            cast_program.function_definition
        )
    }
}