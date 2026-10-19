use super::cast::{ Block, Function };
use super::VariableMap;

use super::block;


pub(super) fn traverse(
    function: Function,
    variable_map: &mut VariableMap
) -> Function {
    let function_body: Block = block::traverse(
        function.body,
        variable_map
    );
    Function {
        name: function.name,
        body: function_body
    }
}