#![feature(untagged_unions)]

mod processes;

fn main() {
    for process in processes::get_processes() {
        println!("{}", process.pid);
    }
}
