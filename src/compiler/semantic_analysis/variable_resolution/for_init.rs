use super::cast::ForInit;
use super::VariableMap;

use super::declaration;
use super::expression;


pub(super) fn traverse(
    for_init: ForInit,
    variable_map: &mut VariableMap
) -> ForInit {
    match for_init {

        ForInit::InitDecl {
            declaration
        } => {
            ForInit::InitDecl {
                declaration: declaration::traverse(
                    declaration,
                    variable_map
                )
            }
        }

        ForInit::InitExp {
            expression
        } => {
            ForInit::InitExp {
                expression: match expression {
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
        
    }
}