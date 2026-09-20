use super::cast::BlockItem;
use super::LabelMap;

use super::statement;


pub(super) fn traverse(
    block_item: BlockItem,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> BlockItem {
    match block_item {
        BlockItem::Statement {
            statement
        } => {
            BlockItem::Statement {
                statement: statement::traverse(
                    statement,
                    label_map,
                    undefined_label_map
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