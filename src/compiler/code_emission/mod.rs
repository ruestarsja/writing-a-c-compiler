mod program;
mod function;
mod identifier;
mod instruction;
mod operators;
mod operand;
mod conditioncode;

use super::aast;


pub(super) fn main(
    filename: &str,
    assembly_ast: aast::Program
) {
    let mut assembly_code: String = String::new();

    assembly_code += &program::emit(assembly_ast);

    std::fs::write(filename, assembly_code).expect("could not write file");
}