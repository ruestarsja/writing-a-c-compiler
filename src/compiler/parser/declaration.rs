use super::cast::{
    Declaration,
    Expression,
    Identifier
};
use super::Token;
use super::expect;

use super::expression;
use super::identifier;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Declaration {
    expect(Token::KInt, tokens);
    let name: Identifier = identifier::parse(tokens);
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected expression or semicolon but got EOF");
    }
    let init: Option<Expression> = match tokens[0] {
        Token::EqualsSign => {
            tokens.remove(0);
            Some(expression::parse(tokens, 0))
        },
        _ => None
    };
    expect(Token::Semicolon, tokens);
    Declaration {
        name,
        init
    }
}