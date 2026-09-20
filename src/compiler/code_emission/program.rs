use super::aast::Program;

use super::function;


pub(super) fn emit(
    aast_program: Program
) -> String {
    let mut program_code: String = String::new();

    program_code += &function::emit(aast_program.function_definition);
    program_code += "\t.section .note.GNU-stack,\"\",@progbits\n";

    return program_code;
}