use super::cast::Program;
use super::VariableMap;

use super::function;


pub(super) fn traverse(
    program: Program,
    variable_map: &mut VariableMap
) -> Program {
    Program {
        function_definition: function::traverse(
            program.function_definition,
            variable_map
        )
    }
}