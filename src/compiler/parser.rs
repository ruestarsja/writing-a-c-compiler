use super::obj::token::Token;
use super::obj::cast;

// FUNCTIONS

pub(super) fn main(mut tokens: Vec<Token>) -> cast::Program {
    let ast: cast::Program = parse_program(&mut tokens);
    if tokens.len() > 0 {
        panic!("syntax error while parsing: expected EOF but got {:?}", tokens[0]);
    }
    return ast;
}

fn parse_program(tokens: &mut Vec<Token>) -> cast::Program {
    let function_definition: cast::Function = parse_function(tokens);
    return cast::Program { function_definition };
}

fn parse_function(tokens: &mut Vec<Token>) -> cast::Function {
    expect(Token::KInt, tokens);
    let name: cast::Identifier = parse_identifier(tokens);
    expect(Token::OpenParenthesis, tokens);
    expect(Token::KVoid, tokens);
    expect(Token::CloseParenthesis, tokens);
    expect(Token::OpenCurlyBracket, tokens);
    let mut body: Vec<cast::BlockItem> = vec![];
    while tokens[0] != Token::CloseCurlyBracket {
        body.push(parse_block_item(tokens))
    }
    expect(Token::CloseCurlyBracket, tokens);
    return cast::Function { name, body }
}

fn parse_identifier(tokens: &mut Vec<Token>) -> cast::Identifier {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected an identifier but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    match next_token {
        Token::Identifier(id) => {
            tokens.remove(0);
            return cast::Identifier { id: id.to_owned() };
        },
        _ => {
            panic!("syntax error while parsing: expected an identifier but got {:?}", tokens[0]);
        }
    }
}

fn parse_block_item(tokens: &mut Vec<Token>) -> cast::BlockItem {
    match tokens[0] {
        Token::KInt => {
            cast::BlockItem::Declaration {
                declaration: parse_declaration(tokens)
            }
        },
        _ => {
            cast::BlockItem::Statement {
                statement: parse_statement(tokens)
            }
        }
    }
}

fn parse_declaration(tokens: &mut Vec<Token>) -> cast::Declaration {
    expect(Token::KInt, tokens);
    let name: cast::Identifier = parse_identifier(tokens);
    let init: Option<cast::Expression> = match tokens[0] {
        Token::EqualsSign => {
            tokens.remove(0);
            Some(parse_expression(tokens, 0))
        },
        _ => None
    };
    expect(Token::Semicolon, tokens);
    return cast::Declaration { name, init };
}

fn parse_statement(tokens: &mut Vec<Token>) -> cast::Statement {
    match tokens[0] {
        Token::KReturn => {
            tokens.remove(0);
            let exp: cast::Expression = parse_expression(tokens, 0);
            expect(Token::Semicolon, tokens);
            return cast::Statement::Return { exp };
        },
        Token::KIf => {
            tokens.remove(0);
            expect(Token::OpenParenthesis, tokens);
            let condition: cast::Expression = parse_expression(tokens, 0);
            expect(Token::CloseParenthesis, tokens);
            let then_statement: Box<cast::Statement> = Box::new(parse_statement(tokens));
            let else_statement: Option<Box<cast::Statement>> = match tokens[0] {
                Token::KElse => {
                    tokens.remove(0);
                    Some(Box::new(parse_statement(tokens)))
                },
                _ => None
            };
            return cast::Statement::If { condition, then_statement, else_statement };
        }
        Token::Semicolon => {
            tokens.remove(0);
            return cast::Statement::Null;
        },
        _ => {
            let exp: cast::Expression = parse_expression(tokens, 0);
            expect(Token::Semicolon, tokens);
            return cast::Statement::Expression { exp };
        }
    }
}

