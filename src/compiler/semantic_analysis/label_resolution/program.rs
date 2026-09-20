use super::cast::Program;
use super::LabelMap;

use super::function;


pub(super) fn traverse(
    program: Program,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Program {
    Program {
        function_definition: function::traverse(
            program.function_definition,
            label_map,
            undefined_label_map
        )
    }
}