# legos
## description
- A collection of problems on structures and algorithms.
- Collected from generally known problem statements, codeforces, leetcode ...
- For each problem, the goal is to have crisp correctness and speed proofs and correctness tests.
    - If proofs are not written for a problem, it means they can be inferred from its code.
    - For problems with online judge, passing all tests should be good enough as correctness tests.
- For few problems, there are speed and speed improvement tests.

## code
- Code is written in stable rust.
- Each crate under `src/bin` corresponds to exactly one problem.
- Unit testing is used to add local correctness and speed[-improvment] tests.
- There is no dependency b/w any two crates.
- `legos_test_tools` is a path dependency crate used as a dev-dependency (i.e. for testing purposes) only.

## documentation
- The documentation of the code is itself.
- For things not expressed in code clearly (some proofs, references, notes ...) minimal documentation is written in same crate.
    - Can be read as-is from source code.
    - Or `cargo doc && cargo doc --open` can be used to view it in HTML form.

## usage
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test --bin <crate>` to run tests in a crate.
- Use `cargo test` to run tests in all crates.
- A crate corresponding to a codeforces problem is a single file and can be submitted as-is.

## roadmap
- Problems solved until now are listed in `src/bin/` directory.
