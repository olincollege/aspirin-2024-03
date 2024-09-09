use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

fn get_input() -> Result<u32, ParseIntError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse()
}

fn main() {
    println!("Guess the number!");

    let mut rng = thread_rng();
    let secret_number: u32 = rng.gen_range(0..=100);

    loop {
        let guess = match get_input() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid entry, try again");
                continue;
            }
        };

        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("Your guess is correct.");
                break;
            }
            Ordering::Greater => println!("Your guess is too low"),
            Ordering::Less => println!("Your guess is too high"),
        }
    }
}
