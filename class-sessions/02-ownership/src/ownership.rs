#![allow(dead_code, unused_variables)]

pub fn stack() {
    let x = 5;
    let y = add_one(x);

    println!("x: {}, y: {}", x, y); // prints 5, 6
}

fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn heap() {
    let arr = Box::new([0; 1_000_000]);
    // drop(arr); This doesn't work, so Rust will manually deallocate the memory for us when a
    // variable leaves scope, right?
    println!("{}", arr[0]); // Undefined behavior when we try to use freed memoory!

    // Aside: What is a scope?
    let s = "Hello";
    {
        let s = "World";
    }
    println!("{s}") // prints "Hello"

    // so arr and s get deallocated, and all is good!
}

// not quite...
pub fn ownership() {
    let foo = Box::new([0; 1_000_000]);
    let bar = foo;

    // drop(foo)
    // drop(bar)
    //
    // How do we avoid this? Each variable has a single owner, and we only deallocate when the
    // owner goes out of scope - in this case, ownership has passed to bar, so bar is the only
    // variable that should get dropped
    //
    // drop(bar) gets called here
}

pub fn moves() {
    let lebron = "Lebron James".to_string();
    // let bronny = add_suffix(lebron);
    let bronny = add_suffix(lebron.clone()); // if we want to do this, we need to clone lebron
    println!("{}'s son is {}", lebron, bronny);
}

fn add_suffix(mut name: String) -> String {
    // This does 3 things:
    // 1. Creates a new larger heap allocation
    // 2. Copies the contents of name into the new allocation
    // 3. Frees the old allocation
    //
    // This makes lebron invalid!
    name.push_str(" Jr.");
    name
}
