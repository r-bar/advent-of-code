use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;

use day11::{Map, distance};

fn solve(mut map: Map, expansion: usize) -> usize {
    map.expand(expansion);
    let locations: Vec<_> = map.iter_locations().copied().collect();
    let mut sum = 0;
    for pair in locations.into_iter().combinations(2) {
        sum += distance(pair[0], pair[1]);
    }
    sum
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let map: Map = buffer.parse()?;
    log::debug!("{map}");
    let solution = solve(map, 1_000_000);
    println!("{solution}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_test() {
        let example: Map = include_str!("../../example.txt").parse().unwrap();
        assert_eq!(solve(example.clone(), 10), 1030);
        assert_eq!(solve(example, 100), 8410);
    }
}
