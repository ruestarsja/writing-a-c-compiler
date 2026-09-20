use super::cast::{ BlockItem, Function };
use super::LabelMap;

use super::block_item;


pub(super) fn traverse(
    function: Function,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Function {
    let mut function_body: Vec<BlockItem> = vec![];
    for block_item in function.body {
        function_body.push(
            block_item::traverse(
                block_item,
                label_map,
                undefined_label_map
            )
        );
    }
    Function {
        name: function.name,
        body: function_body
    }
}