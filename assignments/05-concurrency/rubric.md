# Rubric for Assignment 5: Concurrency

## Submission

### Canvas

1: The link submitted on Canvas is for the pull request containing the assignment submission.

0: The link submitted on Canvas is not for the pull request containing the assignment submission.

### Git Branch

1: The branch includes only changes for this assignment.

0: The branch includes commits or changes outside this assignment.

### Git Commit Message

1: All commit messages for the branch are reasonably clear.

0: One or more commit messages for the branch are reasonably clear.

### GitHub Pull Request

1: The pull request is made to the `main` branch of the student's fork of the course repository.

0: The pull request is made to a branch other than `main` or to a repo that is not the student's fork of the course repository.

### Points

**Submission Score: XX/4**

## Correctness

### Thread Pool

For each of the following features:
- Spawns `n` number of threads where `n` is a user-provided parameter
- Supports sending jobs to threads in an even distribution
- Supports those jobs returning a result to the main thread
- Gracefully joins all threads when dropped

2: This feature is implemented and functions correctly

1: This feature is implemented, but contains an error

0: This feature was not implemented 

### Merge Sort

#### Correctness

3: The program correctly sorts vectors using Merge Sort

2:  The program fails to correctly sort vectors due to a minor issue

1:  The program fails to correctly sort vectors due to a major issue

0: No attempt was made to sort vectors

#### Implementation

2: A thread pool is used to evenly distribute the work among `n` given threads

1: A thread pool is used, but there is a problem with its use or it fails to distribute the work between threads evenly

0: No thread pool is used in the merge sort implementation

### Benchmarking

In `benchmark.md`:

3: The quickest number of threads is identified along with a reasonable explanation for why certain numbers are faster than others

2: A number and explanation is given, but the explanation has a minor error

1: A number and explanation is given, but the explanation has a major error

0: No number and/or explanation is given

### Points

**Correctness Score: XX/16**

## Code Quality

### Design

#### Tests

For `thread_pool.rs` and `main.rs` (merge sort):

3: This functionality is appropriately tested with passing unit test cases

2: This functionality has unit tests, but they miss a few code paths/reasonable input cases

1: This functionality has unit tests, but they miss multiple core code paths

0: This functionality has no unit tests


#### General Cleanliness

4: The code is effectively documented, especially for sources of user-facing errors. Additionally, function signatures are clear and the code is overall very readable.

2: Attempts at verbose documentation were made, but certain sources of error are unclear OR poor function signatures are used and readers would have slight difficulties while scanning the code.

0: Documentation is severely lacking in several places and the code is not written in a readable way.

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
**Code Quality Score: XX/14**

## Summary

**Total Score: XX/34**
