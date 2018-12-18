use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[allow(unused_must_use)]
fn main() -> io::Result<()> {
    one();
    two()
}

fn one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);
    let final_freq = br
        .lines()
        .map(|line| -> i32 {
            match line {
                Ok(v) => {
                    let x = v.parse::<i32>();
                    match x {
                        Ok(i) => i,
                        Err(_) => 0,
                    }
                }
                Err(_) => 0,
            }
        })
        .sum::<i32>();

    println!("Final Frequency: {}", final_freq);

    Ok(())
}

fn two() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let br = BufReader::new(file);

    let input: Vec<i32> = br
        .lines()
        .map(|line| -> i32 {
            match line {
                Ok(v) => {
                    let x = v.parse::<i32>();
                    match x {
                        Ok(i) => i,
                        Err(_) => 0,
                    }
                }
                Err(_) => 0,
            }
        })
        .collect();

    let length = input.len();
    let mut history = HashSet::new();
    let mut freq: i32 = 0;
    let mut idx: usize = 0;

    while history.insert(freq) {
        freq += input[idx % length];
        idx += 1;
    }

    println!("First Recurrent Freqeuency: {}", freq);

    Ok(())
}
