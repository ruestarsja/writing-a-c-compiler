use super::HashMap;
use super::Regex;

use regex::Match;

use super::Token;

// FUNCTIONS

pub(super) fn next(
    code: &String,
    regexes: &HashMap<Token, Regex>
) -> (Token, String) {
    let token_type: Token;
    let token_text: &str;
    (token_type, token_text) = get_next_match(code, regexes);
    let next_token: Token = match token_type {
        Token::Identifier(_) => {
            match token_text {
                "break"    => Token::KBreak,
                "continue" => Token::KContinue,
                "do"       => Token::KDo,
                "else"     => Token::KElse,
                "for"      => Token::KFor,
                "goto"     => Token::KGoto,
                "if"       => Token::KIf,
                "int"      => Token::KInt,
                "return"   => Token::KReturn,
                "void"     => Token::KVoid,
                "while"    => Token::KWhile,
                _          => Token::Identifier(token_text.to_owned())
            }
        },
        Token::Constant(_) => {
            let num: i32 = token_text.parse::<i32>()
                .expect("error parsing constant as i32; maybe it's too big?");
            Token::Constant(num)
        },
        _ => { token_type }
    };

    return (
        next_token,
        code[token_text.len()..].to_owned()
    );
}


fn get_next_match<'a>(
    code: &'a String,
    regexes: &HashMap<Token, Regex>
) -> (Token, &'a str) {
    let mut match_options: HashMap<Token, Option<Match>> = HashMap::new();
    for (token_type, re) in regexes.iter() {
        match_options.insert(token_type.clone(), re.find(code));
    }

    let mut longest_match_token_type: Option<&Token> = None;
    let mut longest_match_length: usize = 0;
    for (
        token_type,
        match_opt
    ) in match_options.iter() {
        if match_opt.is_some() {
            let match_length = match_opt.unwrap().len();
            if match_length > longest_match_length {
                longest_match_token_type = Some(token_type);
                longest_match_length = match_length;
            }
        }
    }

    let token_type: Token = longest_match_token_type
        .expect("syntax error while lexing")
        .to_owned();
    let token_text: &str = match_options[&token_type]
        .unwrap()
        .as_str();

    return (token_type, token_text);
}