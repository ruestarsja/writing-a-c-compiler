mod obj;
mod lexer;
mod parser;
mod semantic_analysis;
mod tacky_generation;
mod assembly_generation;
mod code_emission;


use super::Stage;
use obj::token::Token;
use obj::{cast, tast, aast};

// FUNCTIONS

pub(super) fn main(
    preprocessed_filename: &str,
    assembly_filename: &str,
    stage: &Stage
) {
    // LEX
    let tokens_opt: Option<Vec<Token>> = match stage {
        Stage::Preprocess => {
            None
        },
        Stage::Lex |
        Stage::Parse |
        Stage::Validate |
        Stage::Tacky |
        Stage::CodeGen |
        Stage::Compile |
        Stage::Full => {
            Some(lexer::main(preprocessed_filename))
        }
    };
    dbg!(&tokens_opt);

    // PARSE
    let ast_opt: Option<cast::Program> = match stage {
        Stage::Preprocess |
        Stage::Lex => {
            None
        },
        Stage::Parse |
        Stage::Validate |
        Stage::Tacky |
        Stage::CodeGen |
        Stage::Compile |
        Stage::Full => {
            Some(parser::main(tokens_opt.unwrap()))
        }
    };
    dbg!(&ast_opt);

    // ANALYZE SEMANTICS
    let validated_ast_opt: Option<cast::Program> = match stage {
        Stage::Preprocess |
        Stage::Lex |
        Stage::Parse => {
            None
        },
        Stage::Validate |
        Stage::Tacky |
        Stage::CodeGen |
        Stage::Compile |
        Stage::Full => {
            Some(semantic_analysis::main(ast_opt.unwrap()))
        }
    };
    dbg!(&validated_ast_opt);

    // GENERATE TACKY
    let tacky_ast_opt: Option<tast::Program> = match stage {
        Stage::Preprocess |
        Stage::Lex |
        Stage::Parse |
        Stage::Validate => {
            None
        },
        Stage::Tacky |
        Stage::CodeGen |
        Stage::Compile |
        Stage::Full => {
            Some(tacky_generation::main(validated_ast_opt.unwrap()))
        }
    };
    dbg!(&tacky_ast_opt);
    
    // GENERATE ASSEMBLY
    let assembly_ast_opt: Option<aast::Program> = match stage {
        Stage::Preprocess |
        Stage::Lex | Stage::Parse | Stage::Validate | Stage::Tacky => { None },
        Stage::CodeGen | Stage::Compile | Stage::Full => {
            Some(assembly_generation::main(tacky_ast_opt.unwrap()))
        }
    };
    dbg!(&assembly_ast_opt);

    
    // EMIT CODE
    let _opt: Option<_> = match stage {
        Stage::Preprocess | Stage::Lex | Stage::Parse | Stage::Validate | Stage::Tacky | Stage::CodeGen => { None },
        Stage::Compile | Stage::Full => {
            Some(code_emission::main(assembly_filename, assembly_ast_opt.unwrap()))
        }
    };
}