// precedence climbing
fn parse_expression(tokens: &mut Vec<Token>, min_precedence: u8) -> cast::Expression {
    let mut lfactor: cast::Expression = parse_factor(tokens);
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected an expression but got EOF");
    }
    let mut next_token: Token = tokens[0].clone();
    while get_binary_precedence(&next_token).is_some_and(
        |x: u8| { x >= min_precedence }
    ) {
        match next_token {
            Token::EqualsSign => {
                tokens.remove(0);
                let rfactor: cast::Expression = parse_expression(
                    tokens,
                    get_binary_precedence(&next_token).unwrap()
                );
                lfactor = cast::Expression::Assignment {
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
                let operator: cast::BinaryOperator = get_binary_from_assignment(&next_token).expect("internal error: could not get binary operator from compound assignment token");
                let rfactor: cast::Expression = parse_expression(
                    tokens,
                    get_binary_precedence(&next_token).unwrap()
                );
                lfactor = cast::Expression::Assignment {
                    lvalue: Box::new(lfactor.clone()),
                    exp: Box::new(cast::Expression::Binary {
                        operator,
                        lfactor: Box::new(lfactor),
                        rfactor: Box::new(rfactor)
                    })
                };
            },
            Token::QuestionMark => {
                tokens.remove(0);
                let mfactor: cast::Expression = parse_expression(tokens, 0);
                expect(Token::Colon, tokens);
                let rfactor: cast::Expression = parse_expression(
                    tokens,
                    get_binary_precedence(&next_token).unwrap()
                );
                lfactor = cast::Expression::Conditional {
                    condition: Box::new(lfactor),
                    then_expression: Box::new(mfactor),
                    else_expression: Box::new(rfactor)
                };
            }
            _ => {
                let operator: cast::BinaryOperator = parse_binary_operator(tokens);
                let rfactor: cast::Expression = parse_expression(
                    tokens,
                    get_binary_precedence(&next_token).unwrap() + 1
                );
                lfactor = cast::Expression::Binary {
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

fn parse_factor(tokens: &mut Vec<Token>) -> cast::Expression {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected a factor but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    let mut factor: cast::Expression;
    match next_token {
        Token::Constant(int) => {
            tokens.remove(0);
            factor = cast::Expression::Constant { int };
        },
        Token::Tilde |
        Token::Hyphen |
        Token::ExclamationPoint |
        Token::DoublePlusSign |
        Token::DoubleHyphen => {
            let operator: cast::UnaryOperator = parse_pre_unary_operator(tokens);
            let inner_factor: cast::Expression = parse_factor(tokens);
            factor = cast::Expression::Unary {
                operator,
                factor: Box::new(inner_factor)
            };
        },
        Token::OpenParenthesis => {
            tokens.remove(0);
            let exp: cast::Expression = parse_expression(tokens, 0);
            expect(Token::CloseParenthesis, tokens);
            factor = exp;
        },
        Token::Identifier(id) => {
            tokens.remove(0);
            factor = cast::Expression::Var {
                id: cast::Identifier { id }
            };
        },
        _ => {
            panic!("syntax error while parsing: expected a factor but got {:?}", tokens[0]);
        }
    }
    while match tokens[0] {
        Token::DoublePlusSign |
        Token::DoubleHyphen => {
            let operator: cast::UnaryOperator = parse_post_unary_operator(tokens);
            factor = cast::Expression::Unary {
                operator,
                factor: Box::new(factor)
            };
            true
        },
        _ => false
    } {}
    return factor;
}

fn parse_pre_unary_operator(tokens: &mut Vec<Token>) -> cast::UnaryOperator {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected a unary operator but got EOF");
    }
    match tokens[0] {
        Token::DoublePlusSign => {
            tokens.remove(0);
            return cast::UnaryOperator::PreIncrement;
        },
        Token::DoubleHyphen => {
            tokens.remove(0);
            return cast::UnaryOperator::PreDecrement;
        },
        _ => {
            return parse_unary_operator(tokens);
        }
    }
}

fn parse_post_unary_operator(tokens: &mut Vec<Token>) -> cast::UnaryOperator {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected a unary operator but got EOF");
    }
    match tokens[0] {
        Token::DoublePlusSign => {
            tokens.remove(0);
            return cast::UnaryOperator::PostIncrement;
        },
        Token::DoubleHyphen => {
            tokens.remove(0);
            return cast::UnaryOperator::PostDecrement;
        },
        _ => {
            return parse_unary_operator(tokens);
        }
    }
}

fn parse_unary_operator(tokens: &mut Vec<Token>) -> cast::UnaryOperator {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected a unary operator but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    tokens.remove(0);
    match next_token {
        Token::Tilde => {
            return cast::UnaryOperator::Complement;
        },
        Token::Hyphen => {
            return cast::UnaryOperator::Negate;
        },
        Token::ExclamationPoint => {
            return cast::UnaryOperator::Not;
        },
        Token::DoublePlusSign |
        Token::DoubleHyphen => {
            panic!("internal error: do not use parse_unary_operator where ++ or -- are options (use parse_pre_unary_operator or parse_post_unary_operator instead)");
        },
        _ => {
            panic!("syntax error while parsing: expected a unary operator but got {:?}", tokens[0]);
        }
    }
}

fn parse_binary_operator(tokens: &mut Vec<Token>) -> cast::BinaryOperator {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected a binary operator but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    tokens.remove(0);
    match next_token {
        Token::PlusSign => {
            cast::BinaryOperator::Add
        },
        Token::Hyphen => {
            cast::BinaryOperator::Subtract
        },
        Token::Asterisk => {
            cast::BinaryOperator::Multiply
        },
        Token::ForwardSlash => {
            cast::BinaryOperator::Divide
        },
        Token::PercentSign => {
            cast::BinaryOperator::Remainder
        },
        Token::Ampersand => {
            cast::BinaryOperator::BitwiseAnd
        },
        Token::Pipe => {
            cast::BinaryOperator::BitwiseOr
        },
        Token::Carrot => {
            cast::BinaryOperator::BitwiseXor
        },
        Token::DoubleOpenAngleBracket => {
            cast::BinaryOperator::BitwiseLeftShift
        },
        Token::DoubleCloseAngleBracket => {
            cast::BinaryOperator::BitwiseRightShift
        },
        Token::DoubleAmpersand => {
            cast::BinaryOperator::And
        },
        Token::DoublePipe => {
            cast::BinaryOperator::Or
        },
        Token::DoubleEqualsSign => {
            cast::BinaryOperator::Equal
        },
        Token::ExclamationEquals => {
            cast::BinaryOperator::NotEqual
        },
        Token::OpenAngleBracket => {
            cast::BinaryOperator::LessThan
        },
        Token::OpenAngleBracketEquals => {
            cast::BinaryOperator::LessThanOrEqual
        },
        Token::CloseAngleBracket => {
            cast::BinaryOperator::GreaterThan
        },
        Token::CloseAngleBracketEquals => {
            cast::BinaryOperator::GreaterThanOrEqual
        },
        _ => {
            panic!("syntax error while parsing: expected a binary operator but got {:?}", tokens[0]);
        }
    }
}

fn expect(expected: Token, tokens: &mut Vec<Token>) {
    if tokens.len() == 0 {
        panic!("syntax error while parsing: expected {:?} but got EOF", expected);
    }
    if expected != tokens[0] {
        panic!("syntax error while parsing: expected {:?} but got {:?}", expected, tokens[0]);
    }
    tokens.remove(0);
}

fn get_binary_precedence(operator: &Token) -> Option<u8> {
    match operator {
        Token::Asterisk                      => Some( 50 ), //  *   multiplication
        Token::ForwardSlash                  => Some( 50 ), //  /   division
        Token::PercentSign                   => Some( 50 ), //  %   remainder

        Token::PlusSign                      => Some( 45 ), //  +   addition
        Token::Hyphen                        => Some( 45 ), //  -   subtraction

        Token::DoubleOpenAngleBracket        => Some( 40 ), //  <<  left shift
        Token::DoubleCloseAngleBracket       => Some( 40 ), //  >>  right shift

        Token::OpenAngleBracket              => Some( 35 ), //  <   less than
        Token::OpenAngleBracketEquals        => Some( 35 ), //  <=  less than or equal
        Token::CloseAngleBracket             => Some( 35 ), //  >   greater than
        Token::CloseAngleBracketEquals       => Some( 35 ), //  >=  greater than or equal

        Token::DoubleEqualsSign              => Some( 30 ), //  ==  equal
        Token::ExclamationEquals             => Some( 30 ), //  !=  not equal

        Token::Ampersand                     => Some( 25 ), //  &   bitwise and

        Token::Carrot                        => Some( 20 ), //  ^   bitwise xor

        Token::Pipe                          => Some( 15 ), //  |   bitwise or

        Token::DoubleAmpersand               => Some( 10 ), //  &&  and

        Token::DoublePipe                    => Some( 05 ), //  ||  or

        Token::QuestionMark                  => Some( 03 ), //  ?:  conditional statement

        Token::EqualsSign                    => Some( 01 ), //  =   assignment
        Token::PlusEquals                    => Some( 01 ), //  +=  addition-assignment
        Token::HyphenEquals                  => Some( 01 ), //  -=  subtraction-assignment
        Token::AsteriskEquals                => Some( 01 ), //  *=  multiplication-assignment
        Token::ForwardSlashEquals            => Some( 01 ), //  /=  division-assignment
        Token::PercentEquals                 => Some( 01 ), //  %=  remainder-assignment
        Token::AmpersandEquals               => Some( 01 ), //  &=  bitwise and-assignment
        Token::PipeEquals                    => Some( 01 ), //  |=  bitwise or-assignment
        Token::CarrotEquals                  => Some( 01 ), //  ^=  bitwise xor-assignment
        Token::DoubleOpenAngleBracketEquals  => Some( 01 ), //  <<= left shift-assignment
        Token::DoubleCloseAngleBracketEquals => Some( 01 ), //  >>= right shift-assignment


        _                                    => None
    }
}

fn get_binary_from_assignment(operator: &Token) -> Option<cast::BinaryOperator> {
    match operator {
        Token::PlusEquals                    => Some(cast::BinaryOperator::Add),
        Token::HyphenEquals                  => Some(cast::BinaryOperator::Subtract),
        Token::AsteriskEquals                => Some(cast::BinaryOperator::Multiply),
        Token::ForwardSlashEquals            => Some(cast::BinaryOperator::Divide),
        Token::PercentEquals                 => Some(cast::BinaryOperator::Remainder),
        Token::AmpersandEquals               => Some(cast::BinaryOperator::BitwiseAnd),
        Token::PipeEquals                    => Some(cast::BinaryOperator::BitwiseOr),
        Token::CarrotEquals                  => Some(cast::BinaryOperator::BitwiseXor),
        Token::DoubleOpenAngleBracketEquals  => Some(cast::BinaryOperator::BitwiseLeftShift),
        Token::DoubleCloseAngleBracketEquals => Some(cast::BinaryOperator::BitwiseRightShift),
        _                                    => None
    }
}
