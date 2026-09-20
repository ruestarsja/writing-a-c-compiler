use super::aast::Function;

use super::identifier;
use super::instruction;


pub(super) fn emit(
    aast_function: Function
) -> String {
    let mut function_code: String = String::new();

    let name_code: String = identifier::emit(aast_function.name);

    function_code += &format!("\t.globl {}\n", name_code);
    function_code += &format!("{}:\n", name_code);
    function_code += "\tpushq\t%rbp\n";
    function_code += "\tmovq\t%rsp,\t%rbp\n";
    for instruction in aast_function.instructions {
        function_code += &instruction::emit(instruction);
    }

    return function_code;
}