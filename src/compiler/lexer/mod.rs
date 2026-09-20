mod regex_table;
mod token_getter;

use regex::Regex;
use std::collections::HashMap;

use super::obj::token::Token;

// FUNCTIONS

pub(super) fn main(
    preprocessed_filename: &str
) -> Vec<Token> {
    let regexes: HashMap<Token, Regex> = regex_table::get();
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
            (next_token, code) = token_getter::next(&mut code, &regexes);
            // dbg!(&next_token);
            tokens.push(next_token);
        }
    }

    return tokens;
}