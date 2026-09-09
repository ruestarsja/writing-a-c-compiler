use regex::Regex;
use regex::Match;
use std::collections::HashMap;

use super::obj::token::Token;

// FUNCTIONS

pub(super) fn main(
    preprocessed_filename: &str
) -> Vec<Token> {
    let regexes: HashMap<Token, Regex> = HashMap::from([
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
    ]);

    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let mut code: String = std::fs::read_to_string(preprocessed_filename)
        .expect("could not read file");
    let mut tokens: Vec<Token> = vec![];
    
    while code.len() > 0 {
        // https://stackoverflow.com/a/64361042
        if code.chars().nth(0).unwrap().is_whitespace() {
            // https://www.slingacademy.com/article/trimming-and-stripping-rust-strings-for-clean-data-processing/
            code = code.trim_start().to_owned();
        } else {
            let next_token: Token;
            (next_token, code) = find_next_token(&mut code, &regexes);
            // dbg!(&next_token);
            tokens.push(next_token);
        }
    }

    return tokens;
}


fn find_next_token(code: &String, regexes: &HashMap<Token, Regex>) -> (Token, String) {
    let mut match_options: HashMap<Token, Option<Match>> = HashMap::new();
    for (token_type, re) in regexes.iter() {
        match_options.insert(token_type.clone(), re.find(code));
    }

    let mut longest_match_type: Option<&Token> = None;
    let mut longest_match_length: usize = 0;

    for (token_type, match_opt) in match_options.iter() {
        if match_opt.is_some() {
            let match_length = match_opt.unwrap().len();
            if match_length > longest_match_length {
                longest_match_type = Some(token_type);
                longest_match_length = match_length;
            }
        }
    }

    let next_token: Token;

    match longest_match_type.expect("syntax error while lexing") {
        Token::Identifier(_) => {
            let id: &str = match_options[longest_match_type.unwrap()].unwrap().as_str();
            next_token = match id {
                "else"   => Token::KElse,
                "if"     => Token::KIf,
                "int"    => Token::KInt,
                "return" => Token::KReturn,
                "void"   => Token::KVoid,
                id => Token::Identifier(String::from(id)),
            };
        },
        Token::Constant(_) => {
            let num: i32 = match_options[longest_match_type.unwrap()].unwrap().as_str().parse::<i32>().expect("error parsing constant as i32: maybe it's too big?");
            next_token = Token::Constant(num);
        },
        _ => { next_token = longest_match_type.unwrap().clone(); }
    }

    // dbg!(code);
    // dbg!(match_options[longest_match_name].unwrap().as_str());

    return (
        next_token,
        code[longest_match_length..].to_owned()
    )
}
