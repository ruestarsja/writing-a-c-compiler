mod program;
mod function;
mod block;
mod block_item;
mod statement;
mod statement_label_list;
mod target_label;
mod id;
mod label_map;

use super::cast;

use label_map::LabelMap;


pub(super) fn main(
    c_ast: cast::Program
) -> cast::Program {
    let mut label_map: LabelMap = LabelMap::new();
    let mut undefined_label_map: LabelMap = LabelMap::new();
    let program: cast::Program = program::traverse(
        c_ast,
        &mut label_map,
        &mut undefined_label_map
    );
    if !(undefined_label_map.is_empty()) {
        panic!("use of undefined label(s) `{:?}`", undefined_label_map.keys());
    }
    return program;
}