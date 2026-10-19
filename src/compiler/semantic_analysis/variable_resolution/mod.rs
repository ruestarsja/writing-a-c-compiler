mod program;
mod function;
mod block;
mod block_item;
mod statement;
mod declaration;
mod expression;
mod id;
mod variable_map;

use super::cast;

use variable_map::VariableMap;


pub(super) fn main(
    c_ast: cast::Program
) -> cast::Program {
    let mut variable_map: VariableMap = VariableMap::new();
    let program: cast::Program = program::traverse(c_ast, &mut variable_map);
    return program;
}
