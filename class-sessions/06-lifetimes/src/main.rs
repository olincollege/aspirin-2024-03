use std::u32;
mod lifetimes;

fn largest_u32(array: &[u32]) -> u32 {
    let mut largest_val = u32::MIN;

    for val in array.into_iter() {
        if *val > largest_val {
            largest_val = *val;
        }
    }

    largest_val
}

fn largest_i32(array: &[i32]) -> i32 {
    let mut largest_val = i32::MIN;

    for val in array.into_iter() {
        if *val > largest_val {
            largest_val = *val;
        }
    }

    largest_val
}

fn largest_generic<T>(array: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut largest_val = &array[0];

    for val in array.into_iter() {
        if val > largest_val {
            largest_val = val;
        }
    }

    largest_val
}

enum Result<TYPE, ERROR> {
    Ok(TYPE),
    Err(ERROR),
}

enum Optional<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");
}
