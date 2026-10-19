use std::collections::HashMap;

use super::id;


#[derive(Clone, Debug)]
pub(super) struct UniqueVariableInfo {
    uid: String,
    is_current_block: bool
}
impl UniqueVariableInfo {
    pub(super) fn new(
        variable_name: &String
    ) -> Self {
        Self {
            uid: id::make_unique(variable_name),
            is_current_block: true
        }
    }

    pub(super) fn get_uid(
        self: &Self
    ) -> &str {
        &self.uid
    }

    pub(super) fn is_current_block(
        self: &Self
    ) -> bool {
        self.is_current_block
    }

    pub(self) fn disable_is_current_block(
        self: &mut Self
    ) {
        self.is_current_block = false;
    }
}


pub(super) type VariableMap = HashMap<String, UniqueVariableInfo>;


pub(super) fn copy_for_new_block(
    variable_map: &mut VariableMap
) -> VariableMap {
    let mut new_variable_map: VariableMap = variable_map.clone();
    for (_, uvi) in &mut new_variable_map {
        uvi.disable_is_current_block();
    }
    new_variable_map
}