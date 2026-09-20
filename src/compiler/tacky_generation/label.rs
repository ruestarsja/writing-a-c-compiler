use super::cast;
use super::tast;

use super::identifier;


pub(super) fn generate_instructions(
    cast_label: cast::Label
) -> Vec<tast::Instruction> {
    vec![
        tast::Instruction::Label {
            id: identifier::generate(cast_label.id)
        }
    ]
}