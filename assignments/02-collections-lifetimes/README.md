# Assignment 2: Collections and Lifetimes

## 0.Setup

### 0.1 Sync and Branch your Repo

Make sure that you are at the latest change in your repo by running the following commands:

```
$ git switch main
$ git pull
$ git pull upstream main
$ git push
```
If you run into issues performing the above operations, ask for help on Discord.

Once you have done this, create a new branch for this assignment:
```
git switch -c assignment-02
```
### 0.2 Read the Rubric

The rubric is given to you to provide you an idea of how your work will be assessed, what is expected of you, and what you can expect to get feedback on. You can complete the assignment without reading it, but we strongly recommend that you read it to get a sense of how assignments are graded.

## 1. Getting Your *Life* Together
Life as a programmer is hard. You've been spending too much time coding that you
forgot to check-in on one of your friends, causing them to get mad at you. This
exercise will walk you through the three exercises to help mend your friendship.

NOTE: For each function (and struct) definitions, the lifetimes have been omitted, hindering the code from compiling. Your job is to add in lifetime annotations, wherever necessary, to ensure the functions compile and output the correct lifetime.

### 1.0. Banana Split
As with everything in life, ice cream is always the solution.

For the first part of this assignment, your job is to implement `split_string` in `life.rs`. This functions takes in a string slice, `string`, and a delimeter, `delimeter`, and returns a vector of string slices by splitting `string` based on `delimeter`.

Example:
```rust
let split_str = split_string(&"Hello, world!", &", ");
assert_eq!(split_str, vec!["Hello", "World!"]);
```

### 1.1. Settling Your Differences
After savoring your wonderful ice cream, you've decided to initiate conversations with your friend. To start, you want to settle all of your differences, but, in order to do this, you need to figure out all the differences that exist.

In `find_differences` your job is to find unique words between the two inputted strings, `first_string` and `second_string`. To make the information more digestible, you will create an instance of `Differences` (initial code, excluding the lifetimes, have been provided for you) to show the words that are only in the first string and only in the second string. 

A word is defined to be "only in one string" if it never exists within the second string (even as a subset of another word within that string).

For example, if the two strings were `pineapple` and `apple`, apple would not be considered unique since it exists within `pineapple`, but `pineapple` would be considered unique since it does not exist within `apple`.

Rust Example:
```rust
let differences = find_differences(&"pineapple pen", &"apple");
assert_eq!(differences, Differences {
    only_in_first: vec!["pineapple", "pen"],
    only_in_second: Vec::new(),
});
```

### 1.2. Coming Together
Finally, you and your friend have settled your differences and are coming together. As a sign of peace,
you decided to create a fun name to label the unison. This name is a string that interweaves both of your
first names in a creative way. You should implement this in `merge_names` in `life.rs`. The interweaving algorithm you settled on is as follows:

1. Create an empty string buffer.
2. Starting with `first_name`, append each char in the name to the empty buffer until you hit a vowel. If the starting letter is a vowel, append that letter and then continue until you hit the next vowel.
3. When you hit a vowel, do the same thing to `second_name`, stopping where you hit the next vowel. 
4. Continue steps 2 and 3 until you have append all chars in both names to your empty buffer.
5. Return the generated string.

Example 1:
Merging `alex` with `evan`:
1. Iterating over `alex`, we first collect `al`.
2. Then, iterating over `evan`, we collect `ev`.
3. Going back to `alex`, we collect `ex`.
4. Finally, we get `an` from `evan`.
5. In all, this gives us `alevexan`.

Example 2:
Merge `hot` with `sauce`:
1. Starting with `hot`, we collect `h`.
2. Then, from `sauce`, we collect `s`.
3. Resuming on `hot`, we get `ot`.
4. Since the first word has been completed, we can append the remaining characters from `sauce`, which are `auce`.
5. This gives us `hsotauce`.

Consult the test cases in `test_merge_names` for more examples.

## 2. eLEET Code
Are you ravaged by interview season already? 
Too much ASPIRIN homework to grind leetcode problems?
Well, I've got the perfect solution for you!
This section goes over 4 leetcode style problems that will guarantee you ace your next rust (no guarantees made).

NOTE: Function signatures are provided for each of the 4 problems within this section. However, some of them are incomplete!
As rust-analyzer nicely points out, some of the references are missing lifetime annotations. When appropriate, you should
mark the lifetimes of each parameter to be as long as possible for an external module to use.

