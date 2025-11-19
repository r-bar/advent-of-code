use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type ClearRange = (usize, usize);

fn parse_line(line: &str) -> (ClearRange, ClearRange) {
    let numbers: Vec<usize> = line
        .split(|c: char| !(c.is_ascii_digit()))
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    assert!(numbers.len() == 4);
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

fn overlap(r1: ClearRange, r2: ClearRange) -> bool {
    (r1.0 <= r2.1 && r2.1 <= r1.1) || (r2.0 <= r1.1 && r1.1 <= r2.1)
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);
    let mut overlaps = 0usize;
    for line in reader.lines() {
        let line = line?.trim().to_string();
        let (range1, range2): (ClearRange, ClearRange) = parse_line(&line);
        //println!(
        //    "{} {}",
        //    &line,
        //    if overlap(range1, range2) {
        //        "True"
        //    } else {
        //        "False"
        //    },
        //);
        if overlap(range1, range2) {
            overlaps += 1;
        }
    }
    println!("{}", overlaps);
    Ok(())
}
