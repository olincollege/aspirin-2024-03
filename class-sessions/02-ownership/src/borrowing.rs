#![allow(dead_code, unused_variables, unused_assignments)]
// Only using moves sucks

pub fn moves_suck() -> [String; 2] {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let (s1, s2) = combine_and_print(s1, s2);

    [s1, s2]
}

fn combine_and_print(s1: String, s2: String) -> (String, String) {
    println!("{} {}", s1, s2);
    (s1, s2)
}

// Using references is much easier

pub fn references_are_better() -> [String; 2] {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    combine_and_print_ref(&s1, &s2);
    [s1, s2]
}

fn combine_and_print_ref(s1: &String, s2: &String) {
    println!("{} {}", s1, s2);
}

pub fn derefs() {
    let mut foo: Box<i32> = Box::new(5);
    let mut bar = *foo; // bar is its own 5
    bar += 1; // bar is now 6
    *foo -= 1; // foo is now 4

    let double_ref = &foo;
    let baz = **double_ref; // baz is a new copy of 4

    let single_ref = &*foo;

    println!("{}", foo.abs());
}

pub fn mutable_aliasing_bad() {
    let mut x = vec![1, 2, 3];
    let y = &x[0];
    x.push(4);
    // println!("{}", y);
    // BAD. We should never alias and mutate data at the same time
}

pub fn borrow_checker() {
    let mut v = vec![1, 2, 3];
    let third = &v[2];
    v.push(4);
    // println!("{}", third);
}

pub fn borrow_checker_mut() {
    let mut v = vec![1, 2, 3];
    let third = &mut v[2];
    *third += 1;
    println!("{}", third);
    // Note that a mutable borrow can be downgraded to an immutable borrow
    // and that the lifetime of a variable can change based on control flow
}

// pub fn returns_dangling() -> &String {
//     let s = String::from("hello");
//     &s
// }
