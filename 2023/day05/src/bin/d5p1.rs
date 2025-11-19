use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use day05::Almanac;

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let almanac = Almanac::from_str(&buffer)?;
    let mut locations = Vec::new();
    for seed in almanac.seeds.iter() {
        let location = almanac.seed_to_location(*seed);
        locations.push(location);
    }
    println!("{}", locations.iter().min().unwrap());
    Ok(())
}
