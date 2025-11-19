use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

use day03::{Point, Schematic, Number};


/// Returns a map of offset all offsets covered by a number to the number's starting offset
fn number_locations(schematic: &Schematic) -> HashMap<usize, Number> {
    schematic.numbers()
        .iter()
        .flat_map(|n| {
            //let mut coords = Vec::new();
            (n.offset..(n.offset + n.len()))
                .map(move |offset| (offset, n.clone()))
                .collect::<HashMap<usize, Number>>()
        })
        .collect()
}

/// Returns a list of offsets for gears.
fn gears(schematic: &Schematic) -> Vec<usize> {
    schematic.data.iter().enumerate().filter_map(|(offset, point)| {
        match point {
            Point::Symbol('*') => Some(offset),
            _ => None,
        }
    }).collect()
}

fn part2(schematic: &Schematic) -> u32 {
    let locations = number_locations(schematic);
    let mut solution = 0;
    for gear in gears(schematic).into_iter() {
        let adjacent_numbers: HashSet<Number> = schematic.neighbors(gear, 1)
            .into_iter()
            .filter_map(|n| locations.get(&n))
            .cloned()
            .collect();
        if adjacent_numbers.len() != 2 {
            continue;
        }
        let ratio: u32 = adjacent_numbers.iter().map(|n| n.value()).product();
        solution += ratio;
    }
    solution
}


fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let schematic = Schematic::from(buffer.as_str());
    let solution = part2(&schematic);
    println!("Solution: {}", solution);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE: &str = include_str!("../../example.txt");

    #[test]
    fn example_test() {
        let schematic = Schematic::from(EXAMPLE);
        let solution = part2(&schematic);
        assert_eq!(solution, 467835);
    }
}
