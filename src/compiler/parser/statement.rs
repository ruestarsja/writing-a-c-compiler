use super::cast::{
    Expression,
    Label,
    Statement
};
use super::expect;
use super::Token;

use super::expression;
use super::label;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Statement {

    if tokens.is_empty() {
        panic!("syntax error while parsing: expected statement but got EOF");
    }

    let mut labels: Vec<Label> = vec![];
    while tokens.len() > 1 && match (&tokens[0], &tokens[1]) {
        (Token::Identifier(_), Token::Colon) => {
            labels.push(label::parse(tokens));
            expect(Token::Colon, tokens);
            true
        },
        _ => false
    } {}

    match tokens[0] {

        Token::KReturn => {
            tokens.remove(0);
            let exp: Expression = expression::parse(tokens, 0);
            expect(Token::Semicolon, tokens);
            Statement::Return {
                labels,
                exp
            }
        },

        Token::KIf => {
            tokens.remove(0);
            expect(Token::OpenParenthesis, tokens);
            let condition: Expression = expression::parse(tokens, 0);
            expect(Token::CloseParenthesis, tokens);
            let then_statement: Box<Statement> = Box::new(parse(tokens));
            let else_statement: Option<Box<Statement>> = match tokens[0] {
                Token::KElse => {
                    tokens.remove(0);
                    Some(Box::new(parse(tokens)))
                },
                _ => None
            };
            Statement::If {
                labels,
                condition,
                then_statement,
                else_statement
            }
        },

        Token::KGoto => {
            tokens.remove(0);
            let target: Label = label::parse(tokens);
            expect(Token::Semicolon, tokens);
            Statement::Goto {
                labels,
                target
            }
        },

        Token::Semicolon => {
            tokens.remove(0);
            Statement::Null {
                labels
            }
        },

        _ => {
            let exp: Expression = expression::parse(tokens, 0);
            expect(Token::Semicolon, tokens);
            Statement::Expression {
                labels,
                exp
            }
        }

    }
    
}