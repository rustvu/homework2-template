# RustVU (CS 3891) - Homework 2

## Optional / warmup exercises for the week

I highly recommend the [Rustlings project](https://github.com/rust-lang/rustlings) for practicing the basic concepts we learn in this class. These completely optional, not graded/submitted exercises can help you to teach Rust programming to your "fingers".

The recommended exercises for this week:

- `structs`
- `enums`
- `options`
- `strings`

## Assignment

This assignment contains a single Rust crate w.r.t structs and enums in Rust.

Your job is to make sure that crate compiles and builds properly and passes all of the built-in tests.

I placed `// TODO` comments in the code where I expect you to add implementation code. The test code is clearly marked with a `// DO NOT EDIT BELOW THIS LINE` comment. This should be evident: changing the test code is a (not too intelligent) way of cheating. I will handle any such attempts accordingly. However, you are allowed and encouraged to look at the test code to better understand what is expected from you.

## Use

You can always check your work with `cargo test`. You can also run individual tests by running `cargo test <test-name>` (see the names below).

The crate contains a _dummy_ `main()` function. You can use this function for your own needs.It is up to you if and how you want to use this for your development work. You can use `cargo run` to see your the execution results of your `main()` function. The only important requirement is that this function should not break the compilation/build process.

## Grading

Make sure you __commit__ and __push__ your assignment repository once you manage to run `cargo test` without any errors or warnings.

The homework is graded by test (no partial credits are given for failed tests):

|Test                   | Points|
|-----------------------|-------|
|test_new               |    10 |
|test_debug             |     5 |
|test_eq                |     5 |
|test_clone             |     3 |
|test_copy              |     3 |
|test_from_name         |    15 |
|test_from_hex          |    15 |
|test_to_hex            |     5 |
|test_distance          |     7 |
|test_closest_base_color|    15 |
|test_to_grayscale      |     5 |
|test_darken            |     7 |
|test_mix               |     5 |

Once you __push__ your solution to the repository, GitHub Classroom will run the automated test. I highly recommend to [verify your results of this CI/CD worflow](https://docs.github.com/en/education/manage-coursework-with-github-classroom/learn-with-github-classroom/view-autograding-results) - I use these results for grading your work.
