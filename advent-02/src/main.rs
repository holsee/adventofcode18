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
    Ok(())
}

fn file_reader(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    let br = BufReader::new(file);
    Ok(br)
}

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
