use super::cast::BlockItem;
use super::VariableMap;

use super::statement;
use super::declaration;


pub(super) fn traverse(
    block_item: BlockItem,
    variable_map: &mut VariableMap
) -> BlockItem {
    match block_item {
        BlockItem::Statement {
            statement
        } => {
            BlockItem::Statement {
                statement: statement::traverse(
                    statement,
                    variable_map
                )
            }
        },
        BlockItem::Declaration {
            declaration
        } => {
            BlockItem::Declaration {
                declaration: declaration::traverse(
                    declaration,
                    variable_map
                )
            }
        }
    }
}