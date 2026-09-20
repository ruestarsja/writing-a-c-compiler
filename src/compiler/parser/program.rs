use super::cast::{ Function, Program };
use super::Token;

use super::function;


pub(super) fn parse(
    tokens: &mut Vec<Token>
) -> Program {
    let function_definition: Function = function::parse(tokens);
    Program {
        function_definition
    }
}
