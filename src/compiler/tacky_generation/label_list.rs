use super::cast;
use super::tast;

use super::label;


pub(super) fn generate_instructions(
    cast_label_list: Vec<cast::Label>
) -> Vec<tast::Instruction> {
    let mut label_list: Vec<tast::Instruction> = vec![];
    for cast_label in cast_label_list {
        label_list.append(
            &mut label::generate_instructions(cast_label)
        );
    }
    label_list
}