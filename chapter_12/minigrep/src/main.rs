use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // The run function doesn’t return a value that we want to unwrap in the same way that Config::build returns the Config instance
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
