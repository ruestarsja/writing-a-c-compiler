use super::cast;
use super::tast;

use super::declaration;
use super::expression;


pub(super) fn generate_instructions(
    cast_init: cast::ForInit
) -> Vec<tast::Instruction> {
    let instructions: Vec<tast::Instruction>;
    match cast_init {
        cast::ForInit::InitDecl {
            declaration: cast_declaration
        } => {
            instructions = declaration::generate_instructions(
                cast_declaration
            );
        },
        cast::ForInit::InitExp {
            expression: cast_expression
        } => {
            instructions = match cast_expression {
                Some(exp) => expression::generate_instructions(
                    exp
                ),
                None => vec![]
            };
        }
    }

    instructions
}