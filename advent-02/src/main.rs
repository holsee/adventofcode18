use core::iter::FromIterator;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let filename = "input.txt";
    if let Ok(file) = file_reader(&filename) {
        let checksum = checksum(file);
        println!("Checksum: {}", checksum);
    };
    if let Ok(file) = file_reader(&filename) {
	for common in find_common_patterns(file) {
	    println!("Common Pattern: {:?}", common);
	}
    }
    Ok(())
}

/**
 * Part 2
 */

fn find_common_patterns(file: BufReader<File>) -> Vec<String> {
    let common_codes: Vec<_> = file
        .lines()
        .filter_map(|line| line.ok())
        .combinations(2)
        .map(|pair| {
            let code: Vec<char> = pair[0].chars().collect();
            let other: Vec<char> = pair[1].chars().collect();
            let mut diff: usize = 0;
            let mut common: Vec<char> = Vec::with_capacity(code.len());
            for (i, code_char) in code.iter().enumerate() {
                let other_char: char = other[i];
                let mismatch: bool = code_char.ne(&other_char);
                if mismatch {
                    diff += 1;
                    common.insert(i, '_');
                } else {
                    common.insert(i, other_char);
                }
            }
            if diff == 1 {
                // println!(
                //     "  code: {:?} \n other: {:?}\ncommon: {:?}",
                //     code, other, common
                // );
                let pattern = String::from_iter(common);
                Some(pattern)
            } else {
                None
            }
        })
        .filter_map(|res| res)
        .collect();

     common_codes
}

/**
 * Common
 */

fn file_reader(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let br = BufReader::new(file);
    Ok(br)
}

/**
 * Part 1
 */

fn checksum(file: BufReader<File>) -> usize {
    let (x, y): (usize, usize) = file.lines().fold((0, 0), |(ls, rs), line| match line {
        Ok(ln) => {
            let (l, r) = scan_code(ln);
            (ls + l, rs + r)
        }
        Err(_) => (ls, rs),
    });

    x * y
}

fn scan_code(code: String) -> (usize, usize) {
    let mut occ = HashMap::new();

    code.chars().for_each(|ch| {
        match occ.get(&ch) {
            Some(cnt) => occ.insert(ch, cnt + 1),
            None => occ.insert(ch, 1),
        };
    });

    let mut res = (0, 0);

    for (_, v) in occ.iter() {
        res = match v {
            0 => res,
            1 => res,
            2 => (1, res.1),
            _ => (res.0, 1),
        };
        if res == (1, 1) {
            break;
        };
    }
    res
}

#[test]
fn examples() {
    assert_eq!((0, 0), scan_code("abcdef".to_string()));
    assert_eq!((1, 1), scan_code("bababc".to_string()));
    assert_eq!((1, 0), scan_code("abbcde".to_string()));
    assert_eq!((0, 1), scan_code("abcccd".to_string()));
    assert_eq!((1, 0), scan_code("aabcdd".to_string()));
    assert_eq!((1, 0), scan_code("abcdee".to_string()));
    assert_eq!((0, 1), scan_code("ababab".to_string()));
}
