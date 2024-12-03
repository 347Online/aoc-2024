use std::error::Error;

use day1::*;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", day1b()?);

    Ok(())
}
