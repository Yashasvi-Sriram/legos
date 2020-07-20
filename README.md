# legos
## description
- A collection of problems on structures and algorithms.
- Collected from generally known problem statements, codeforces, leetcode ...

### structure
- A problem can have multiple solutions.
- For each solution of a problem, the goal is to have
    - A crisp correctness proof (CP). If it is indicated that proof is done and is not written, it can be inferred from its code.
    - A crisp speed proof (SP). If it is indicated that proof is done and is not written, it can be inferred from its code.
    - Good number of correctness tests (CT). If there is an online judge, passing all online tests should suffice for this.
    - Optionally speed tests (ST).
- For a problem with multiple solutions, there can be optional speed-improvement tests (SIT).

## documentation
- The documentation of the code is itself.
- For each problem(crate), there is
    - If it is not a generally known problem, a link to problem statement.
    - Status table of each solution, representing status of CP, SP, CT, ST.
- For each solution, there is
    - If it is not inferred by code a CP.
    - If it is not inferred by code an SP.
    - Optionally notes, references ...

## code
- Code is written in stable rust.
- Each crate under `src/bin` corresponds to exactly one problem.
- There is no dependency b/w any two crates.
- Unit testing is used to add local correctness and speed[-improvment] tests.
- `legos_test_tools` is a path dependency crate used as a dev-dependency (i.e. for testing purposes) only.

## usage
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test --bin <crate>` to run tests in a crate.
- Use `cargo test` to run tests in all crates.
- Documentation can be read as-is from source code or `cargo doc --open` can be used to view it in HTML form.
- A crate corresponding to a codeforces problem is a single file and can be submitted as-is.

### common queries
- Q: How to quickly get status of each problem?
    - A: `rg //!` gives problem statement, status tables of each solution.
- Q: What does entries of status table mean?
    - A: `N` = not doing it, `TODO: <...>` = have to do, `Y` = done
- Q: How to get correctness and speed proofs for a problem?
    - A: Check `rg //!` to see if the proofs are done. If so, search for `///` in the corresponding crate.
- Q: How to get correctness and speed tests for a problem.
    - A: Tests are always unit tests. So just search the corresponding crate.
- Q: How to known which problems have speed-improvement test?
    - A: Only problems with multiple solutions can have speed improvement tests, so look for multiple status tables
- Q: How to get list of things left out to do?
    - A: Duh! `rg TODO`

## roadmap
- Problems(as crates) considered until now are listed in `src/bin/` directory.
