use super::cast::{
    Block,
    Function,
    Identifier
};
use super::expect;
use super::Token;

use super::identifier;
use super::block;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Function {
    expect(Token::KInt, tokens);
    let name: Identifier = identifier::parse(tokens);
    expect(Token::OpenParenthesis, tokens);
    expect(Token::KVoid, tokens);
    expect(Token::CloseParenthesis, tokens);
    let body: Block = block::parse(tokens);
    
    Function {
        name,
        body
    }
}