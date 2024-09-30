# Rubric for Assignment 3: Grep-rs

## Submission

### Canvas

1: The link submitted on Canvas is for the pull request containing the
assignment submission.

0: The link submitted on Canvas is not for the pull request containing the
assignment submission.

### Git Branch

1: The branch includes only changes for this assignment.

0: The branch includes commits or changes outside this assignment.

### Git Commit Message

1: All commit messages for the branch are reasonably clear.

0: One or more commit messages for the branch are reasonably clear.

### GitHub Pull Request

1: The pull request is made to the `main` branch of the student's fork of the
course repository.

0: The pull request is made to a branch other than `main` or to a repo that is
not the student's fork of the course repository.

### Points

**Submission Score: XX/4**

## Correctness

### Functionality

We're not providing unit tests this time around (you should be writing those), but we will test your binary for correctness!

#### Input

2: Your program correctly supports both a file and stdin

1: Your program only supports one input mode

0: Your program cannot take either file or stdin input

#### Needle Type

2: Your program correctly supports both literal text and a regex as a pattern type through the CLI flag

1: Your program only supports no needle type

0: Your program cannot accept a user provided needle

#### Flags

2: Your program correctly supports both the `ignore_case` and `invert_match` CLI flags

1: Your program supports one of the two flags

0: Your program supports neither of the flags

#### Output

2: Your program correctly supports plain printing of matching lines and printing the matched substring in a color specified by the CLI flag

1: Your program only supports one output mode

0: Your program is unable to provide output

### Implementation

For each of:
- Input stage
- Needle Type
- Output stage

2: Correctly used traits to abstract common functionality of each stage of the program

1: Traits are present, but there is an error with their use/they are not used to polymorphize your code

0: Did not use traits for this portion of the program

### Points

**Correctness Score: XX/14**

## Code Quality

This section is growing!

### Design

#### File/Module organization

2: The program is appropriately split into reasonable files/modules

1: The program is somewhat organized, but there's room for improvement (splits aren't idiomatic, or some modules are still too large, etc.)

0: The program is not split at all

#### Tests

For each of:
- Input Stage
- Needle Type
- Modifier flags (`invert_match` / `ignore_case`)
- Output Stage

2: This functionality is appropriately tested with passing unit test cases

1: This functionality has unit tests, but they don't adequately cover code path/reasonable input cases

0: This functionality has no unit tests

#### Miscellaneous Code Style

- Variable and function names are descriptive and follow style guidelines
- The code is structured using idiomatic Rust patterns that we've learned in class 
- Where needed, there are comments to explain the rationale behind certain blocks of code
- The code is generally readable and easy to understand

XX/4

### Lint/Format

#### RustFmt

2: RustFmt produces no warnings when run on the changed files

1: RustFmt produces a few minor warnings when run on the changed files

0: RustFmt produces many warnings when run on the changed files

#### Clippy

2: Clippy produces no warnings when run on the changed files

1: Clippy produces a few minor warnings when run on the changed files

0: Clippy produces many warnings when run on the changed files

### Points
**Code Quality Score: XX/18**

## Summary
Total Score: XX/36
