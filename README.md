# exercise

A small collection of Leetcode-style coding exercises solved in Rust. Each
solution lives in its own module under `src/solutions/` and implements the
`Solution` trait.

---

## Usage

Run a solution by passing its index as a command-line argument:

```
cargo run         # runs solution 0 (default)
cargo run -- 0    # runs solution 0
cargo run -- 1    # runs solution 1
```

The program will prompt you for the required input. You can also pipe input
directly:

```
echo "3 1 2 3 4" | cargo run -- 0
echo "3" | cargo run -- 1
```

## Adding a new solution

1. Create a new file under `src/solutions/` (e.g., `two_sum.rs`).
2. Define a struct and implement `Solution` for it.
3. Register it in `src/solutions/mod.rs` — add the module declaration, a
   re-export of the struct, and an entry in `all_solutions()`.

## License

CC-BY-4.0 — see `LICENSE`.
