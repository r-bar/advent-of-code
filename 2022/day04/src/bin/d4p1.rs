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

fn contains((s1, e1): ClearRange, (s2, e2): ClearRange) -> bool {
    s1 <= s2 && e1 >= e2
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);
    let mut overlap = 0usize;
    for line in reader.lines() {
        let (range1, range2): (ClearRange, ClearRange) = parse_line(&line?);
        if contains(range1, range2) || contains(range2, range1) {
            overlap += 1;
        }
    }
    println!("{}", overlap);
    Ok(())
}
