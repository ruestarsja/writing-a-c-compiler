mod variable_resolution;
mod label_resolution;
mod loop_labeling;

use super::obj::cast;

// FUNCTIONS

pub(super) fn main(
    c_ast: cast::Program
) -> cast::Program {

    // RESOLVE VARIABLES
    let cast_resolved_variables = variable_resolution::main(c_ast);
    
    // VALIDATE LABELS
    let cast_validated_labels = label_resolution::main(cast_resolved_variables);

    // LABEL LOOPS
    let cast_labeled_loops = loop_labeling::main(cast_validated_labels);

    return cast_labeled_loops;
}