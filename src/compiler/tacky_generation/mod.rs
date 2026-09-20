mod program;
mod function;
mod block_item;
mod statement;
mod declaration;
mod label_list;
mod label;
mod expression;
mod identifier;
mod operators;
mod id;


use super::cast;
use super::tast;


pub(super) fn main(
    cast: cast::Program
) -> tast::Program {
    program::generate(cast)
}