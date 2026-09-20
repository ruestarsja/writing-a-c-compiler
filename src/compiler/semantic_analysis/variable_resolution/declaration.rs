use super::cast::{ Declaration, Identifier };
use super::VariableMap;

use super::expression;
use super::id;


pub(super) fn traverse(
    declaration: Declaration,
    variable_map: &mut VariableMap
) -> Declaration {
    if variable_map.get(&declaration.name.id).is_some() {
        panic!("cannot declare the same variable twice");
    }
    let unique_id: String = id::make_unique(&declaration.name.id);
    variable_map.insert(declaration.name.id, unique_id.clone());
    Declaration {
        name: Identifier {
            id: unique_id
        },
        init: match declaration.init {
            Some(exp) => Some(
                expression::traverse(
                    exp,
                    variable_map
                )
            ),
            None => None
        }
    }
}