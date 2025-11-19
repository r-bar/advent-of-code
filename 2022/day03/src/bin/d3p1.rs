use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn value(chr: char) -> u32 {
    let ord = chr as u32;
    if chr.is_ascii_lowercase() {
        ord - 96
    } else if chr.is_ascii_uppercase() {
        ord - 38
    } else {
        0
    }
}

fn parse_line<'a>(line: &'a str) -> (&'a str, &'a str) {
    let compartment_items = line.len() / 2;
    line.split_at(compartment_items)
}

fn prioritize(left: &str, right: &str) -> u32 {
    let left: HashSet<char> = left.chars().collect();
    let right: HashSet<char> = right.chars().collect();
    left.intersection(&right).map(|c| value(*c)).sum()
}

fn main() -> anyhow::Result<()> {
    dbg!('a' as u32);
    dbg!('A' as u32);
    dbg!('1' as u32);
    dbg!(value('a'));
    dbg!(value('A'));
    dbg!(value('1'));
    let filename = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);

    let mut sum = 0usize;
    for line in reader.lines() {
        let line = line?;
        let (left, right) = parse_line(&line);
        let priority = prioritize(left, right);
        sum += priority as usize;
    }
    println!("{}", sum);
    Ok(())
}
