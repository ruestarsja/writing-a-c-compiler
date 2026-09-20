use super::tast;
use super::aast;

use super::identifier;
use super::instructions;


pub(super) fn generate(
    tast_function: tast::Function
) -> aast::Function {
    
    let name: aast::Identifier = identifier::generate(
        tast_function.name
    );

    let mut instructions: Vec<aast::Instruction> = vec![];
    for tast_instruction in tast_function.body {
        instructions.append(
            &mut instructions::generate(
                tast_instruction
            )
        );
    }

    aast::Function {
        name,
        instructions
    }
}