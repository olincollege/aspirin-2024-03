# ASP(i)RIN Assignment 3

In this assignment, you'll practice the skills you've learned so far, especially traits and polymorphism, by building a simple Grep clone. You'll also have the chance to stretch your Rust architecture skills a bit, instead of implementing our pre-built functions.

## 0. Setup

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
git switch -c assignment-03
```

### 0.2 Read the Rubric

The `rubric.md` file contains the rubric for this assignment. You should take a look at the rubric to get a sense of how your submission will be evaluated.

## 1. Grep

For this assignment, you'll reimplement a subset of the functionality of `grep`, a common command line tool that takes in an input, and prints any lines that contain a certain pattern. To practice your polymorphism skills, we'll have a few different modes at each stage of the program that you'll be expected to define a shared trait for and use in your code.

(Could these be enums instead? Yes. Would the code be simpler? Probably. But then you wouldn't get to practice your traits! That's no fun :( ) 

A departure from previous assignments - you should also write your own unit tests for your functions (Hint: you can use fakes/mocks that also implement your traits!)

### 1.1 CLI

Your grep clone (just like the real grep) will be structured as a command line application that takes various flags. Some good news - we've taken care of this part for you through the `Args` struct, so the `Args::parse()` call in `main` will create an instance of `Args` from the command line input. If you `cargo run` the project right now, you'll see the help message for all the commands you'll eventually have to implement. We've used the `clap` crate to do this, which is the most popular CLI crate used today.

### 1.2 Input

The first stage is the input - just like the real `grep` your program should allow passing a file in as the search space - or if none is provided, use the standard input.

You should have a trait definining some common functionality that gets implemented by an input from a file or from stdin.

> Hint: you may want to look at the `std::io::BufRead` trait and the `lines` method!

### 1.3 Search String

Your program should also support the search string being either a regular literal string, or a regular expression - you'll probably want to use the `regex` crate for this, which you can add to your project with `cargo add regex`. Again, you should have a trait that encapsulates the common functionality!

### 1.4 Optional Flags

There are two additional optional flags that your program should support:
- `ignore_case` - as you might guess, disregard letter case when finding matching lines
- `invert_match` - print the lines that do NOT contain the search pattern

This doesn't necessarily need to utilize polymorphism - just some additional wrinkles!

### 1.5 Output

Lastly, your program should support at least two modes of output - one plain output, that just prints any matching lines, and one colored output, where the matching substring is printed in a color specified by a command line flag. There are lots of crates that make colored printing relatively simple - `colored` is a simple and popular one that we've tested. Use traits here as well, please!

> Hint: taking advantage of the `std::io::Write` trait that is also implemented by `Vec` will make it much easier to test these functions!

### 1.6 Error Handling

Ahh, error-handling. Simultaneously one of the great things in Rust, because you have to handle all of them, and also one of the annoying ones, because you have to handle them. For a small-scale app like this, we'd recommend the [`anyhow`](https://docs.rs/anyhow/latest/anyhow/) crate, which more or less propagates all the errors up to the main function, prints, and exits. It's a little crude, and your program often won't know how it failed, but for an app like this in which an error often indicates malformed user input and we want to communicate that and end the program, it works.

### 1.7 Extras

If you want to take your `grep`-fu to the next level, here are some other fun things you can try to implement:

1. Make your program as close to zero-copy as possible - that is, taking data from the source and only using references to it, instead of copying it around from place to place. This can be difficult and requires satisfaction of the borrow checker, but can offer significant performance gains when you start working at a larger scale

2. Add some additional features from the original grep! You can check out its man page for more details - one interseting one might be the context printing feature, where you print a certain amount of lines before/after each matching line as specified by a CLI flag - you might need to modify the `Args` struct for this - the `clap` documentation is unfortunately pretty terrible, but [this](https://docs.rs/clap/latest/clap/_derive/index.html) is probably where you'd want to look.

3. Anything else you can think of! We'd love to see what cool things you can implement.

If you want to see a Rust implementation of grep that's super-fast, check out [`ripgrep`](https://github.com/BurntSushi/ripgrep)

Bonus points may be on the table? (run your idea by us first if it's not one of the examples!)

## 2. Submission

To submit this assignment, add and commit your changed files. These should be some files in the `src` directory. Be sure to write a reasonably clear commit message.

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Assign the team "olincollege/aspirin-2024-03-assistant" as reviewers.
