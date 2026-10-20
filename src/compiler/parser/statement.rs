use super::cast::{
    Expression,
    ForInit,
    Label,
    Statement
};
use super::expect;
use super::Token;

use super::block;
use super::expression;
use super::for_init;
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

        Token::OpenCurlyBracket => {
            let block = block::parse(tokens);
            Statement::Compound {
                labels,
                block
            }
        },

        Token::KBreak => {
            tokens.remove(0);
            expect(Token::Semicolon, tokens);
            Statement::Break {
                labels,
                loop_id: None
            }
        },

        Token::KContinue => {
            tokens.remove(0);
            expect(Token::Semicolon, tokens);
            Statement::Continue {
                labels,
                loop_id: None
            }
        },

        Token::KWhile => {
            tokens.remove(0);
            expect(Token::OpenParenthesis, tokens);
            let condition: Expression = expression::parse(tokens, 0);
            expect(Token::CloseParenthesis, tokens);
            let body: Statement = parse(tokens);
            Statement::While {
                labels,
                condition,
                body: Box::new(body),
                loop_id: None
            }
        },

        Token::KDo => {
            tokens.remove(0);
            let body: Statement = parse(tokens);
            expect(Token::KWhile, tokens);
            expect(Token::OpenParenthesis, tokens);
            let condition: Expression = expression::parse(tokens, 0);
            expect(Token::CloseParenthesis, tokens);
            expect(Token::Semicolon, tokens);
            Statement::DoWhile {
                labels,
                body: Box::new(body),
                condition,
                loop_id: None
            }
        },

        Token::KFor => {
            tokens.remove(0);
            expect(Token::OpenParenthesis, tokens);
            let init: ForInit = for_init::parse(tokens);
            let condition: Option<Expression> = match tokens[0] {
                Token::Semicolon => None,
                _ => Some(expression::parse(tokens, 0))
            };
            expect(Token::Semicolon, tokens);
            let post: Option<Expression> = match tokens[0] {
                Token::CloseParenthesis => None,
                _ => Some(expression::parse(tokens, 0))
            };
            expect(Token::CloseParenthesis, tokens);
            let body: Statement = parse(tokens);
            Statement::For {
                labels,
                init,
                condition,
                post,
                body: Box::new(body),
                loop_id: None
            }
        }

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