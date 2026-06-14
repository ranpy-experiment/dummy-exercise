mod solution;
mod solutions;
mod types;
mod utils;

fn main() {
    let all = solutions::all_solutions();
    let idx: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let s = &all[idx];
    println!("Running: {}", s.name());
    utils::get_downcast_string_result(s.solve());
}
