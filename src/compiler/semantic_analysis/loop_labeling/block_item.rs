use super::cast::BlockItem;

use super::statement;


pub(super) fn traverse(
    block_item: BlockItem,
    current_loop_id: Option<String>
) -> BlockItem {
    match block_item {

        BlockItem::Statement {
            statement
        } => {
            BlockItem::Statement {
                statement: statement::traverse(
                    statement,
                    current_loop_id
                )
            }
        },

        BlockItem::Declaration {
            declaration
        } => {
            BlockItem::Declaration {
                declaration
            }
        }

    }
}