### 2.0. Longest Equal Sequence
First up is getting the length longest sequence of equal values within an inputted vector. To ensure that your function gets used as much as possible, you should use generics in your function signature (the types were intentionally omitted for you to fill out). Your function should support any rust type that implements the `std::cmp::PartialEq` trait. For a refresher on generics, check out [this section](https://doc.rust-lang.org/book/ch10-01-syntax.html) from the rust book.

For your implementation, we are requiring that you implement it in two different ways:
1. Using prescriptive programming where you are explicitly directing what the code should do. This should be done in `longest_equal_sequence_prescriptive` in `leet_code.rs`.
2. Using functional programming, where you are describing the behavior you want using some of the built-in functions on iterators in rust. This should be done in `longest_equal_sequence_functional` in `leet_code.rs`.

Example 1:
```rust
// The longest sequence is the sequence of 3 4s at the end.
let longest_equal_sequence_len = longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 5, 5, 4, 4, 4]);
assert_eq!(longest_equal_sequence_len, 3);
```

Example 2:
```rust
// The longest sequence (ties) are the 2 consecutive gs and 2 consecutive zs.
let longest_equal_sequence_len = longest_equal_sequence_prescriptive(&vec!['a', 'b', 'e', 'g', 'g', 'z', 'z']);
assert_eq!(longest_equal_sequence_len, 2);
```

### 2.1. Paranthesis Matching
This problem is stolen from the [2021 Advent of Code Day 10 problem](https://adventofcode.com/2021/day/10). In `is_valid_paranthesis`, you will take in a ref string, `paranthesis`, and need to determine whether the inputted string is a valid combination of open and closed paranthesis. This means that for every open paranthesis ('(', '{',, '[') has its corresponding closed paranthesis (')', '}', ']') and that the closing paranthesis appear in the correct order.

Examples:
'()' is a valid paranthesis since each open paranthesis has a close paranthesis and they are in the correct order.

'}{' is not valid. While each open paranthesis has a corresponding close paranthesis, they do not appear in the correct order.

'[][' is not valid since not every open paranthesis has a corresponding close paranthesis.
match paranthesis

### 2.2. Longest Substring
Next up is finding the longest substring between two strings. `longest_common_substring` takes in two parameters, `first_str` and `second_str`, both as reference strings, and returns a reference to the longest shared substring between the two strings.

Example:
```rust
let common_substring = longest_common_substring(&"apple", &"pineapple");
assert_eq!(common_substring, "apple");
```

### 2.3. Longest Substrings
Very similar to problem 2.2, but this time, you are given a slice of reference strings of arbitrary length instead of just two strings. 


## 3. A Wide *Array* of Problems
Didn't you love QEA1? Feeling a little rusty in terms of matrix operations? Well I've got just the solution for you. Well, I guess you're coming up with the solutions so, better phrased, I've got just the problem for you.

### 3.0. Product-Matrix Fit
First up, is at the intersection of P&M and QEA: the dot product! For those needing a refresher, take a look at [this page](https://www.mathsisfun.com/algebra/vectors-dot-product.html) for how to calculate the dot product of 2 1-dimensional vectors. For this problem, we are requiring that you implement your dot product in two ways:
1. One prescriptive solution that explictly details the code that should be run. This should be done in `dot_product_prescriptive` in `matrices.rs`.
2. One functional solution that just uses functionally programming in rust to describe the operations you would like to run. This should be done in `dot_product_functional` in `matrices.rs`. 

In both cases, the functions return a Result of either the result of the vectors, if it can be performed, or an error if it cannot be computed mathematically. The two errors that you should be able to return are:
1. Return `MatrixError::EmptyVector` if either of the inputted vectors are empty.
2. Return `MatrixError::DimensionMismatch` if the lengths of the two vectors are not the same.

### 3.1. Row-mantic Results
(This title was chatgpt-generated, I take no responsibility for how bad it is)

The final coding exercise is to implement a matrix multiplication function, `multiply_matrices` that takes in two 2d vectors, `vec1` and `vec2`, and returns a Result, the result of the matrix multiplication if it can be computed, or an error if it cannot be. You can assume that all inputted matrices are 2 dimensional and that you will always return a two dimensional vector.

For those needing a refresher, consult [this webpage](https://www.mathsisfun.com/algebra/matrix-multiplying.html).

These are the errors you should account for:
1. Return `MatrixError::EmptyVector` if either of the inputted vectors are empty.
2. Return `MatrixError::InvalidShape` if either of the matrices are not squares.
3. Return `MatrixError::DimensionMismatch` if the two matrices cannot be multiplied together based on the size of their rows and columns.

## 4. Mirrors on the Wall
To conclude this assignment, we want you to give a short reflection. In parts 2.0 and 3.0, we asked you to implement the same function with prescriptive programming and functional programing. In `reflection.md`, we would like you to give a short reflection on both techniques and your experience with each of them. Some topics that you may include are: 
1. General readability of each implementation
2. Ease of implementation from a software development standpoint
3. Scalaibility of each implementation
4. Your preference towards one style or the other

This reflection does not need to be lengthy. We are only expecting, at minimum, 4 sentences that show significant considerations and evaluations of each method.


## 5. Submission
To submit this assignment, add and commit your changed files (don't forget to format and lint!). These should just be the files in the src directory. Be sure to write a reasonably clear commit message.

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Assign the team "olincollege/aspirin-2024-03-assistant" as reviewers.