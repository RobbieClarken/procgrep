#![feature(untagged_unions)]

use docopt::Docopt;
use serde_derive::Deserialize;

mod processes;

const USAGE: &'static str = "
procgrep

Usage:
  procgrep <pattern>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_pattern: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    for process in processes::get_processes() {
        if process.command.contains(&args.arg_pattern) {
            println!("{}", process.pid);
        }
    }
}
