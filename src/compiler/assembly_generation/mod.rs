mod tacky_to_assembly;
mod replacing_pseudoregisters;
mod instruction_fixup;

use super::obj::{tast,aast};

// FUNCTIONS

pub(super) fn main(
    tacky_ast: tast::Program
) -> aast::Program {

    // GENERATE ASSEMBLY
    let mut assembly_ast: aast::Program = tacky_to_assembly::main(tacky_ast);
    dbg!(&assembly_ast);

    // REPLACE PSEUDOREGISTERS
    let allocation_size: u32;
    (assembly_ast, allocation_size) = replacing_pseudoregisters::main(assembly_ast);
    dbg!(&assembly_ast);
    dbg!(&allocation_size);

    // FIX UP INSTRUCTIONS
    assembly_ast = instruction_fixup::main(assembly_ast, allocation_size);

    return assembly_ast;
}