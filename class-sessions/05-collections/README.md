# Collections
The purpose of this worksheet is to boost your comfort level with the common 
collections (Vectors, Strings, and HashMaps) in Rust. For a few of the problems
covered in this worksheet (will be specified in the description), there will be an imperative and functional way to
implement your solution. We encourage you to develop both implementations to
gain familiarity with both methods. This practice will be particularly helpful
as you begin to work on the assignment, where you will be required to write
both implementations.

## 1. Intro to Vectors
### 1.0. Fibonacci Sequence
In `vector.rs`, the first function to implement is `get_fibonacci`, which takes in a usize, `fibonacci_size` and outputs of fibonacci sequence, as a vector, with the appropriate size.

Hint: To implement the functional solution, check out the [scan method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan).

You can test your fibonacci sequence with the following command:
```bash
cargo test vectors::tests::test_get_fibonacci
```
### 1.1. Binary Search
Your next task is to implement a binary search (DSA throwback) in the `binary_search` function of `vector.rs`. This function takes in a vector, `arr`, and a `search_val` and turns the index of `search_val` in `arr`. For those that need a referesher, check out [this link](https://www.geeksforgeeks.org/binary-search/).

You can test your binary search with the following command:
```bash
cargo test vectors::tests::test_binary_search
```
### 1.2. Filter Even Numbers
In `vector.rs`, implement `filter_even_numbers`, which takes in a vector, `arr`, of u8s and returns a vector that removes all the even numbers from the inputted array. This function can be written functionally and imperatively - try both!
(Hint for the functional implementation - check out the [filter method](https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.filter))

You can test your even number filter with the following command:
```bash
cargo test vectors::tests::test_filter_even_numbers
```
### 1.3. Longest Increasing Subsequence
In `vector.rs`, implement `get_longest_increasing_subsequence_len`, which outputs the length of the longest monotonically increasing subsequence within the inputted vector, `arr`.

Example:
```rust
// The longest subsequence is index 4 -> 8.
let arr = vec![1, 2, 3, 2, 1, 2, 3, 4, 5];
assert_eq!(get_longest_increasing_subsequence_len(arr), 5);
```

Hint: This function can be written completely with functional functions (in one line!). Check out [zip](https://doc.rust-lang.org/beta/std/iter/fn.zip.html), [map](https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.map), and [fold](https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.fold) for some starting points! Also, feel free to start with a functional solution, and then transition to an imperative solution.

You can test this function with the following command:
```bash
cargo test vectors::tests::test_get_longest_increasing_subsequence_len
```

## 2. Intro to Strings
### 2.0. Merge Strings
Given, a vector of strings, return a single String that merges each element together. For example, `vec!["hello", " ", "world", "!"]` would become `hello world!`. Implement this function in `merge_strings` within `strings.rs`.

Hint: To implement this functionally, take a look at the [fold method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold).

You can test this function with the following command:
```bash
cargo test strings::tests::test_merge_strings
```
### 2.1. Count Letter
Next up is implementing `count_letter` in `strings.rs`. This function takes in a String, word, and a char, letter, and outputs the number of times letter appears in word.

You can test this function with the following command:
```bash
cargo test strings::tests::test_count_letter
```

### 2.2. Selectively Capitalize
Your final string task is selectively capitalize. In `selectively_capitalize` in `strings.rs`, you will be given a String, input, and a vec, `idx_to_capitalize`. You should return a new string that capitalizes each char in `input` corresponding to the specified char indexes in `idx_to_capitalize`. For example, `selectively_capitalize(String::from("hello"), vec![1, 3])` should yield `hElLo`.

You can test this function with the following command:
```bash
cargo test strings::tests::test_selectively_capitalize
```

## 3. Intro to HashMaps
### 3.0. Most Common Word in Text
Now time for HashMaps. First up is to determine the most common word that appears in a given text. You should implement this in `get_most_common_words` within `hashmap.rs`. A few example texts have been provided in the `poems` directory for you to test out. 

Hint: Make sure you remove all punctuations are normalize between uppercase/lowercase.

Note: Currently, all texts have been selected to guarantee there is only one max word. How would you modify your code/functions to account for the possibility that there could be a tie between two words to be the most common?

You can test this function with the following command:
```bash
cargo test hashmap::tests::test_get_most_common_words
```

### 3.1. Unique Characters
You've made it to the final task! Nice and simple, all you need to do is, in `get_unique_characters`, take in an input string and return a vector of chars that represent all the chars that only appear once within the inputted String. For example, `None` should return `vec!['o', 'e']`.

You can test this function with the following command:
```bash
cargo test hashmap::tests::test_get_unique_characters
```