use super::cast::{ BlockItem, Function };
use super::VariableMap;

use super::block_item;


pub(super) fn traverse(
    function: Function,
    variable_map: &mut VariableMap
) -> Function {
    let mut function_body: Vec<BlockItem> = vec![];
    for block_item in function.body {
        function_body.push(
            block_item::traverse(
                block_item,
                variable_map
            )
        );
    }
    Function {
        name: function.name,
        body: function_body
    }
}