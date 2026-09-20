use super::cast::{ Identifier, Label };
use super::LabelMap;

use super::id;


pub(super) fn traverse(
    target_label: Label,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Label {
    let unique_id: String;
    match label_map.get(&target_label.id.id) {
        Some(uid) => {
            unique_id = uid.to_owned();
        },
        None => match undefined_label_map.get(&target_label.id.id) {
            Some(uid) => {
                unique_id = uid.to_owned();
            },
            None => {
                println!("goto {} - label not yet defined.", &target_label.id.id);
                unique_id = id::make_unique(&target_label.id.id);
                undefined_label_map.insert(
                    target_label.id.id,
                    unique_id.clone()
                );
            }
        }
    }
    Label {
        id: Identifier {
            id: unique_id
        }
    }
}