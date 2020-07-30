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
- If any of the requirements is not complete, it is marked with `TODO` for tracking purposes.

### testing
- The goals mentioned above are ensured using the unit test system.
- `legos_test_tools` is a crate used as a dev-dependency (i.e. for testing purposes) only.
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

## documentation
- The documentation of the code is itself.
- For things that cannot be expressed in code (links, notes, ...), there is minimal documentation.

## usage
- Documentation can be read as-is from source code or `cargo doc --open` can be used to view it in HTML form.
- Use `cargo run --bin <crate>` to run (the main function of, not the tests) a crate.
- Use `cargo test --bin <crate> -- --nocapture` to run tests in a crate while printing their output. For silent execution use `cargo test --bin <crate>`.
- To ensure that all tests are implemented for each solution.
    - Ensure that `test_suite!` exists for each solution.
        - Try using `rg` or similar tools.
        - `rg --files-without-match test_suite!` prints all files without `test_suite!`.
        - Some problems may have multiple solutions in same file, ensure each solution has a `test_suite!`.
    - Ensure that `cargo test` compiles.
- Note that although the `test_suite!` ensures all goal functions are implemented, it does not validate their implementations. That burden falls on the user.
- Use `cargo test` to run all tests in all crates.
- To get TODOs, use `rg -i TODO`.

## roadmap
Problems considered until now can be found as crates in `src/bin/` directory.

### maintainance
- [ ] How to detect solutions with no `test_suite!`?
- [ ] More validation on test implementation.
- [ ] Handle `time_complexity_improvment()`.
- [x] Return types for tests.
    CorrectnessProof and ComplexityProof enums.
    No enum for time complexity test.
    If left empty it implies not planned.
    If planned but not completed, leave a TODO.
- [x] Use `test_suite!` for all solutions until now.

### problems
- [ ] knapsack 0/1
    - [ ] dp
    - [x] exponential
- [ ] merge sort
    - [ ] iterative
    - [ ] recursive
- [ ] quick sort
