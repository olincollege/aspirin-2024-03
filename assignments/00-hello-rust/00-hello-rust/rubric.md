# Rubric for Assignment 0: Hello Rust!

## Submission
### Canvas
1: The link submitted on Canvas is for the pull request containing the assignment submission.

0: The link submitted on Canvas is not for the pull request containing the assignment submission.

### Git Branch
1: The branch includes only changes for this assignment and no unnecessary files are included.

0: The branch includes commits or changes outside this assignment.

### Git Commit Message
1: All commit messages for the branch are reasonably clear.

0: One or more commit messages for the branch are reasonably clear.

### GitHub Pull Request
1: The pull request is made to the main branch of the student's fork of the course repository.

0: The pull request is made to a branch other than main or to a repo that is not the student's fork of the course repository.

### Points
Submission Score: XX/4

## Correctness
### 2.2. Documentation
All of the following pertain to `guessing_game.rs`

#### File-Level Comment
1: A file-level comment exists at the top of the file and accuractely and concisely captures the purpose of the file.

0: A file-level comment is missing, or it is not representative of the file.

#### Document Comments
For each function, `get_input` and `main`, :

1: A document comment exists for this function and accurately captures the inputs and outputs of the function, along with a high-level description of its purpose.

0: A document comment is missing, or is missing detail about the purpose of the function.

#### In-Line Comments
1: In-line comments are present when necessary to describe significant blocks of code.

0: In-line comments are missing.

### 3.0. Fizz Buzz
1: The function `print_fizz_buzz` correctly prints out numbers 1 to the inputted `max_num`, replacing numbers divisible by 3 with "Fizz", divisble by 5 with "Buzz`, and disible by both with "FizzBuzz".

0: There is a bug in the implementation.

### 3.1. *Red*y for Action?
#### Unit Tests
For each function, `get_next_color` and `get_next_state`

2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile

### 3.2 Show Some *Class*
#### 3.2.0
1: The three students specified in the problem description are correctly added to the `OLIN_STUDENTS array.

0: The students are incorrectly added to the list.

#### Unit Tests
For each function, `get_average_gpa`, `get_num_excel_students_for_class`, and `get_best_class`

2: All unit tests for this function pass

1: One or more unit tests fail due to a minor error

0: The code fails one or more unit tests due to a major error, or the test fails to compile

### 3.3.All About That Base
#### Implementation
##### Standard Input
1: The calculator implementation takes in two numbers, and the desired operation through stdin.

0: The calculator does not take in three parameters through stdin.

##### Feature Support
For each of the required numerical bases (decimal, hexadecimal, and binary)

1: The calculator can correctly parse inputs of this type and represent them as u32s.

0: A bug exists in parsing the numerical base as an input.

For each of the required operations (AND, OR, XOR)

1: Users can input the mathematical symbol, or english representation for the operation, and the calculator correctly performs said operation.

0: The operation is not parsed correctly.

##### Unit Tests
For each of the required numerical bases and operations (hexadecimal, decimal, binary, AND, OR, and XOR)

1: Comprehensive unit test coverage exists to exercise the feature

0: Unit test coverage is missing, or insufficient.

##### Scalability
2: The calculator was written with scalability in mind, and expanding functionality would require minimal changes.

1: Minor changes would need to be made in order to implement more features into the calculator.

0: The calculator was not implemented with scalability in mind, and the entire script would need to be refactored in order to append more features.

### Points
Correctness Score: XX/31

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

Code Quality Score: XX/4

## Summary
Total Score: XX/39