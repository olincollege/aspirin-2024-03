
# ASP(i)RIN Assignment 5

In this assignment, you'll use the concurrency skills you've built so far to build a general thread pool implementation, then use that thread pool to write a program that concurrently sorts 

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
git switch -c assignment-05
```

### 0.2 Read the Rubric

The `rubric.md` file contains the rubric for this assignment. You should take a look at the rubric to get a sense of how your submission will be evaluated.

## 1. Thread Pool

For this assignment, you'll be implementing a thread pool. A thread pool is a collection of worker threads that can be assigned individual tasks to be executed concurrently by some supervising process. Some advantages of this approach as opposed to spawning a new thread for each task include avoiding the expensive setup/teardown of a new thread for each task, as well as preventing system overload (for instance, if you are running a web server)

### The Code

You should write your thread pool in `src/thread_pool.rs` - we've given you a little bit of starter code and some (incomplete) function signatures, but most of the architecture is up to you!

Your thread pool should:
- Take a number as an input to the `new`, and create that number of threads (0 threads should be an error)
- The `execute` function should take some task and pass it to an available thread to execute
- The `get_results` function should return any available results from the tasks sent to the threads
	- You should structure your code such that the type of the result is as general as possible, although you can assume that every job in a particular thread pool will return the same type
- The thread pool as a whole should be as abstract as possible, and not tied at all to the merge sort to come\
- Don't forget to include unit tests!

## 2. Merge Sort

To test our thread pool, we'll use it to write a concurrent version of merge sort, a popular, relatively simple sorting algorithm that lends itself quite well to concurrency. Feel free to write this code in `main.rs`, or make a different file. Here are some links you may find helpful. 
- [GeeksForGeeks](https://www.geeksforgeeks.org/merge-sort/) What is Merge Sort?
- [GeeksForGeeks](https://www.geeksforgeeks.org/merge-k-sorted-arrays/) How to do a k-way merge (hint: this might be useful!)

Exactly how you implement it is up to you, so long as you use concurrency to equally split the tasks.

### Benchmarking

Using a large enough array (say around 10 million elements), try sorting it with different numbers of threads between 1 and 100. In `benchmark.md`, say which number of threads resulted in the fastest runtime, and briefly compare and contrast the pros and cons of more/less threads. 

We'd recommend a benchmarking crate like `criterion` for this, although you can use something as simple as `std::time`

## 3. Submission

To submit this assignment, add and commit your changed files. These should be some files in the `src` directory. Be sure to write a reasonably clear commit message.

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Assign the team "olincollege/aspirin-2024-03-assistant" as reviewers.

