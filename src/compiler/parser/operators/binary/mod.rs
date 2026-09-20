use super::super::cast::BinaryOperator;
use super::super::Token;


pub(crate) fn parse(
    tokens: &mut Vec<Token>
) -> BinaryOperator {
    if tokens.is_empty() {
        panic!("syntax error while parsing: expected a binary operator but got EOF");
    }
    let next_token: Token = tokens[0].clone();
    tokens.remove(0);
    match next_token {
        Token::PlusSign => BinaryOperator::Add,
        Token::Hyphen => BinaryOperator::Subtract,
        Token::Asterisk => BinaryOperator::Multiply,
        Token::ForwardSlash => BinaryOperator::Divide,
        Token::PercentSign => BinaryOperator::Remainder,
        Token::Ampersand => BinaryOperator::BitwiseAnd,
        Token::Pipe => BinaryOperator::BitwiseOr,
        Token::Carrot => BinaryOperator::BitwiseXor,
        Token::DoubleOpenAngleBracket => BinaryOperator::BitwiseLeftShift,
        Token::DoubleCloseAngleBracket => BinaryOperator::BitwiseRightShift,
        Token::DoubleAmpersand => BinaryOperator::And,
        Token::DoublePipe => BinaryOperator::Or,
        Token::DoubleEqualsSign => BinaryOperator::Equal,
        Token::ExclamationEquals => BinaryOperator::NotEqual,
        Token::OpenAngleBracket => BinaryOperator::LessThan,
        Token::OpenAngleBracketEquals => BinaryOperator::LessThanOrEqual,
        Token::CloseAngleBracket => BinaryOperator::GreaterThan,
        Token::CloseAngleBracketEquals => BinaryOperator::GreaterThanOrEqual,
        _ => {
            panic!("syntax error while parsing: expected a binary operator but got {:?}", next_token);
        }
    }
}

pub(crate) fn get_from_compound_assignment(
    operator: &Token
) -> Option<BinaryOperator> {
    match operator {
        Token::PlusEquals                    => Some(BinaryOperator::Add),
        Token::HyphenEquals                  => Some(BinaryOperator::Subtract),
        Token::AsteriskEquals                => Some(BinaryOperator::Multiply),
        Token::ForwardSlashEquals            => Some(BinaryOperator::Divide),
        Token::PercentEquals                 => Some(BinaryOperator::Remainder),
        Token::AmpersandEquals               => Some(BinaryOperator::BitwiseAnd),
        Token::PipeEquals                    => Some(BinaryOperator::BitwiseOr),
        Token::CarrotEquals                  => Some(BinaryOperator::BitwiseXor),
        Token::DoubleOpenAngleBracketEquals  => Some(BinaryOperator::BitwiseLeftShift),
        Token::DoubleCloseAngleBracketEquals => Some(BinaryOperator::BitwiseRightShift),
        _                                    => None
    }
}

pub(crate) fn get_precedence(
    operator: &Token
) -> Option<u8> {
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