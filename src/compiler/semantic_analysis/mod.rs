mod variable_resolution;
mod label_resolution;

use super::obj::cast;

// FUNCTIONS

pub(super) fn main(
    c_ast: cast::Program
) -> cast::Program {

    // RESOLVE VARIABLES
    let cast_resolved_variables = variable_resolution::main(c_ast);
    
    // VALIDATE LABELS
    let cast_validated_labels = label_resolution::main(cast_resolved_variables);

    return cast_validated_labels;
}