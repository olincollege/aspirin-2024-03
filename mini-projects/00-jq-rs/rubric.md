# Rubric For Mini Project 0: JQ

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

## Features
### Command Line Interface
For each of the required CLI arguments `filter-string`, `path-to-json`, `color-output`, `monochrome-output`, `sort-keys`, `indent`, and `compact-output`

0.5: The argument can be correctly parsed and consumed by the rest of the package.

0: The argument is not correctly parsed and propagated to the rest of the package.

Additionally, the command line parsing correctly raises errors to the user when conflicting arguments are inputted.

1.5: Errors are properly determined and bubbled up to the user.

0: Errors are not propagated back to the user.

### Supported Filters
For each of the required filters `identity`, `object-identifier index`, `array index`, `array slice`, `pipe`, and `array iterator`

2: The filter is correctly implemented and properly parses all possible JSON inputs.

1: Minor errors exist in the implement of the filter.

0: Major errors exist in the implementation, or the filter was omitted from the package.

Additionally, comprehensive unit tests are required for each feature:

2: Full unit test coverage is provided (with passing implementations) to exercise a wide varity of use-cases and edge-cases of the filter, ensuring that the output is correct and that errors are properly outputted.

1: Unit tests are present, but lack a significant breadth of cases to fully exercise the filter.

0: Tests are missing, or lack significant assessment of the filter's performance.

### Built-In Functions
For each of the required built-in functions `add`, `length`, `del`

2: The built-in function is correctly implemented and properly processes all possible JSON inputs.

1: Minor errors exist in the implementation of the operator.

0: Major errors exist in the implementation, or the function was omitted from the package.

Additionally, comprehensive unit tests are required for each function:

2: Full unit test coverage is provided (with passing implementations) to exercise a wide varity of use-cases and edge-cases of the function, ensuring that the output is correct and that errors are properly outputted.

1: Unit tests are present, but lack a significant breadth of cases to fully exercise the function.

0: Tests are missing, or lack significant assessment of the function's performance.

### Formatted Printing
For each of the required formats, `colored printing`, `monochrome printing`, `sorted keys`, `indents`, and `compact output`

2: The formatter is correctly implemented and properly displays any JSON object..

1: Minor errors exist in the implementation of the formatter.

0: Major errors exist in the implementation, or the formatter was omitted from the package.

Additionally, unit tests are required for each function:

2: Full unit test coverage is provided (with passing implementations) to ensure that the formatter can correctly handle all expected inputs.

1: Unit tests are present, but lack significant breadth of cases to fully exercise the formatter.

0: Tests are missing, or lack significant assessment of the formatter.

### Points
**Features Score: XX/61**

## Code Quality

### Design

#### File/Module organization

2: The program is appropriately split into reasonable files/modules

1: The program is somewhat organized, but there's room for improvement (splits aren't idiomatic, or some modules are still too large, etc.)

0: The program is not split at all

#### Scalability

5: The code is architected with scalability in mind. If more features needed to be added to the filtering/printing/CLI, they could easily be integrated.

3: The code was architected with some scalaibility in mind. However, significant refactoring would be needed in order to expand the feature set of this tool. (Possible symptoms include not modularizing enough/not effectively using traits + generics).

0: The code is not scalable. The entire package would need to be refactored in order to expand its features.

#### General Cleanliness

4: The code is effectively documented, especially for sources of user-facing errors. Additionally, function signatures are clear and the code is overall very readable.

2: Attempts at verbose documentation were made, but certain sources of error are unclear OR poor function signatures are used and readers would have slight difficulties while scanning the code.

0: Documentation is severely lacking in several palces and the code is not written in a readable way.

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
**Code Quality Score: XX/15**

## Summary
Add up the point totals here

**Total Score: XX/80**