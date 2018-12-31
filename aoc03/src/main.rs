#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::error::Error;
use std::io::{self, Read, Write};
use std::result;
use std::collections::{HashMap, HashSet};

type Result<T> = result::Result<T, Box<Error>>;
type Surface = Vec<[u8; 1000]>;
type ClaimCode = (usize, (usize, usize), (usize, usize));

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let surface = compute_surface(&input)?;

    let area = area_with_overlapping_claims(surface)?;
    writeln!(io::stdout(), "Area with overlapping Claims: {} in^2", area)?;

    let ids = find_ids_with_no_overlap(&input)?;
    writeln!(io::stdout(), "Ids with no overlap: {:?}", ids)?;

    Ok(())
}



/**
 * Part 1
 */

fn compute_surface(input: &String) -> Result<Surface> {
    // Representation of surface
    let mut area = vec![[0u8; 1000]; 1000];

    // Parse codes
    for pattern in input.lines() {
        if let Some((_id, (x_start, y_start), (xs, ys))) = parse_code(pattern) {
            // Update surface
            for x in 0..xs {
                for y in 0..ys {
                    let x = x_start + x;
                    let y = y_start + y;
                    area[x][y] += 1;
                }
            }
        }
    }

    Ok(area)
}

fn area_with_overlapping_claims(surface: Surface) -> Result<usize> {
    let mut count: usize = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if surface[x][y] > 1 {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn parse_code(pattern: &str) -> Option<ClaimCode> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#([0-9]{1,4}) @ ([0-9]{1,3}),([0-9]{1,3}): ([0-9]{1,3})x([0-9]{1,3})")
                .unwrap();
    }
    if let Some(cap) = RE.captures(pattern) {
        Some((
            cap[1].parse().unwrap(),
            (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
        ))
    } else {
        None
    }
}

/**
 * Part 2
 */

fn find_ids_with_no_overlap(input: &String) -> Result<HashSet<usize>> {
    // Representation of surface
    let mut ids = HashMap::new();
    let mut uniq_ids = HashSet::new();

    // Parse codes
    for pattern in input.lines() {
        if let Some((id, (x_start, y_start), (xs, ys))) = parse_code(pattern) {
            uniq_ids.insert(id);
            // Update surface
            for x in 0..xs {
                for y in 0..ys {
                    let x = x_start + x;
                    let y = y_start + y;
                    match ids.insert((x, y), id) {
                        Some(prev) => {
                            uniq_ids.remove(&id);
                            uniq_ids.remove(&prev);
                        },
                        None => ()
                    }
                }
            }
        }
    }

    Ok(uniq_ids)
}

#[test]
fn compute_surface_should_be_completely_unset_on_no_input() {
    let surface = compute_surface("#1 @ 55,885: 22x10\n#2 @ 102,14: 23x14".to_string());
    if let Ok(s) = surface {
        assert_eq!(0, s[0][0]);
    };
}

#[test]
fn area_with_overlapping_claims_should_return_0_when_no_overlap() {
    let surface = vec![[0u8; 1000]; 1000];
    let area = area_with_overlapping_claims(surface).unwrap();
    assert_eq!(0, area);
}

#[test]
fn area_with_overlapping_claims_should_return_overlapping_area() {
    let surface = compute_surface("#1 @ 1,1: 10x10\n#2 @ 1,1: 10x10".to_string()).unwrap();
    let area = area_with_overlapping_claims(surface).unwrap();
    assert_eq!(100, area);
}

#[test]
fn parse_code_should_capture_claim() {
    let code = "#1 @ 55,885: 22x10";
    let expected = (1, (55, 885), (22, 10));
    assert_eq!(Some(expected), parse_code(code));
}

#[test]
fn parse_code_should_return_none_on_invalid_code() {
    let code = "#1 @ 55,885 22x10";
    assert_eq!(None, parse_code(code));
}
