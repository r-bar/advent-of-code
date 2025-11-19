use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use day02::{Color, Game};

fn game_max_per_color(game: &Game) -> (u8, u8, u8) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for pick in &game.picks {
        match pick.color {
            Color::Red => {
                red = std::cmp::max(red, pick.count);
            }
            Color::Green => {
                green = std::cmp::max(green, pick.count);
            }
            Color::Blue => {
                blue = std::cmp::max(blue, pick.count);
            }
        }
    }
    (red, green, blue)
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
        let (red, green, blue) = game_max_per_color(&game);
        let power = red as u32 * green as u32 * blue as u32;
        //println!("{power} = {red}R * {green}G * {blue}B");
        sum += power;
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
