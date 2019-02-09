#![feature(untagged_unions)]

use docopt::Docopt;
use serde_derive::Deserialize;

mod processes;
mod tty;

const USAGE: &'static str = "
procgrep

Usage:
    procgrep [options] <pattern>
    procgrep -t <terminal>
    procgrep (-h | --help)

Options:
    -h, --help                       Show this screen.
    -t <terminal>, --tty <terminal>  Restrict matches to processes associated with a terminal.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_pattern: String,
    flag_tty: String,
    flag_help: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_help {
        println!("{}", USAGE);
        return;
    }
    let tty = match args.flag_tty.len() {
        0 => None,
        _ => Some(tty::get_tty(&args.flag_tty)),
    };
    for process in processes::get_processes() {
        if let Some(t) = tty {
            if process.tty != t {
                continue;
            }
        }
        if process.command.contains(&args.arg_pattern) {
            println!("{}", process.pid);
        }
    }
}
