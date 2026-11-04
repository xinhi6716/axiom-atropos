use axiom_atropos::{cli, Context};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let output = cli::run(&args, &Context::default());
    let _ = io::stdout().write_all(output.stdout.as_bytes());
    let _ = io::stderr().write_all(output.stderr.as_bytes());
    std::process::exit(output.exit_code);
}
