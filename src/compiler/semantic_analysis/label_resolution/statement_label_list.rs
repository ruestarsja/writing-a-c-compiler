use super::cast::{ Identifier, Label };
use super::LabelMap;

use super::id;


pub(super) fn traverse(
    label_list: Vec<Label>,
    label_map: &mut LabelMap,
    undefined_label_map: &mut LabelMap
) -> Vec<Label> {
    let mut new_label_list: Vec<Label> = vec![];
    for label in label_list {

        if label_map.get(&label.id.id).is_some() {
            panic!("cannot assign the same label to multiple statements");
        }
        let unique_id: String;
        match undefined_label_map.get(&label.id.id) {
            Some(uid) => {
                println!("creating previously undefined label {}", &label.id.id);
                unique_id = uid.to_owned();
                // no longer an undefined label
                undefined_label_map.remove(&label.id.id);
            },
            None => {
                println!("creating new label {}", &label.id.id);
                unique_id = id::make_unique(&label.id.id);
            }
        }
        label_map.insert(label.id.id, unique_id.clone());
        new_label_list.push(
            Label {
                id: Identifier {
                    id: unique_id
                }
            }
        );

    }
    new_label_list
}