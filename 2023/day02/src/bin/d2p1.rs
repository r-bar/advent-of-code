use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use day02::{Color, Game};

fn filter(game: &Game) -> bool {
    for pick in game.picks.iter() {
        match pick.color {
            Color::Red => {
                if pick.count > 12 {
                    return false;
                }
            }
            Color::Green => {
                if pick.count > 13 {
                    return false;
                }
            }
            Color::Blue => {
                if pick.count > 14 {
                    return false;
                }
            }
        }
    }
    true
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }
        let game = Game::try_from(line.as_str())?;
        if filter(&game) {
            println!("{line}");
            //println!("{:#?}", game);
            sum += game.id;
        }
    }
    println!("sum: {}", sum);
    Ok(())
}

#[cfg(test)]
mod test {
    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;

        #[test]
        fn example_test() {
            let _example = include_str!("../../example.txt");
        }
    }
}
