use super::cast::{ Block, Function };
use super::LabelMap;

use super::block;


pub(super) fn traverse(
    function: Function,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Function {
    let function_body: Block = block::traverse(
        function.body,
        label_map,
        undefined_label_map
    );
    Function {
        name: function.name,
        body: function_body
    }
}