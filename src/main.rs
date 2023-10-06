use std::env;
use std::process;

use grep_rust::Args;

fn main() {
    let args = Args::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_rust::run(args) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
