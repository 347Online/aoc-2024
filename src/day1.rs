use std::{error::Error, fs::read_to_string};

fn day1() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let input = read_to_string("./input/day1.txt")?;

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for (x, y) in input.lines().map(|x| x.split_once("   ").unwrap()) {
        left.push(x.parse()?);
        right.push(y.parse()?);
    }

    Ok((left, right))
}

pub fn day1a() -> Result<i32, Box<dyn Error>> {
    let (mut left, mut right) = day1()?;

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (x, y)| acc + (x - y).abs());

    Ok(result)
}

pub fn day1b() -> Result<i32, Box<dyn Error>> {
    let (left, right) = day1()?;

    let mut similarity = 0;

    for x in left.iter() {
        let mut s = 0;
        for y in right.iter() {
            if y == x {
                s += 1;
            }
        }
        similarity += x * s;
    }

    Ok(similarity)
}
