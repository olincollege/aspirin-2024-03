# Mini Project 0 - JQ
Welcome to your first big assignment - JQ!

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
git switch -c assignment-04
```
### 0.2 Read the Rubric

The rubric is given to you to provide you an idea of how your work will be assessed, what is expected of you, and what you can expect to get feedback on. You can complete the assignment without reading it, but we strongly recommend that you read it to get a sense of how assignments are graded.

NOTE: Since no starter code is provided for this assignment, the assessment will be largely open-ended and feature-focused. To maintain the spirit of this assignment, if you find certain bullets to be overly constraining in your implementation (within reason), feel free to reach out to the teaching team for a discussion. The intention of the rubric is to offer guidance and restrictions on the features you are implementing, not to overly constraint your architecture.

## 1. Design Requirements
Contrary to past assignments, we're giving you a lot more freedom in how you want to develop your software for this assignment. This means no starter code, no syntax requirements, and no unit tests (provided). But rather, we're giving you a set of required features that your tool must support.

These requirements will be split into the four main components of JQ:
1. Command Line Interface (CLI)
2. Supported Filters
3. Built-In Functions
4. Formatted Printing

### 1.0 Command Line Interface
At its base, the jq command line interface has 2 required arguments:
1. Filter String
2. Path to JSON File

Additionally, there are several arguments that can be specified to modify your tools behavior. The following are the arguments you are required to support (followed by the default value):
- `color-output` (True)
- `monochrome-output` (False)
- `sort-keys` (False)
- `indent` (2) - this value must be in the range of 0-7 inclusive
- `compact-output` (False)

Each of these arguments modify the formatting used to print. More verbose descriptions and examples are provided in Section 1.3 Formatted Printing.

Additionally, if conflicting arguments are provided, an error should be raised to the user to prevent confusing behavior. Ex. a user should not specify `color-output` and `monochrome-output` since both cannot be honored at the same time. Same with `compact-output` and `indent`.

### 1.1. Supported Filters
This section will outline all the filters you are required to support. This [link](https://jqlang.github.io/jq/manual/) is the man page of JQ and can offer more details about each of these, along with additional features that you may choose to implement.

NOTE: Each of the displayed examples in this section are not displayed with correct colors (I was too lazy to figure out how to color markdown text). 

In short, the filters you are required to implement are:
1. Identity Filter
2. Object-Identifier Index
3. Array Index
4. Array Slice
5. Pipe
6. Array Iterator

#### 1.1.0 Identity Filter
The most basic filter! Represented as a `.`, this filter takes in the inputted JSON and outputs the same exact values.

Since JQ pretty prints by default, this is a great way to pretty print JSON data.

Example:
```bash
jq . sample_data/all_types.json
```
would output:
```
{
  "fizz": "buzz",
  "baz": null,
  "fuzz": true,
  "bizz": 22.0,
  "biz": 42,
  "fizzes": [
    "buzz",
    null,
    true,
    22.0,
    42.0
  ]
}
```

#### 1.1.1 Object-Identifier Index
This filter is used to extract specific attributes (keys) from JSON object (dictionary). This is done through the `.<KEY_NAME>` syntax. 

Example:
```bash
jq ".fizz" sample_data/all_types.json
```
would output:
```
"buzz"
```

#### 1.1.2. Array Index
This filter is used to extract specific elements within a an array. It is formatted as follows: `.[<number>]

Example:
```bash
jq ".[0]" sample_data/array.json
```
would output:
```
"one"
```

#### 1.1.3. Array Slice
Similar to the previous feature, but now, you can extract a subsequence from an array. It follows the following format: `.[<start_number>:<end_number>]`

Example:
```bash
jq ".[0:2]" sample_data/array.json
```
would output:
```
[
  "one",
  "two"
]
```

#### 1.1.4. Pipe
While this is not explicitally a filter, the `|` operator allows you to pass the output of one filter to another.

