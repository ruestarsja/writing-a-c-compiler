mod program;
mod function;
mod block;
mod block_item;
mod statement;
mod id;


use super::cast;


pub(super) fn main(
    c_ast: cast::Program
) -> cast::Program {
    let program: cast::Program = program::traverse(
        c_ast,
        None
    );
    return program;
}