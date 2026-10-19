use super::cast::Block;
use super::VariableMap;

use super::block_item;
use super::variable_map;


pub(super) fn traverse(
    block: Block,
    variable_map: &mut VariableMap
) -> Block {
    let mut inner_variable_map: VariableMap = variable_map::copy_for_new_block(variable_map);
    let mut new_block: Block = Block::new();
    for block_item in block {
        new_block.push(
            block_item::traverse(
                block_item,
                &mut inner_variable_map
            )
        );
    }
    new_block
}