use std::fmt::{self, Display};

fn main() {
    let foo = Wrapper(5);

    println!("foo: {}", foo);
}

struct Wrapper(i32);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i32 of {}", self.0)
    }
}
