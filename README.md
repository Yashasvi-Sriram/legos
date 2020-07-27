# legos

## description
- A collection of independent problems on structures and algorithms.
- Collected from generally known problem statements, codeforces, leetcode ...

- Each problem has one problem statement.
- A problem can have multiple solutions.
- For each solution, the goal is to have the following requirements satisfied.
    - The correctness proof.
    - The time complexity proof.
    - The space complexity proof.
    - Enough correctness tests.
    - The time complexity test.
- For a problem with multiple solutions, there can be one time complexity improvement test.

## code
- Code is written in rust stable.
- Each crate under `src/bin` corresponds to exactly one problem. There is no dependency b/w any two crates.
- Unit testing is used to add correctness and time complexity[ improvment] tests.
- `legos_test_tools` is a path dependency crate used as a dev-dependency (i.e. for testing purposes) only.
- The proofs are inferred by code as much as possible. In case that's not possible, a minimal proof is written in documentation.
- The time complexity test (of a solution) is always named `time_complexity()`.
- The time complexity improvement test (of a problem) is always named `time_complexity_improvment()`.
- If any of the requirements is not complete, it is marked with `TODO` for tracking purposes.

## documentation
- The documentation of the code is itself.
- For things that cannot be expressed in code (some proofs, references ...), there is minimal documentation.

## usage
- Note that none of the requirements are validated by the compiler. That burden falls on the developer.
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test --bin <crate>` to run tests in a crate.
- Use `cargo test` to run tests in all crates.
- Documentation can be read as-is from source code or `cargo doc --open` can be used to view it in HTML form.

## roadmap
- Problems considered until now can be found as crates in `src/bin/` directory.
- [ ] dp
    - [ ] knapsack 0/1
- [ ] greedy
- [ ] divide and conquer
    - [ ] merge sort
    - [ ] quick sort
