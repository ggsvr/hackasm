pub mod codegen;
pub mod instr;
pub mod parser;
pub mod symbols;

use std::path::{Path, PathBuf};

pub fn assemble(source: &str) -> Result<String, ()> {
    let instructions = match parser::parse_asm(&source) {
        Ok(ins) => ins,
        Err(errs) => {
            for e in errs {
                print_error(e);
            }
            return Err(());
        }
    };
    let opcodes = codegen::codegen_instructions(instructions);
    let bincode = codegen::opcodes_to_string(&opcodes);
    Ok(bincode)
}
/// assemble a source file and save a file with the same name containing the bincode
pub fn assemble_from_file(file: &Path) {
    let Some(outfile) = get_out_filename(file) else {
        let filename = file.to_string_lossy();
        print_error(format!("invalid file name: {filename}"));
        return;
    };
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            print_error(e.to_string());
            return;
        }
    };
    let Ok(bincode) = assemble(&source) else {
        return;
    };
    if let Err(e) = std::fs::write(&outfile, bincode) {
        print_error(e.to_string());
    }
}

fn get_out_filename(in_file: &Path) -> Option<PathBuf> {
    if !validate_filename(in_file) {
        return None;
    }
    let mut out = PathBuf::from(in_file);
    out.set_extension("hack");
    Some(out)
}
fn validate_filename(file: &Path) -> bool {
    file.extension().and_then(|ext| ext.to_str()) == Some("asm")
}
fn print_error(error: String) {
    eprintln!("Error: {error}");
}