Example:
```bash
jq ".fizzes | .[1]" sample_data/all_types.json
```
would output:
```
null
```

#### 1.1.5 Array Iterator
If no arguments are passed within .[], then an iterator is created over the inputted array (or an error if it is not an array). This allows you to run the same operation on multiple elements.

NOTE: An iterator is not an array (meaning, when it gets printed, the elements are not printed within an array, but as separate elements).

Example:
The following code will fail because .name cannot be called on a list, but only on individual elements:
```bash
jq ".name" sample_data/football.json
```
Correct command:
```bash
jq ".[] | .name" sample_data/football.json
```
would output:
```
"Leo Lightning"
"Maximus Defender"
"Sophie Swift"
```

### 1.2 Built-In Functions
The traditional JQ tool also provides some nice built-in functions to allow users to run certain computations on their data structures. These can also be nested within filters through pipes.

For each of these operators, it is up to you on what is considered a "valid" input and can be computed vs. what should throw an error.

In all, you must implement the following built-in functions:
1. add
2. length
3. del

#### 1.2.0 add
Given an array of values (not an iterator), `add` returns the sum of those values. It is up to you to determine how to handle summing different datatypes.

Example:
```bash
jq ". | add" sample_data/array.json
```
would output:
```
"onetwothree"
```

#### 1.2.1 length
Given a value, `length` returns the length of the input. Once again, length can be how you define it for each datatype.

Example:
```bash
jq ". | length" sample_data/array.json
```
would output:
```
3
```

#### 1.2.2 del
Given an array or a object, delete the inputted value and return the remained of the element. For arrays, the syntax is `del(.[<indexes>])` and for objects: `del(.<key_name>)`.

Example:
```bash
jq ". | del(.fizzes)" sample_data/all_types.json
```
would output:
```
{
  "fizz": "buzz",
  "baz": null,
  "fuzz": true,
  "bizz": 22.0,
  "biz": 42
}
```

### 1.3 Formatted Printing
Finally, the most important part of any query tool is the output format. In total, you need to support the following customizations to the output format:
1. Colored Printing
2. Monochrome Printing
3. Sorted Keys
4. Customizeable Indents
5. Compact Output

