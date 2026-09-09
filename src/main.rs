mod compiler;

// OBJECTS

enum Stage {
    Preprocess,
    Lex,
    Parse,
    Validate,
    Tacky,
    CodeGen,
    Compile,
    Full
}

// FUNCTIONS

fn main() {
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = std::env::args().collect();
    let mut flag_opt: Option<String> = None;
    let mut filename_opt: Option<String> = None;

    for arg in &args[1..] {
        if arg.chars().nth(0).unwrap().eq(&'-') {
            if flag_opt.is_none() {
                flag_opt = Some(arg.clone());
            } else {
                panic!("too many flags");
            }
        } else {
            if filename_opt.is_none() {
                filename_opt = Some(arg.clone());
            } else {
                panic!("too many arguments");
            }
        }
    }

    let filename: String = filename_opt.expect("missing one required argument (filename)");
    // https://doc.rust-lang.org/book/ch04-03-slices.html
    let base_filename: &str = &filename[..filename.len() - 2];

    let mut stage: Stage = Stage::Full;
    match flag_opt {
        None => {},
        Some(flag) => {
            if flag.eq("--preprocess") {
                stage = Stage::Preprocess;
            } else if flag.eq("--lex") {
                stage = Stage::Lex;
            } else if flag.eq("--parse") {
                stage = Stage::Parse;
            } else if flag.eq("--validate") {
                stage = Stage::Validate;
            } else if flag.eq("--tacky") {
                stage = Stage::Tacky;
            } else if flag.eq("--codegen") {
                stage = Stage::CodeGen;
            } else if flag.eq("-S") {
                stage = Stage::Compile;
            } else {
                panic!("unexpected flag");
            }
        }
    }

    // PREPROCESS
    // https://doc.rust-lang.org/std/process/struct.Command.html
    let _preprocessing_output: std::process::Output = std::process::Command::new("gcc")
        .args(["-E", "-P", &filename, "-o", &(base_filename.to_owned() + ".i")])
        .output()
        .expect("failed to execute gcc preprocessor");
    // dbg!();
    // dbg!(&preprocessing_output);
    
    // COMPILE
    match &stage {
        Stage::Preprocess => {},
        Stage::Lex |
        Stage::Parse |
        Stage::Validate |
        Stage::Tacky |
        Stage::CodeGen |
        Stage::Compile |
        Stage::Full => {
            compiler::main(
                &(base_filename.to_owned() + ".i"),
                &(base_filename.to_owned() + ".s"),
                &stage
            );
            match std::fs::remove_file(&(base_filename.to_owned() + ".i")) {
                Ok(_) => {},
                Err(err) => {
                    dbg!(&err);
                }
            }
        }
    }

    // ASSEMBLE
    match &stage {
        Stage::Preprocess |
        Stage::Lex |
        Stage::Parse |
        Stage::Validate |
        Stage::Tacky |
        Stage::CodeGen |
        Stage::Compile => {},
        Stage::Full => {
            let _assembling_output: std::process::Output = std::process::Command::new("gcc")
                .args([&(base_filename.to_owned() + ".s"), "-o", base_filename])
                .output()
                .expect("failed to execute gcc assembler");
            // dbg!();
            // dbg!(&_assembling_output);
            match std::fs::remove_file(&(base_filename.to_owned() + ".s")) {
                Ok(_) => {},
                Err(err) => {
                    dbg!(&err);
                }
            }
        }
    }

    // dbg!();
}
