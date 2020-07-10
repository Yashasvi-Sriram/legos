# legos
## description
- A collection of problems on structures and algorithms.

## code
- Code is written in rust.
- Rust stable is used.
- Each crate under `src/bin` corresponds to exactly one problem.
- There is no dependency b/w any two crates
- Unit testing is used to add manual test cases for correctness and speed-improvements.
- `legos_*` are path dependency crates and are used as dev-dependencies (for testing purposes) only.

## documentation
- The documentation of the code is itself.
- For things that can not be expressed in code (notes, proofs ...), there is minimal documentation.

## usage
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test` to run tests in all crates.
- A crate corresponding to a codeforces problem is a single file and can be submitted as-is.
