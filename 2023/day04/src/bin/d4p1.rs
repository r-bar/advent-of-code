use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use day04::Card;

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines() {
        let card = Card::try_from(line?)?;
        sum += card.points();
    }
    println!("Solution: {}", sum);
    Ok(())
}
