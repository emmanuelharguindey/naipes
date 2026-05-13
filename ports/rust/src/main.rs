use std::process;

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let code = naipes::cli::main(argv);
    process::exit(code);
}
