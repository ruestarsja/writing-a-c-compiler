use super::cast::Program;

use super::function;


pub(super) fn traverse(
    program: Program,
    current_loop_id: Option<String>
) -> Program {
    Program {
        function_definition: function::traverse(
            program.function_definition,
            current_loop_id
        )
    }
}