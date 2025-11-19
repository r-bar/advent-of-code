use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

const VALID_CARDS: [char; 13] = [
    'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Card(char);

impl Card {
    fn rank(&self) -> u8 {
        match self.0 {
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'J' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => unreachable!(),
        }
    }

    fn joker_rank(&self) -> u8 {
        match self.0 {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if !VALID_CARDS.contains(&c) {
            Err(anyhow::anyhow!("Invalid card: {}", c))
        } else {
            Ok(Card(c))
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rank().cmp(&other.rank()))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn rank(&self) -> u8 {
        match self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }
}

impl Hand {
    fn score(&self, jokers: bool) -> u32 {
        let hand_type = if jokers {
            self.joker_hand_type()
        } else {
            self.hand_type()
        };
        let type_rank: u32 = hand_type.rank().into();
        let card_score: u32 = self
            .cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                if jokers {
                    card.joker_rank() as u32 * 13u32.pow(4 - i as u32)
                } else {
                    card.rank() as u32 * 13u32.pow(4 - i as u32)
                }
            })
            .sum();
        type_rank * 13u32.pow(5) + card_score
    }

    fn hand_type(&self) -> HandType {
        match self.counts().as_slice() {
            [(1, _), (1, _), (1, _), (1, _), (1, _)] => HandType::HighCard,
            [(1, _), (1, _), (1, _), (2, _)] => HandType::OnePair,
            [(1, _), (2, _), (2, _)] => HandType::TwoPair,
            [(1, _), (1, _), (3, _)] => HandType::ThreeOfAKind,
            [(2, _), (3, _)] => HandType::FullHouse,
            [(1, _), (4, _)] => HandType::FourOfAKind,
            [(5, _)] => HandType::FiveOfAKind,
            counts => {
                println!("Invalid counts: {counts:?}");
                unreachable!()
            }
        }
    }

    fn joker_hand_type(&self) -> HandType {
        let mut counts = self.counts();
        let jokers = counts
            .iter()
            .find(|(_, card)| card == &Card('J'))
            .map(|(count, _)| *count);
        counts.retain(|(_, card)| card != &Card('J'));
        match (jokers, counts.as_slice()) {
            (None, _) => self.hand_type(),
            (Some(1), [(1, _), (1, _), (1, _), (1, _)]) => HandType::OnePair,
            (Some(1), [(1, _), (1, _), (2, _)]) => HandType::ThreeOfAKind,
            (Some(1), [(2, _), (2, _)]) => HandType::FullHouse,
            (Some(1), [(1, _), (3, _)]) => HandType::FourOfAKind,
            (Some(1), [(4, _)]) => HandType::FiveOfAKind,
            (Some(2), [(1, _), (1, _), (1, _)]) => HandType::ThreeOfAKind,
            (Some(2), [(1, _), (2, _)]) => HandType::FourOfAKind,
            (Some(2), [(3, _)]) => HandType::FiveOfAKind,
            (Some(3), [(1, _), (1, _)]) => HandType::FourOfAKind,
            (Some(3), [(2, _)]) => HandType::FiveOfAKind,
            (Some(4), [(1, _)]) => HandType::FiveOfAKind,
            (Some(5), []) => HandType::FiveOfAKind,
            counts => {
                println!("Invalid counts: {counts:?}");
                unreachable!()
            }
        }
    }

    fn counts(&self) -> Vec<(u8, Card)> {
        let mut counts = HashMap::new();
        for &count in self.cards.iter() {
            *counts.entry(count).or_insert(0) += 1;
        }
        let mut counts_list: Vec<(u8, Card)> = counts
            .into_iter()
            .map(|(card, count)| (count, card))
            .collect();
        counts_list.sort();
        counts_list
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [cards, bid] => Ok(Hand {
                cards: cards
                    .chars()
                    .map(Card::try_from)
                    .collect::<Result<Vec<Card>, _>>()?,
                bid: bid.parse()?,
            }),
            _ => Err(anyhow::anyhow!("Invalid hand: {}", s)),
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards_str: String = self.cards.iter().map(|c| c.0).collect();
        write!(f, "{} {}", cards_str, self.bid)
    }
}

fn play(mut hands: Vec<Hand>, jokers: bool) -> usize {
    hands.sort_by_key(|hand| hand.score(jokers));
    hands
        .into_iter()
        .zip(1..)
        .map(|(hand, rank)| {
            let winnings = rank * hand.bid as usize;
            //println!(
            //    "hand = {} rank = {} bid = {} winnings = {}, score = {}",
            //    hand, rank, hand.bid, winnings, hand.score()
            //);
            winnings
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut hands = Vec::new();
    for line in reader.lines() {
        let hand = Hand::from_str(&line?)?;
        hands.push(hand);
    }
    let p1_total = play(hands.clone(), false);
    println!("Part 1: {p1_total}");
    let p2_total = play(hands, true);
    println!("Part 2: {p2_total}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part1_example() {
        let example = include_str!("../example.txt");
        let hands: Vec<Hand> = example
            .lines()
            .map(|line| Hand::from_str(line).unwrap())
            .collect();
        let total = play(hands, false);
        assert_eq!(total, 6440);
    }

    #[test]
    fn test_part2_example() {
        let example = include_str!("../example.txt");
        let hands: Vec<Hand> = example
            .lines()
            .map(|line| Hand::from_str(line).unwrap())
            .collect();
        let total = play(hands, true);
        assert_eq!(total, 5905);
    }

    #[test]
    fn test_card_scores() -> anyhow::Result<()> {
        let hands = [
            Hand::from_str("KQJT9 0")?,
            Hand::from_str("KTJJT 0")?,
            Hand::from_str("KK877 0")?,
            Hand::from_str("T55J5 0")?,
            Hand::from_str("T55K5 0")?,
            Hand::from_str("45555 0")?,
            Hand::from_str("53333 0")?,
        ];
        for (low_hand, high_hand) in hands.iter().zip(hands.iter().skip(1)) {
            let low_score = low_hand.score(false);
            let low_type = low_hand.hand_type();
            let high_score = high_hand.score(false);
            let high_type = high_hand.hand_type();
            println!(
                "low hand: {} type: {:?} score: {}",
                low_hand, low_type, low_score
            );
            println!(
                "high hand: {} type: {:?} score: {}",
                high_hand, high_type, high_score
            );
            assert!(low_score < high_score);
        }
        Ok(())
    }
}
