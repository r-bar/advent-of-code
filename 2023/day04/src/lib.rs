use std::collections::HashSet;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/card.pest"]
pub struct CardParser;

pub struct Card {
    pub id: usize,
    pub winning: HashSet<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let count = self.winning_numbers().count();
        if count == 0 {
            0
        } else {
            2_u32.pow(count as u32 - 1)
        }
    }

    pub fn winning_numbers(&self) -> impl Iterator<Item = &u32> {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
    }
}


impl TryFrom<String> for Card {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let pairs = CardParser::parse(Rule::card, &value)?;
        let mut id = 0;
        let mut winning = HashSet::new();
        let mut numbers = Vec::new();
        for field in pairs.flatten() {
            match field.as_rule() {
                Rule::card => continue,
                Rule::id => {
                    id = field.as_span().as_str().parse::<usize>()?;
                },
                Rule::winning => {
                    let num = field.as_span().as_str().parse::<u32>()?;
                    winning.insert(num);
                },
                Rule::number => {
                    let num = field.as_span().as_str().parse::<u32>()?;
                    numbers.push(num);
                },
                _ => panic!("unexpected rule: {:?}", field.as_rule()),
            }
        }
        Ok(Card { id, winning, numbers })
    }
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_parsing() {
        let cards: Vec<Card> = EXAMPLE.lines()
            .map(|line| Card::try_from(line.to_string()).unwrap())
            .collect();
        assert_eq!(cards.len(), 6);
        assert_eq!(cards[0].id, 1);
        assert_eq!(cards[0].winning, vec![41, 48, 83, 86, 17].into_iter().collect());
    }

    #[test]
    fn test_points() {
        let cards: Vec<Card> = EXAMPLE.lines()
            .map(|line| Card::try_from(line.to_string()).unwrap())
            .collect();
        assert_eq!(cards[0].points(), 8);
        assert_eq!(cards[1].points(), 2);
        assert_eq!(cards[2].points(), 2);
        assert_eq!(cards[3].points(), 1);
        assert_eq!(cards[4].points(), 0);
        assert_eq!(cards[5].points(), 0);
    }
}
