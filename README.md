# legos
## description
- A collection of problems on structures and algorithms.
- Collected from generally known problem statements and codeforces.
- The goal is to have crisp proofs, correctness tests (& speed/speed-improvment tests if required) for each problem.

## code
- Code is written in rust.
- Rust stable is used.
- Each crate under `src/bin` corresponds to exactly one problem.
- There is no dependency b/w any two crates
- Unit testing is used to add manual test cases for correctness and speed-improvements.
- `legos_*` are path dependency crates and are used as dev-dependencies (for testing purposes) only.

## documentation
- The documentation of the code is itself.
- In problems where proof is not expressed in code clearly, it is written in documentation of same crate.
    - It can be read as-is from source code.
    - Or `cargo doc && cargo doc --open` can be used to view it in HTML form.
- For things not expressed in code (references, notes ...) there is minimal documentation.

## usage
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test --bin <crate>` to run tests in a crate.
- Use `cargo test` to run tests in all crates.
- A crate corresponding to a codeforces problem is a single file and can be submitted as-is.
- Things left (proofs, correctness tests, speed tests, cleaning, optimizations ...) are marked with `TODO`. Use grep or ripgrep to list them viz. `rg TODO`.

## roadmap
- Problems solved until now are listed in `src/bin/` directory.
