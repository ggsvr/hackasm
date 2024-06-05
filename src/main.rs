use std::env;
use std::path::Path;
use std::process::ExitCode;
static USAGE: &str = "usage: hackasm <source file>";

fn main() -> ExitCode {
    let Some(filename) = env::args().nth(1) else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };
    eprintln!("assembling {filename}...");
    hackasm::assemble_from_file(Path::new(&filename));
    ExitCode::SUCCESS
}
