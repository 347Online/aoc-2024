use std::error::Error;

use day1::day1a;

mod day1;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", day1a()?);

    Ok(())
}
