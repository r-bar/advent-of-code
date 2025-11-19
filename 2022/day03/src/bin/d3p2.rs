use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const GROUP_SIZE: usize = 3;

fn value(chr: char) -> usize {
    let ord = chr as usize;
    if chr.is_ascii_lowercase() {
        ord - 96
    } else if chr.is_ascii_uppercase() {
        ord - 38
    } else {
        0
    }
}

fn group_value(group: &[String]) -> usize {
    let shared = group
        .iter()
        .map(|items| HashSet::from_iter(items.chars()))
        .reduce(|accum: HashSet<char>, items| {
            HashSet::from_iter(accum.intersection(&items).copied())
        })
        .unwrap();
    assert!(shared.len() == 1);
    let shared = shared.into_iter().next().unwrap();
    value(shared) as usize
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(&filename)?;
    let reader = BufReader::new(f);

    let mut sum = 0usize;
    let mut group: Vec<String> = Vec::with_capacity(3);
    for line in reader.lines() {
        group.push(line?.to_string());
        if group.len() == GROUP_SIZE {
            sum += group_value(&group);
            group.clear();
        }
    }
    println!("{}", sum);
    Ok(())
}
