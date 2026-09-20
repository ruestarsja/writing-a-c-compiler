use super::super::cast::UnaryOperator;
use super::super::Token;


pub(crate) fn parse_pre(
    tokens: &mut Vec<Token>
) -> UnaryOperator {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected a unary operator but got EOF");
    }
    let next_token: Token = tokens.remove(0);
    match next_token {
        Token::DoublePlusSign => {
            UnaryOperator::PreIncrement
        },
        Token::DoubleHyphen => {
            UnaryOperator::PreDecrement
        },
        Token::Tilde => {
            UnaryOperator::Complement
        },
        Token::Hyphen => {
            UnaryOperator::Negate
        },
        Token::ExclamationPoint => {
            UnaryOperator::Not
        },
        _ => {
            panic!("syntax error while parsing: expected a prefix unary operator but got {:?}", tokens[0]);
        }
    }
}


pub(crate) fn parse_post(
    tokens: &mut Vec<Token>
) -> UnaryOperator {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected a unary operator but got EOF");
    }
    match tokens[0] {
        Token::DoublePlusSign => {
            tokens.remove(0);
            UnaryOperator::PostIncrement
        },
        Token::DoubleHyphen => {
            tokens.remove(0);
            UnaryOperator::PostDecrement
        },
        _ => {
            panic!("syntax error while parsing: expected a postfix unary operator but got {:?}", tokens[0]);
        }
    }
}