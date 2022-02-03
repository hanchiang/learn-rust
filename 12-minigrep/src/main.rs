use std::env;
use std::fs;
use std::process;

use minigrep::{Config, run};

// Usage: cargo run searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    // error message is captured in the closure, i.e. |err|
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }

}

