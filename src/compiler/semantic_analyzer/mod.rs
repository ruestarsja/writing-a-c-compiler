mod variable_resolver;

use super::obj::cast;

// FUNCTIONS

pub(super) fn main(
    ast: cast::Program
) -> cast::Program {

    // RESOLVE VARIABLES
    let ast_resolved_variables = variable_resolver::main(ast);

    return ast_resolved_variables;
}