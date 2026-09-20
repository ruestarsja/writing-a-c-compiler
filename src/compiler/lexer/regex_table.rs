use super::HashMap;
use super::Regex;

use super::Token;

pub(super) fn get() -> HashMap<Token, Regex> {
    HashMap::from([
        ( Token::Identifier(String::new()), Regex::new(r"^[a-zA-Z_]\w*\b").unwrap() ),
        ( Token::Constant(0),               Regex::new(r"^[0-9]+\b").unwrap()       ),
    
        ( Token::Ampersand,         Regex::new(r"^\&").unwrap() ),
        ( Token::Asterisk,          Regex::new(r"^\*").unwrap() ),
        ( Token::Carrot,            Regex::new(r"^\^").unwrap() ),
        ( Token::CloseAngleBracket, Regex::new(r"^>").unwrap()  ),
        ( Token::CloseCurlyBracket, Regex::new(r"^\}").unwrap() ),
        ( Token::CloseParenthesis,  Regex::new(r"^\)").unwrap() ),
        ( Token::Colon,             Regex::new(r"^:").unwrap()  ),
        ( Token::EqualsSign,        Regex::new(r"^=").unwrap()  ),
        ( Token::ExclamationPoint,  Regex::new(r"^!").unwrap()  ),
        ( Token::ForwardSlash,      Regex::new(r"^\/").unwrap() ),
        ( Token::Hyphen,            Regex::new(r"^-").unwrap()  ),
        ( Token::OpenAngleBracket,  Regex::new(r"^<").unwrap()  ),
        ( Token::OpenCurlyBracket,  Regex::new(r"^\{").unwrap() ),
        ( Token::OpenParenthesis,   Regex::new(r"^\(").unwrap() ),
        ( Token::PercentSign,       Regex::new(r"^%").unwrap()  ),
        ( Token::Pipe,              Regex::new(r"^\|").unwrap() ),
        ( Token::PlusSign,          Regex::new(r"^\+").unwrap() ),
        ( Token::QuestionMark,      Regex::new(r"^\?").unwrap() ),
        ( Token::Semicolon,         Regex::new(r"^;").unwrap()  ),
        ( Token::Tilde,             Regex::new(r"^~").unwrap()  ),
        // ( Token::OpenSquareBracket,  Regex::new(r"^\[").unwrap() ),
        // ( Token::CloseSquareBracket, Regex::new(r"^\]").unwrap() ),
    
        ( Token::AmpersandEquals,               Regex::new(r"^\&=").unwrap()  ),
        ( Token::AsteriskEquals,                Regex::new(r"^\*=").unwrap()  ),
        ( Token::CarrotEquals,                  Regex::new(r"^\^=").unwrap()  ),
        ( Token::CloseAngleBracketEquals,       Regex::new(r"^>=").unwrap()   ),
        ( Token::DoubleAmpersand,               Regex::new(r"^\&\&").unwrap() ),
        ( Token::DoubleCloseAngleBracket,       Regex::new(r"^>>").unwrap()   ),
        ( Token::DoubleCloseAngleBracketEquals, Regex::new(r"^>>=").unwrap()  ),
        ( Token::DoubleEqualsSign,              Regex::new(r"^==").unwrap()   ),
        ( Token::DoubleHyphen,                  Regex::new(r"^--").unwrap()   ),
        ( Token::DoubleOpenAngleBracket,        Regex::new(r"^<<").unwrap()   ),
        ( Token::DoubleOpenAngleBracketEquals,  Regex::new(r"^<<=").unwrap()  ),
        ( Token::DoublePipe,                    Regex::new(r"^\|\|").unwrap() ),
        ( Token::DoublePlusSign,                Regex::new(r"^\+\+").unwrap() ),
        ( Token::ExclamationEquals,             Regex::new(r"^!=").unwrap()   ),
        ( Token::ForwardSlashEquals,            Regex::new(r"^\/=").unwrap()  ),
        ( Token::HyphenEquals,                  Regex::new(r"^-=").unwrap()   ),
        ( Token::OpenAngleBracketEquals,        Regex::new(r"^<=").unwrap()   ),
        ( Token::PercentEquals,                 Regex::new(r"^%=").unwrap()   ),
        ( Token::PipeEquals,                    Regex::new(r"^\|=").unwrap()  ),
        ( Token::PlusEquals,                    Regex::new(r"^\+=").unwrap()  ),
    ])
}