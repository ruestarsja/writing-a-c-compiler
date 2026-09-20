use super::cast::{
    BlockItem
};
use super::Token;

use super::declaration;
use super::statement;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> BlockItem {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected a block item but got EOF");
    }
    match tokens[0] {
        Token::KInt => {
            BlockItem::Declaration {
                declaration: declaration::parse(tokens)
            }
        },
        _ => {
            BlockItem::Statement {
                statement: statement::parse(tokens)
            }
        }
    }
}