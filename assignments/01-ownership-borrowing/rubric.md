# Rubric for Assignment 1: Ownership and Borrowing

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

### 1. Ownership

#### Unit Tests

For each of the 4 functions `takes_ownership and prints`, `first_letter`, `get_name`, and `slice_sum` 

2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile

#### `find_in_string`

2: An appropriate explanation is given in `analysis.md` for why the compiler needs more information on this function

1: An explanation is given, but there is at least one error within or it fails to identify what information the compiler requires

0: No attempt at an explanation is amde

### 2. Working with Mutable References

#### Unit Tests

For each of the 3 functions `increments`, `swap` and `strip_vowels_and_digits`:

2: All unit tests for this function pass. Additionally, for `strip_vowels_and_digits`, the helper functions must be used and called correctly.

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile

### 3. Singly Linked Stack

#### Unit Tests

1 point for each passing test case (so up to 3 in total)

### 4. Doubly Linked Stack

2: An appropriate explanation is given in `analysis.md` for why implementing a doubly-linked list is difficult using safe Rust, along with another example of something that would be difficult to implement.

1: An explanation is given, but there is at least one error within, it fails to articulate which ownership or borrowing rule is being violated, or fails to give a secondary example

0: No attempt at an explanation is made

### Points

**Correctness Score: XX/21**

## Code Quality

### Code Cleanliness

#### RustFmt

2: RustFmt produces no warnings when run on the changed files

1: RustFmt produces a few minor warnings when run on the changed files

0: RustFmt produces many warnings when run on the changed files

#### Clippy

2: Clippy produces no warnings when run on the changed files

1: Clippy produces a few minor warnings when run on the changed files

0: Clippy produces many warnings when run on the changed files

### Points
**Code Quality Score: XX/4**

## Summary

Add up the points in the three totals here.

**Total Score: XX/29**