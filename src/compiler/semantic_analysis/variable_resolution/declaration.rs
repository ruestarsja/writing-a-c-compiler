use super::cast::{ Declaration, Identifier };
use super::VariableMap;
use super::variable_map::UniqueVariableInfo;

use super::expression;


pub(super) fn traverse(
    declaration: Declaration,
    variable_map: &mut VariableMap
) -> Declaration {
    if variable_map.get(&declaration.name.id).is_some_and(
        |uvi| -> bool { uvi.is_current_block() }
    ) {
        panic!("cannot declare the same variable twice in the same block");
    }
    let uvi: UniqueVariableInfo = UniqueVariableInfo::new(&declaration.name.id);
    variable_map.insert(
        declaration.name.id,
        uvi.clone()
    );
    Declaration {
        name: Identifier {
            id: uvi.get_uid().to_owned()
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