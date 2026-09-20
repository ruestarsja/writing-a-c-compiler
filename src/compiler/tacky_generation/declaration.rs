use super::cast;
use super::tast;

use super::expression;
use super::identifier;


pub(super) fn generate_instructions (
    cast_declaration: cast::Declaration
) -> Vec<tast::Instruction> {
    match cast_declaration.init {
        Some(exp) => {
            let mut instructions: Vec<tast::Instruction>;
            let val: tast::Value;
            (instructions, val) = expression::generate_instructions_and_value(exp);
            instructions.push(
                tast::Instruction::Copy {
                    src: val,
                    dst: tast::Value::Variable {
                        name: identifier::generate(cast_declaration.name)
                    }
                }
            );
            instructions
        },
        None => vec![]
    }
}