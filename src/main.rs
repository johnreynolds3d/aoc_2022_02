use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();
    let mut score: u32 = 0;

    for line in lines {
        if let Ok(ln) = line {
            match ln.as_str().chars().nth(0).unwrap() {
                'A' => match ln.as_str().chars().nth(2).unwrap() {
                    'X' => score += 4,
                    'Y' => score += 8,
                    'Z' => score += 3,
                    _ => println!("Something went terribly wrong!"),
                },
                'B' => match ln.as_str().chars().nth(2).unwrap() {
                    'X' => score += 1,
                    'Y' => score += 5,
                    'Z' => score += 9,
                    _ => println!("Something went terribly wrong!"),
                },
                'C' => match ln.as_str().chars().nth(2).unwrap() {
                    'X' => score += 7,
                    'Y' => score += 2,
                    'Z' => score += 6,
                    _ => println!("Something went terribly wrong!"),
                },
                _ => println!("Something went terribly wrong!"),
            }
        }
    }
    println!("Total score: {}", score);

    Ok(())
}
