use super::cast;
use super::tast;

use super::block_item;


pub(super) fn generate_instructions(
    cast_block: cast::Block
) -> Vec<tast::Instruction> {
    let mut instructions: Vec<tast::Instruction> = vec![];
    for block_item in cast_block {
        instructions.append(
            &mut block_item::generate_instructions(
                block_item
            )
        );
    }
    instructions
}