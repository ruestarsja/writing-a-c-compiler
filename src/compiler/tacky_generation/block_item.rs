use super::cast;
use super::tast;

use super::declaration;
use super::statement;


pub(super) fn generate_instructions(
    cast_block_item: cast::BlockItem
) -> Vec<tast::Instruction> {
    let instructions: Vec<tast::Instruction>;
    match cast_block_item {
        cast::BlockItem::Statement {
            statement: cast_statement
        } => {
            instructions = statement::generate_instructions(
                cast_statement
            );
        },
        cast::BlockItem::Declaration {
            declaration: cast_declaration
        } => {
            instructions = declaration::generate_instructions(
                cast_declaration
            );
        }
    }
    
    instructions
}