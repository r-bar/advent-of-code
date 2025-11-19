use pest_derive::Parser;
use pest::Parser;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(anyhow::anyhow!("Invalid color: {value}")),
        }
    }
}

#[derive(Debug)]
pub struct DieSet {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
pub struct Pick {
    pub color: Color,
    pub count: u8,
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub picks: Vec<Pick>,
}

#[derive(Parser)]
#[grammar = "game.pest"]
pub struct GameParser;

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut id = 0;
        let mut picks = Vec::new();
        let parsed_game = GameParser::parse(Rule::game, value)?;
        for pair in parsed_game {
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::game_number => {
                        id = inner.as_str().parse()?;
                    }
                    Rule::set => {
                        let mut color_ptr: Option<Color> = None;
                        let mut count_ptr: Option<u8> = None;
                        for thing in inner.into_inner().flatten() {
                            match thing.as_rule() {
                                Rule::color => {
                                    color_ptr = Some(Color::try_from(thing.as_str())?);
                                }
                                Rule::count => {
                                    count_ptr = Some(thing.as_str().trim().parse()?);
                                }
                                Rule::pick => continue,
                                _ => unreachable!(),
                            }
                            if let (Some(color), Some(count)) = (&color_ptr, &count_ptr) {
                                picks.push(Pick {
                                    color: *color,
                                    count: *count,
                                });
                                color_ptr = None;
                                count_ptr = None;
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(Game { id, picks })
    }
}
