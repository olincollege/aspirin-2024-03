# Rubric for Assignment 2: Collections and Lifetimes

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

### 1. Getting your *Life* Together
For each of the three functions `split_string`, `find_differences`, and `merge_names`

2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile

Each of the 4 elements that could require lifetime annotations `split_string`, `Differences`, `find_differences`, and `merge_names`

1: The function/struct has the appropriate lifetime, if required, that allows the output to live aslong as possible OR the lifetime annotation is omitted because it can be inferred by the compiler.

0: The lifetime annotation cuts the output's lifetime shorter than it can be OR lifetimes are unnecessarily annotated.

### 2. eLEET Code
For each of the functions `longest_equal_sequence_prescriptive`, `longest_equal_sequence_functional`, `is_valid_paranthesis`, `longest_common_substring`, and `longest_common_substring_multiple`

2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile.

Additional Note: 0 points will be provided for `longest_equal_sequence_XXXXXX` if it is not implemented prescriptively/functionally for each of the respective functions.

Each of the 5 elements that could require lifetime annotations `longest_equal_sequence_prescriptive`, `longest_equal_sequence_functional`, `is_valid_paranthesis`, `longest_common_substring`, and `longest_common_substring_multiple`

1: The function/struct has the appropriate lifetime, if required, that allows the output to live aslong as possible OR the lifetime annotation is omitted because it can be inferred by the compiler.

0: The lifetime annotation cuts the output's lifetime shorter than it can be OR lifetimes are unnecessarily annotated.

For `longest_equal_sequence_prescriptive` and `longest_equal_sequence_prescriptive`

1: The function is implemented using generics such that generic type, T, is limited (and only limited) to types that implement `std::cmp::PartialEq`.

0: The function is implemented in a way that requires it to be more strict that just requiring `std::cmp::PartialEq` to be implemented on the desired type.

### 3. A Wide *Array* of Problems
#### 3.0. Product-Matrix Fit
For each of the functions `dot_product_prescriptive` and `dot_product_functional`

2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile.

Note: 0 points will be provided for `dot_product_XXXXXX` if it is not implemented prescriptively/functionally for each of the respective functions.

#### 3.1. Row-mantic Results
2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile.

### 4. Mirrors on the Wall
For the reflection in `reflection.md`:

4: The reflection is thoughtful and shows thorough evaluations of each potential implementation, including significant discussion of at least one of the following topics: General Readability, Ease of Implementation, or Scalability.

2: A surface level reflection is provided, but lacks any depth of significant considerations of at least one of the specified topic areas of discussion.

0: No reflection was submitted, or it lacks significant thought given to the two different implementations.

### Points

**Correctness Score: XX/37**

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

**Total Score: XX/45**