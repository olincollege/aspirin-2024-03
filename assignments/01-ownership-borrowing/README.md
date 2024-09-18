# Assignment 1: Ownership and Borrowing

In this assignment, you will get the chance to explore the Rust ownership and borrowing model, implement some functions that explore some of its properties, and analyze the trade-offs that come from using it.

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
git switch -c assignment-01
```
### 0.2 Read the Rubric

Assignments from this point forward will have rubrics. They are there to give you an idea of how your work will be assessed, what is expected of you, and what you can expect to get feedback on. You can complete the assignment without reading it, but we strongly recommend that you read it to get a sense of how assignments are graded.

## 1. Taking Ownership

This problem will allow you to dip your feet into writing simple functions using the Rust type system. In these examples, you are given the function name and docstring describing the intended behavior, and will have to complete the function signature and body.

The functions are:
- `takes_ownership_and_prints`
- `first_letter`
- `get_name`
    - This function uses a `type Student`, which is just an alias for a tuple of `String, i32`
- `slice_sum`
- `find_in_string` - you may find some functions in the `std::str` module to be useful

For `find_in_string`, you may find you run into an error - see if you can explain why that error occurs in `analysis.md`. We will learn how to work around this next week, but you can also look into lifetimes and see if you can get this example to comple on your own!

Unit tests are included for these functions, and can be run with `cargo test`

## 2. Please Borrow Responsibly

Building on the previous section, we're going to practice working with mutable references too in `mutables.rs`.

Implement these three functions:
- `increments`
- `swap`
- `strip_vowels_and_digits`
    - For this function, we've provided some helper functions you should use, as they purposely have different function signatures that you can practice calling.

Unit tests are included for these functions, and can be run with `cargo test`

## 3. `Link`ed for Life

Now, we're going to put it all together and use what we know to make a linked stack - essentially a linked list, but we push and pop from the head to make it a little simpler.

In `singly_linked_list.rs`, you'll find a LinkedStack struct that you'll need to fill in, and function signatures for the 4 functions you'll need to implement for LinkedStack - `new`, `push`, `pop`, and `drop`.

Unit tests are included for these functions, and can be run with `cargo test`

## 4. Doubly Linked Stack?

Great! Now that you've got a singly linked list, making it doubly-linked shouldn't be too hard, right? 

We've given you the same starting platform in `doubly_linked_list.rs`, and this should have the same API as the singly linked version - `new`, `push`, `pop`, and `drop`, pushing and popping from the head again.

Hmm...that didn't seem to work either (using the Rust we've learned so far!) Explain why this violates the ownership and borrowing rules in `analysis.md` (or if you manage to find a sneaky workaround - explain why that workaround was necessary)

Come up with another example of something that may be difficult to implement in Rust due to ownership/borrowing rules.

## 5. Submission

To submit this assignment, add and commit your changed files (don't forget to format and lint!). These should be `analysis.md` and some files in the `src` directory. Be sure to write a reasonably clear commit message.

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Assign the team "olincollege/aspirin-2024-03-assistant" as reviewers.


