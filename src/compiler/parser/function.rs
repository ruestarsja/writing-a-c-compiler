use super::cast::{
    BlockItem,
    Function,
    Identifier
};
use super::expect;
use super::Token;

use super::identifier;
use super::block_item;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Function {
    expect(Token::KInt, tokens);
    let name: Identifier = identifier::parse(tokens);
    expect(Token::OpenParenthesis, tokens);
    expect(Token::KVoid, tokens);
    expect(Token::CloseParenthesis, tokens);
    expect(Token::OpenCurlyBracket, tokens);
    let mut body: Vec<BlockItem> = vec![];
    while tokens[0] != Token::CloseCurlyBracket {
        body.push(block_item::parse(tokens));
    }
    expect(Token::CloseCurlyBracket, tokens);
    
    Function {
        name,
        body
    }
}