#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let area_result = if let Some(input) = file_reader("input.txt") {
        let mut area = [[0u8; 1000]; 1000];
        for pattern in input {
            if let Some((_id, (x_start, y_start), (xs, ys))) = parse(pattern) {
                for x in 0..xs {
                    for y in 0..ys {
                        let x = x_start + x;
                        let y = y_start + y;
                        if area[x][y] > 1 {
                        } else {
                            area[x][y] += 1
                        }
                    }
                }
            }
        }

        let mut count: usize = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if area[x][y] > 1 {
                    count += 1;
                }
            }
        }
        Some(count)
    } else {
        None
    };
    match area_result {
        Some(area) => println!("Area with 2 or more Claims: {} sq inch", area),
        None => println!("Failed!"),
    }
}

/**
 * Common
 */

fn file_reader(filename: &str) -> Option<Vec<String>> {
    if let Ok(file) = File::open(filename) {
        let br = BufReader::new(file);
        let lines: Vec<String> = br.lines().filter_map(|line| line.ok()).collect();
        Some(lines)
    } else {
        None
    }
}

/**
 * Part 1
 */

fn parse(pattern: String) -> Option<(String, (usize, usize), (usize, usize))> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#([0-9]{1,4}) @ ([0-9]{1,3}),([0-9]{1,3}): ([0-9]{1,3})x([0-9]{1,3})")
                .unwrap();
    }
    if let Some(cap) = RE.captures(pattern.as_str()) {
        let res: (String, (usize, usize), (usize, usize)) = (
            cap[1].to_string(),
            (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
        );
        Some(res)
    } else {
        None
    }
}
