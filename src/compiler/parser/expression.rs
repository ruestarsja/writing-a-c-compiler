use super::cast::{
    BinaryOperator,
    Expression,
    Identifier,
    UnaryOperator
};
use super::Token;
use super::expect;

use super::operators;


pub(crate) fn parse(
    tokens: &mut Vec<Token>,
    min_precedence: u8
) -> Expression {
    let mut lfactor: Expression = parse_factor(tokens);
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected an expression but got EOF");
    }
    let mut next_token: Token = tokens[0].clone();
    while operators::binary::get_precedence(&next_token).is_some_and(
        |x: u8| { x >= min_precedence }
    ) {
        match next_token {
            Token::EqualsSign => {
                tokens.remove(0);
                let rfactor: Expression = parse(
                    tokens,
                    operators::binary::get_precedence(&next_token).unwrap()
                );
                lfactor = Expression::Assignment {
                    lvalue: Box::new(lfactor),
                    exp: Box::new(rfactor)
                };
            },
            Token::PlusEquals |
            Token::HyphenEquals |
            Token::AsteriskEquals |
            Token::ForwardSlashEquals |
            Token::PercentEquals |
            Token::AmpersandEquals |
            Token::PipeEquals |
            Token::CarrotEquals |
            Token::DoubleOpenAngleBracketEquals |
            Token::DoubleCloseAngleBracketEquals => {
                tokens.remove(0);
                let operator: BinaryOperator = operators::binary::get_from_compound_assignment(&next_token)
                    .expect("internal error: could not get binary operator from compound assignment token");
                let rfactor: Expression = parse(
                    tokens,
                    operators::binary::get_precedence(&next_token).unwrap()
                );
                lfactor = Expression::Assignment {
                    lvalue: Box::new(lfactor.clone()),
                    exp: Box::new(Expression::Binary {
                        operator,
                        lfactor: Box::new(lfactor),
                        rfactor: Box::new(rfactor)
                    })
                };
            },
            Token::QuestionMark => {
                tokens.remove(0);
                let mfactor: Expression = parse(tokens, 0);
                expect(Token::Colon, tokens);
                let rfactor: Expression = parse(
                    tokens,
                    operators::binary::get_precedence(&next_token).unwrap()
                );
                lfactor = Expression::Conditional {
                    condition: Box::new(lfactor),
                    exp_if_true: Box::new(mfactor),
                    exp_if_false: Box::new(rfactor)
                };
            },
            _ => {
                let operator: BinaryOperator = operators::binary::parse(tokens);
                let rfactor: Expression = parse(
                    tokens,
                    operators::binary::get_precedence(&next_token).unwrap() + 1
                );
                lfactor = Expression::Binary {
                    operator,
                    lfactor: Box::new(lfactor),
                    rfactor: Box::new(rfactor)
                };
            }
        }
        next_token = tokens[0].clone();
    }
    return lfactor;
}


fn parse_factor(
    tokens: &mut Vec<Token>
) -> Expression {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected a factor but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    let mut factor: Expression = match next_token {
        Token::Constant(int) => {
            tokens.remove(0);
            Expression::Constant {
                int
            }
        },
        Token::Tilde |
        Token::Hyphen |
        Token::ExclamationPoint |
        Token::DoublePlusSign |
        Token::DoubleHyphen => {
            let operator: UnaryOperator = operators::unary::parse_pre(tokens);
            let inner_factor: Expression = parse_factor(tokens);
            Expression::Unary {
                operator,
                factor: Box::new(inner_factor)
            }
        },
        Token::OpenParenthesis => {
            tokens.remove(0);
            let exp: Expression = parse(tokens, 0);
            expect(Token::CloseParenthesis, tokens);
            exp
        },
        Token::Identifier(id) => {
            tokens.remove(0);
            Expression::Var {
                id: Identifier {
                    id
                }
            }
        },
        _ => {
            panic!("syntax error while parsing: expected a factor but got {:?}", next_token);
        }
    };
    // match any postfix ++ or --
    while match tokens[0] {
        Token::DoublePlusSign |
        Token::DoubleHyphen => {
            let operator: UnaryOperator = operators::unary::parse_post(tokens);
            factor = Expression::Unary {
                operator,
                factor: Box::new(factor)
            };
            true
        },
        _ => false
    } {}
    return factor;
}