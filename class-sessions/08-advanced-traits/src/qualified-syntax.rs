trait Animal {
    fn noise(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn noise(&self) -> String {
        "woof".to_string()
    }
}

impl Dog {
    fn noise(&self) -> String {
        "bark".to_string()
    }
}

fn main() {
    let dog = Dog;
    println!("A dog says: {}", Dog::noise(&dog));
}
