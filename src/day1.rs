use std::{error::Error, fs::read_to_string};

pub fn day1a() -> Result<i32, Box<dyn Error>> {
    let input = read_to_string("./input/day1a.txt")?;

    let mut a = vec![];
    let mut b = vec![];

    for (x, y) in input.lines().map(|x| {
        let (a, b) = x.split_once("   ").unwrap();
        (a, b)
    }) {
        a.push(x);
        b.push(y);
    }

    a.sort();
    b.sort();

    let result = a.iter().zip(b.iter()).fold(0, |acc, (x, y)| {
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        acc + (x - y).abs()
    });

    Ok(result)
}
