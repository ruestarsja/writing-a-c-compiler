use super::cast::Block;
use super::LabelMap;

use super::block_item;


pub(super) fn traverse(
    block: Block,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Block {
    let mut new_block: Block = Block::new();
    for block_item in block {
        new_block.push(
            block_item::traverse(
                block_item,
                label_map,
                undefined_label_map
            )
        );
    }
    new_block
}