use anyhow::Result;
use clap::Parser;
use colored::Color;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short = 'v', long)]
    invert_match: bool,

    #[clap(short, long)]
    regex: bool,

    #[clap(short, long)]
    color: Option<Color>,

    needle: String,

    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}
