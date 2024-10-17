trait Animal {
    fn number_of_feet(&self) -> u32;
    fn number_of_heads(&self) -> u32 {
        1
    }
    fn noise(&self) -> String;
    fn name(&self) -> String;

    fn about_me(&self) {
        println!(
            "The {}, has {} feet, {} heads, and says {}",
            self.name(),
            self.number_of_feet(),
            self.number_of_heads(),
            self.noise()
        );
    }
}

struct Cherry {}

impl Animal for Cherry {
    fn number_of_feet(&self) -> u32 {
        2
    }
    fn number_of_heads(&self) -> u32 {
        2
    }
    fn noise(&self) -> String {
        "schreech".to_string()
    }
    fn name(&self) -> String {
        "Cherry".to_string()
    }
}

struct Dog;

impl Animal for Dog {
    fn name(&self) -> String {
        "Fido".to_string()
    }

    fn noise(&self) -> String {
        "Woof".to_string()
    }

    fn number_of_feet(&self) -> u32 {
        4
    }
}

fn feet_plus_heads(animal: &dyn Animal) -> u32 {
    animal.number_of_heads() + animal.number_of_feet()
}

fn main() {
    let Cherry_thing = Cherry {};
    let fido = Dog {};
    Cherry_thing.about_me();

    println!(
        "cherry: {}, dog: {}",
        feet_plus_heads(&Cherry_thing),
        feet_plus_heads(&fido)
    );
}
