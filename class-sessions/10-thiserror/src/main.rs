use thiserror::Error;

#[derive(Debug, Error)]
enum FeedingError {
    #[error("Invalid Animal Name: {0}")]
    InvalidName(String),
    #[error("Cannot feed {0}")]
    CanNotFeedError(String),
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}

fn get_animal_breed(name: &str) -> Result<String, FeedingError> {
    match name {
        "Steve" => Ok("Cat".to_owned()),
        "Luke" => Ok("Dog".to_owned()),
        other => Err(FeedingError::InvalidName(name.to_owned())),
    }
}

fn feed_animal(name: &str, amount: &str) -> Result<String, FeedingError> {
    let breed = get_animal_breed(name)?;

    let amount_num: i32 = amount.parse()?;

    let owned_cat = String::from("Cat");
    match breed {
        owned_cat => Ok(format!(
            "successfully fed cat {} amount of food",
            amount_num
        )),
        other_breed => Err(FeedingError::CanNotFeedError(other_breed)),
    }
}

fn main() {
    let res = feed_animal("Steve", "1asdfa");
}
