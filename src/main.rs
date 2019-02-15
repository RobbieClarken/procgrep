#![feature(untagged_unions)]

use docopt::Docopt;
use serde_derive::Deserialize;

mod processes;
mod tty;

use processes::Process;

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

trait Matcher {
    fn matches(&self, process: &Process) -> bool;
}

struct TtyMatcher {
    tty: i32,
}

impl Matcher for TtyMatcher {
    fn matches(&self, process: &Process) -> bool {
        process.tty == self.tty
    }
}

struct PatternMatcher {
    pattern: String,
}
impl Matcher for PatternMatcher {
    fn matches(&self, process: &Process) -> bool {
        process.command.contains(&self.pattern)
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_help {
        println!("{}", USAGE);
        return;
    }
    let mut matchers: Vec<Box<dyn Matcher>> = Vec::new();
    if args.flag_tty.len() > 0 {
        let tty = tty::get_tty(&args.flag_tty).expect("invalid tty");
        matchers.push(Box::new(TtyMatcher { tty }));
    }
    if args.arg_pattern.len() > 0 {
        matchers.push(Box::new(PatternMatcher {
            pattern: args.arg_pattern,
        }));
    }
    for process in processes::get_processes() {
        if !matchers.iter().all(|matcher| matcher.matches(&process)) {
            continue;
        }
        println!("{} {}", process.pid, process.command);
    }
}
