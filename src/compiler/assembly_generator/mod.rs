pub(super) mod generator;
mod pseudoregister_replacer;
mod instruction_fixer;

use super::obj::{tast,aast};

// FUNCTIONS

pub(super) fn main(
    tacky_ast: tast::Program
) -> aast::Program {

    // GENERATE ASSEMBLY
    let mut assembly_ast: aast::Program = generator::main(tacky_ast);
    dbg!(&assembly_ast);

    // REPLACE PSEUDOREGISTERS
    let allocation_size: u32;
    (assembly_ast, allocation_size) = pseudoregister_replacer::main(assembly_ast);
    dbg!(&assembly_ast);
    dbg!(&allocation_size);

    // FIX UP INSTRUCTIONS
    assembly_ast = instruction_fixer::main(assembly_ast, allocation_size);

    return assembly_ast;
}