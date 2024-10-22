# ASP(i)RIN Assignment 4

Welcome to the world of Rust and networking! As a budding developer at Aspirin Eats, a food delivery service that's quickly becoming the talk of the town, you'll be building the foundation of our online ordering system. Your mission: create a server that will serve as the backbone of our API, handling placing and removing orders from the system. You'll implement basic HTTP request and response handling, API request routing, and to top it off, you'll also be writing a reverse proxy to ensure the security of our primary origin server.  

## 0. Setup

### 0.1 Sync and Branch your Repo

Make sure that you are at the latest change in your repo by running the following commands.
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

## 1. Origin Server

Before we get into the proxy business, we need to first write the origin server that will allow us to actually interact with our order database. Luckily, you've got a few other developers working with you on this project, and the food and database teams have already taken care of creating Rust data structures to represent our menu in `food.rs`, as well as creating a library with an API you can use the interact with the database storing all of our orders in `db.rs`; you can look at those files (or the generated documentation) for more info on how to use that code.

Some of the key things to note are:

- The `AspirinEatsDb` type is essentially just a handle to the database object that you can call methods on in your sever implementation. You can create a new one at a particular file location with the `from_path` method, which will either create a new database at the given path, or load it if one already exists. You may also find it useful for testing to use `in_memory`, which allows you to quickly spin up a database within the program memory (hint hint).

- The `Order` type implements `From<OrderRequest>` - this means you can convert an `OrderRequest` that you create from user input into an order using `.into()`, and it will give you back an Order with the ID, total, and status fields automatically filled in.

- The `Order` type is also set up to allow you to convert to JSON with the `to_string()` method, and an `OrderRequest` can be created from JSON with the `OrderRequest::from_str` method.

  

### An HTTP Primer

The origin server should be structured as a REST API - that is, we should be able to make HTTP Requests to interact with it. For this assignment, we'll be using HTTP 1.1, as it's much simpler than later versions. The anatomy of an HTTP request goes like this:

1. Request line - this usually looks something like `GET /orders/15 HTTP/1.1` - you'll see the HTTP method (for this assignment, either GET, POST, or DELETE), the request target (the path of what resource you're trying to interact with), and the protocol, which should always be HTTP/1.1 for this assignment.

2. Headers - immediately following the first line, there will be some metadata about the request and where it's coming from. You can ignore this for this assignment.

3. Body - after the headers, there will be a `\r\n\r\n` sequence, and then the request body. Not all requests have a body, but if they do (for example, a POST request seeking to add a new order to the database), the body is where that request would live.

The HTTP Response is similar but slightly different - the first line will contain the status code and status response instead of request type and target; more information on HTTP Requests and Responses is available [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages).

We've given a partial implementation (unit tests are included for this one) of the HTTP Protocol in `http.rs` - you'll have to finish it by filling out all of the methods marked `todo!()`.


### The API

Ok - now that we've got some background on HTTP, now let's talk about the API we're implementing, which should be able to get, add, and remove orders from the database.


**Orders**

- Getting orders

	- A GET request to `/orders` should return a JSON list of all of the orders in the database in its body

	- a GET request to `/orders/{id}` should return a JSON representation of the order with the specified ID in its body

- Adding orders

	- A POST request to `/orders` should add the `OrderRequest` in the request body to the database

- Removing Orders

	- A DELETE request to `/orders` should remove all of the orders in the database

	- A DELETE request to `/orders/{id}` should remove the order with the specified ID

**Other**
If we get a request to the root (as in, no path or `/`), return a welcome message that says "Welcome to Aspirin Eats!"

If we run into an error (malformed input, path not not defined, trying to call an HTTP method not specified here, etc), the server should sent an HTTP response with the appropriate [status code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status) and with an error message.

  
### The Code
You've been given some starter code in `/bin/origin.rs` that gets a handle to the database object (make sure you set the `DB_PATH` variable to a convenient spot where the code will create the database file); using your new Rust networking toolbox, write the rest of the server so that it accepts new TCP connections to `localhost` on port 8080  (Real HTTP is usually routed through port 80, however on most systems ports 1-1023 are restricted to `root`, so port 8080 is often employed as an easier-to-use substitute for test applications like this), reads the HTTP Request path and body, performs the appropriate action based on the above spec, and sends back the appropriate HTTP Response. Don't forget to write unit tests! We'd recommend you do some thinking at the start of this assignment as to how you might structure your code to make writing tests easier (hint - where can you take advantage of things you've learned earlier in this course?)

#### Errors
Along the way, your code might fail! Don't forget that we have to handle all errors in Rust. We've stated and documented most reasonable error cases in `error.rs` - you should probably be catching most of these in your program and returning them where appropriate, and you can also add error cases you think are appropriate.

#### Running your server
You might find if you just type something like `localhost:8080/orders` with your server running that you're getting some data back! However, for testing the POST and DELETE endpoints, you might find it useful to instead use `curl`.

By default, running something like `curl 127.0.0.1:<port-number>/orders` will send a GET request. However, you can also use the `-X` flag to specify the http method and `-d` to add a body, so deleting an order might look like:
```
curl -X DELETE 127.0.0.1:8080/orders/1
```
And inserting an order might look like
```
curl -X POST 127.0.0.1:8080/orders -d '{"customer":"Amit","food":[{"Burger"{"bun":"Plain","patty":"Beef","toppings":["Lettuce","Tomato","Bacon"]}}, "Fries"]}'
``` 
> If you want to generate additional test cases, remember that you can always create a JSON representation of an `Order` using the `to_string()` method

## Reverse Proxy Server

Looks like you've gotten another feature request - turns out there's some security concerns with your server, so the CEO would like it to be hidden behind a *reverse proxy*. If you're familiar with a traditional proxy, where you, the client, has your internet traffic routed through another proxy server to protect your identity, a reverse proxy is what might exist on the server side; that is:

- The client sends a request to the reverse proxy server
- The proxy forwards this along to the origin server
- The origin server handles the request and sends the response to the proxy
- The proxy forwards the response along to the client

The end result of this being that you should be able to send an API request to the proxy server (using `curl` or another method) and the origin server should handle that command and return its response to the proxy without the requester having to know the address of the origin server.

We've placed a bit of starter code in `bin/reverse_proxy.rs`, mainly to parse command line arguments for the reverse proxy and origins server addresses; the rest is up to you! Don't forget to unit test this too; remember that you can inject your dependencies as traits like `Read` and `Write` in your functions so that you can pass in some simpler types like `Vec`s in your tests.

## 2. Submission

To submit this assignment, add and commit your changed files. These should be some files in the `src` directory. Be sure to write a reasonably clear commit message. Don't forget to lint and format!

Once you have committed your changes, push them to origin (your fork of the course repository) and open a pull request to your main branch. Submit the link to your pull request in Canvas.
