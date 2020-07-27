# legos

## description
- A collection of independent problems on structures and algorithms.
- Collected from generally known problem statements, codeforces, leetcode ...

### goals
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

### testing
- The goals mentioned above are ensured using the unit test system.
- `legos_test_tools` is a path dependency crate used as a dev-dependency (i.e. for testing purposes) only.
- `legos_test_tools` provides a macro `test_suite!` which takes > 0 function names as arguments.
    - It adds a set of tests, specifically.
        | | test | inner function call(s) |
        | --- | --- | --- |
        | 1 | `correctness_proof()` | `cp()` |
        | 2 | `time_complexity_proof()` | `tp()` |
        | 3 | `space_complexity_proof()` | `sp()` |
        | 4 | `correctness_tests()` | all macro args |
        | 5 | `time_complexity_test()` | `tt()` |
    - 1, 2, 3, 5 each call one specific named function (col 3), which user has to implement.
    - 4 calls all inputs given to the macro, which are meant to be names of correctness tests.
    - This way the `cargo test` won't compile unless all the tests are implemented.
- `test_suite!` is meant to be used per solution.
- The time complexity improvement test (of a problem) is always named `time_complexity_improvment()`.
- If any of the requirements is not complete, it is marked with `TODO` for tracking purposes.

## documentation
- The documentation of the code is itself.
- For things that cannot be expressed in code (references, notes ...), there is minimal documentation.

## usage
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test --bin <crate>` to run tests in a crate.
- To ensure that all tests are implemented for each solution.
    - Ensure that `test_suite!` exists for each solution.
        - Use `rg` or similar tools.
        - `rg --files-without-match test_suite!` prints all files without `test_suite!`.
        - Some problems may have multiple solutions in same file, ensure each solution has a `test_suite!`.
    - Ensure that `cargo test` compiles.
- Note that although the `test_suite!` ensures all goal functions are implemented, it does not validate their implementations. That burden falls on the user.
- Use `cargo test` to run tests in all crates.
- To get TODOs, use `rg TODO`.
- Documentation can be read as-is from source code or `cargo doc --open` can be used to view it in HTML form.

## roadmap
Problems considered until now can be found as crates in `src/bin/` directory.

### maintainance
- [ ] How to detect solutions with no `test_suite!`?
- [ ] More validation on test implementation.
- [ ] Handle `time_complexity_improvment()`.
- [ ] Proof enum: Inferred, Todo, NotPlanned variants.
- [ ] Use `test_suite!` for all solutions.

### problems
- [ ] dp
    - [ ] knapsack 0/1
- [ ] greedy
- [ ] divide and conquer
    - [ ] merge sort
    - [ ] quick sort
