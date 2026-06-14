mod solution;
mod solutions;
mod types;
mod utils;

fn main() {
    for s in solutions::all_solutions() {
        println!("Running: {}", s.name());
        utils::get_downcast_string_result(s.solve());
    }
}
