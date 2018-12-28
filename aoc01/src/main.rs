use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let final_freq = one(&input)?;
    writeln!(io::stdout(), "Final Freq: {}", final_freq)?;

    let first_recurrent_freq = two(&input)?;
    writeln!(io::stdout(), "1st Recurrent Freq: {}", first_recurrent_freq)?;

    Ok(())
}

fn one(input: &str) -> Result<i32> {
    let final_freq: i32 = input
        .lines()
        .map(|line| line.parse::<i32>())
        .filter_map(|r| r.ok())
        .sum();

    Ok(final_freq)
}

fn two(input: &str) -> Result<i32> {
    let mut freqs = input
        .lines()
        .map(|line| line.parse::<i32>())
        .filter_map(|r| r.ok())
        .cycle();

    let mut history = HashSet::new();
    let mut freq: i32 = 0;

    while history.insert(freq) {
        if let Some(f) = freqs.next() {
            freq += f;
        }
    }

    Ok(freq)
}

#[test]
fn one_should_return_final_freqency() {
    assert_eq!(360, one("100\n200\n60\n400\n-400").unwrap());
}

#[test]
fn one_should_ignore_non_int_input() {
    assert_eq!(360, one("100\n200\nlolololol\n60\n").unwrap());
}

#[test]
fn two_should_return_first_recurrent_freqency() {
    assert_eq!(300, two("100\n200\n60\n-60").unwrap());
}
