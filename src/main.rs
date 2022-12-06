use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut s = 0;

    for l in lines {
        if let Ok(l) = l {
            let c1 = l.as_str().chars().nth(0).unwrap();
            let c2 = l.as_str().chars().nth(2).unwrap();
            match c1 {
                'A' => match c2 {
                    'X' => s += 3, // 4 for Part 1
                    'Y' => s += 4, // 8 for Part 1
                    'Z' => s += 8, // 3 for Part 1
                    _ => println!("Error!"),
                },
                'B' => match c2 {
                    'X' => s += 1,
                    'Y' => s += 5,
                    'Z' => s += 9,
                    _ => println!("Error!"),
                },
                'C' => match c2 {
                    'X' => s += 2, // 7 for Part 1
                    'Y' => s += 6, // 2 for Part 1
                    'Z' => s += 7, // 6 for Part 1
                    _ => println!("Error!"),
                },
                _ => println!("Error!"),
            }
        }
    }
    println!("Total score: {}", s);

    Ok(())
}
