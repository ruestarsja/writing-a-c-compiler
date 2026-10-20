use super::cast::Block;

use super::block_item;


pub(super) fn traverse(
    block: Block,
    current_loop_id: Option<String>
) -> Block {
    let mut new_block: Block = Block::new();
    for block_item in block {
        new_block.push(
            block_item::traverse(
                block_item,
                current_loop_id.clone()
            )
        );
    }
    new_block
}