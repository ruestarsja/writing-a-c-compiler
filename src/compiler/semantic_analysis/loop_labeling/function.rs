use super::cast::Function;

use super::block;


pub(super) fn traverse(
    function: Function,
    current_loop_id: Option<String>
) -> Function {
    Function {
        name: function.name,
        body: block::traverse(
            function.body,
            current_loop_id
        )
    }
}