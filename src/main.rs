use std::io::Read;
use std::sync::OnceLock;

mod solution;
mod solutions;
mod types;
mod utils;

pub static STDIN_INPUT: OnceLock<String> = OnceLock::new();

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("read stdin");
    STDIN_INPUT.set(input).expect("stdin already initialized");

    for s in solutions::all_solutions() {
        println!("Running: {}", s.name());
        utils::get_downcast_string_result(s.solve());
    }
}
