use super::cast::{
    Declaration,
    Expression,
    ForInit
};
use super::Token;
use super::expect;

use super::declaration;
use super::expression;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> ForInit {
    let next_token: &Token = &tokens[0];
    match next_token {
        
        Token::KInt => {
            let declaration: Declaration = declaration::parse(tokens);
            ForInit::InitDecl {
                declaration
            }
        },
        
        Token::Semicolon => {
            tokens.remove(0);
            ForInit::InitExp {
                expression: None
            }
        },

        _ => {
            let expression: Expression = expression::parse(tokens, 0);
            expect(Token::Semicolon, tokens);
            ForInit::InitExp {
                expression: Some(expression)
            }
        }
        
    }
}