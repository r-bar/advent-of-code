use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

use day04::Card;

fn play(cards: &[Card]) -> HashMap<usize, u32> {
    let mut copies = HashMap::new();
    for (card, id) in cards.iter().zip(1..) {
        let current_copies = *copies.get(&id).unwrap_or(&1);
        let won = card.winning_numbers().count();
        for won_id in id + 1..id + 1 + won {
            let won_copies = copies.entry(won_id).or_insert(1);
            *won_copies += current_copies;
        }
    }
    copies
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut cards = Vec::new();
    for line in reader.lines() {
        let card = Card::try_from(line?)?;
        cards.push(card);
    }
    let copies = play(&cards);
    dbg!(&copies);
    let total = (1..=cards.len())
        .map(|id| copies.get(&id).unwrap_or(&1))
        .sum::<u32>();
    println!("Solution: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../example.txt");

    #[test]
    fn test_play() {
        let cards: Vec<Card> = EXAMPLE.lines()
            .map(|line| Card::try_from(line.to_string()).unwrap())
            .collect();
        let copies = play(&cards);
        assert_eq!(copies.get(&1), None);
        assert_eq!(copies.get(&2), Some(&2));
        assert_eq!(copies.get(&3), Some(&4));
        assert_eq!(copies.get(&4), Some(&8));
        assert_eq!(copies.get(&5), Some(&14));
        assert_eq!(copies.get(&6), None);
        let sum = (1..=cards.len())
            .map(|id| copies.get(&id).unwrap_or(&1))
            .sum::<u32>();
        assert_eq!(sum, 30);
    }
}
