use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use day03::{Point, Schematic};


fn numbers_with_symbols(schematic: &Schematic) -> Vec<u32> {
    let mut numbers = schematic.numbers();
    numbers.retain(|n| {
        schematic
            .neighbors(n.offset, n.len())
            .iter()
            .any(|&n| matches!(schematic.data[n], Point::Symbol(_)))
    });
    numbers.into_iter().map(|n| n.value()).collect()
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
    let solution: u32 = numbers_with_symbols(&schematic)
        .into_iter()
        .sum();
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
        let numbers = numbers_with_symbols(&schematic);
        assert_eq!(numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
        assert!(!numbers.contains(&58));
        assert!(!numbers.contains(&114));
    }
}