#### 1.3.0 Colored Printing
Traditional JQ tools utilize the [VT100/ANSI Escapes](https://en.wikipedia.org/wiki/ANSI_escape_code) for formatted printing, which rust natively supports! The default escape codes can be found [here](https://jqlang.github.io/jq/manual/#colors).

Example:
```rust
println!("\x1b[34m{}\x1b[0m", "null");
```
would output the text `null` in blue text. `\x1b[34m` represents the start of blue text and `\x1b[0m` resets the terminal to print normally again.

```rust
println!("\x1b[31;1m{}\x1b[0m", "Hello, World!");
```
would output the text `Hello, World!` in bright red text.

Another feature of JQ is the ability to customize each of these formats. This is done by reading the `JQ_COLORS` environment variables. `JQ_COLORS` will be a set of colon-separated values where, in order, entries correspond to:
1. color for null
2. color for false
3. color for true
4. color for numbers
5. color for strings (including quotations)
6. color for arrays (just the brackets and commas)
7. color for objects (just the brackets and commas)
8. color for object keys

For each entry, there will be two value, separated by a semi-colon. The first value represents any special formatting (ex: bright, dim, underscore) and the second value represents the color (ex: red, blue).

The default entry for `JQ_COLORS` is `"0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34"`. This translates to the following colors:
null: no formatting, gray
false: no formatting, white
true: no formatting, white
numbers: no formatting, white
strings: no formatting, green
arrays: bright, white
objects: bright, white
object keys: bright, blue

which looks like the following:
![Color Print Image](/assets/color_print.png)

By default, your JQ implementation should color print, but it can also be set through the `color-output` CLI option.

#### 1.3.1 Monochrome Printing
The exact opposite of colored printing, monochrome printing features no colors at all and uses the default terminal format for all printing.

Example:
![Monochrome Print Image](/assets/monochrome_print.png)

This can be specified through the `monochrome-output` CLI option.

#### 1.3.2 Sorted Keys
This is specified through the `sort-keys` CLI option. This option will recursively sort all objects that will be printed by key. 

Example:
```bash
jq . sample_data/all_types.json --sort-keys
```
would output:
```
{
  "baz": null,
  "biz": 42,
  "bizz": 22.0,
  "fizz": "buzz",
  "fizzes": [
    "buzz",
    null,
    true,
    22.0,
    42.0
  ],
  "fuzz": true
}
```

#### 1.3.3 Indents
The user can also customize the number of spaces used per indent. By default, 2 spaces will be used (max of 7 and min of 0)

Example:
```bash
jq . sample_data/all_types.json --indent 7
```
would output
```
{
       "baz": null,
       "biz": 42,
       "bizz": 22.0,
       "fizz": "buzz",
       "fizzes": [
              "buzz",
              null,
              true,
              22.0,
              42.0
       ],
       "fuzz": true
}
```

#### 1.3.4. Compact Output
This is the opposite of pretty printing, where no newlines or extra whitespace is inserted between elements. This option can be specified through the `compact-output` argument in the CLI.

Example:
```bash
jq . sample_data/all_types.json --compact-output
```
would output
```
{"fizz":"buzz","baz":null,"fuzz":true,"bizz":22.0,"biz":42,"fizzes":["buzz",null,true,22.0,42.0]}
```

## 2. Helpful Documentation
This section will highlight some helpful rust tools that will make your assignment easier!

### 2.0 Serde!
We are *NOT* requiring that you read in a JSON file from scratch. Rust has a really handy crate called [`serde`](https://serde.rs) (*ser*ialize and *de*serialize) that makes parsing JSONs really easy!. In particular, [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) was specifically designed for efficiently parsing JSONs. Simple Example:

```rust
use serde_json::Value;

fn main() {
  let file = File::open("sample_data/all_types.json").unwrap();
  let reader = BufReader::new(file);
  let json_obj: serde_json::Value = serde_json::from_reader(reader).unwrap();

  println!("{}", json_obj);
}
```
would output
```
{"fizz":"buzz","baz":null,"fuzz":true,"bizz":22.0,"biz":42,"fizzes":["buzz",null,true,22.0,42.0]}
```
Using the debug print to inspect the types, we see the following: 
```
Object {"fizz": String("buzz"), "baz": Null, "fuzz": Bool(true), "bizz": Number(22.0), "biz": Number(42), "fizzes": Array [String("buzz"), Null, Bool(true), Number(22.0), Number(42.0)]}
```

`serde_json` can also be used to parse directly into more structured data, like HashMaps, but for JQ, we don't have the freedom of knowing our types a priori, this example parses the values into a generic [`serde_json::Value`](https://docs.rs/serde_json/latest/serde_json/enum.Value.html) object, which is an enum of all possible datatypes stored in a JSON.

Hint: Take a look at the `serde_json` documentation, there might be some functions that will make your life very easy in this assignment (printing?). 

### 2.1 Environment Variables
The rust standard library includes `std::env` which allows you to directly read environment variables. This will enable you to read the `JQ_COLORS` variable (if it exists) to configure your colored printing. 

Sample Code:
```bash
export test_var=10
```

```rust
use std::env;

fn main() {
  let test_var = env::var("test_var").unwrap();
  println!("environment variable value: {}", test_var);
}
```
would output
```
environment variable value: 10
```

## 3. Submission
To submit this assignment, add and commit your changed files (don't forget to format and lint!). These should just be the files in the src directory. Be sure to write a reasonably clear commit message.

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Assign the team "olincollege/aspirin-2024-03-assistant" as reviewers.