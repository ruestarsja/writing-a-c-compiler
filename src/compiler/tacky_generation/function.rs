use super::cast;
use super::tast;

use super::identifier;
use super::block_item;


pub(super) fn generate(
    cast_function: cast::Function
) -> tast::Function {
    let mut function_body: Vec<tast::Instruction> = vec![];
    for block_item in cast_function.body {
        function_body.append(
            &mut block_item::generate_instructions(
                block_item
            )
        )
    }
    function_body.push(
        tast::Instruction::Return {
            val: tast::Value::Constant {
                int: 0
            }
        }
    );
    tast::Function {
        name: identifier::generate(
            cast_function.name
        ),
        body: function_body
    }
}