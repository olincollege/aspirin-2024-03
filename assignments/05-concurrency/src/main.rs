use anyhow::Result;
use rand::Rng;

mod error;
mod thread_pool;

/// Generate a random vector of size capacity filled with random i64s
fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

fn main() -> Result<()> {
    let data = random_vec(10_000_000);
    Ok(())
}
