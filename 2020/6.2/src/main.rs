use std::str::FromStr;
use std::{env, fs};
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

type Group = HashSet<char>;


fn read_groups(path: &str) -> anyhow::Result<Vec<Group>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(&file);
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if &line == "" {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(HashSet::from_iter(line.chars()));
        }
    }
    if group.len() > 0 {
        groups.push(group);
    }
    let groups = groups.into_iter().map(|people| {
        people.into_iter().reduce(|mut accum, person| {
            accum.retain(|i| person.contains(i));
            accum
        }).unwrap()
    });
    Ok(groups.collect())
}

fn main() -> anyhow::Result<()> {
    let path = env::args().nth(1).expect("Input file argument is required");
    let groups = read_groups(&path)?;
    let sum: usize = groups.iter().map(|g| g.len()).sum();
    println!("{}", sum);
    Ok(())
}
