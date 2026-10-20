mod program;
mod function;
mod identifier;
mod block;
mod block_item;
mod label;
mod declaration;
mod for_init;
mod statement;
mod expression;
mod operators;
mod expect;

use super::Token;
use super::cast;

use expect::expect;


pub(super) fn main(mut tokens: Vec<Token>) -> cast::Program {
    let ast: cast::Program = program::parse(&mut tokens);
    if !tokens.is_empty() {
        panic!("syntax error while parsing: expected EOF but got {:?}", tokens[0]);
    }
    return ast;
}