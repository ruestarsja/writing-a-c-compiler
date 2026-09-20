use super::cast::Identifier;
use super::Token;

// FUNCTIONS

pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Identifier {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected an identifier but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    match next_token {
        Token::Identifier(id) => {
            tokens.remove(0);
            return Identifier {
                id: id.to_owned()
            };
        },
        _ => {
            panic!("syntax error while parsing: expected an identifier but got {:?}", tokens[0]);
        }
    }
}