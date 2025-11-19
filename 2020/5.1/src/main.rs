use std::str::FromStr;
use std::{env, fs};
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
struct Seat {
    row: i8,
    col: i8,
}

impl Seat {

    fn from_file(path: &str) -> anyhow::Result<Vec<Seat>> {
        let mut seats = Vec::new();
        let file = fs::File::open(&path)?;
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            let line = line?;
            seats.push(Seat::from_str(line.trim())?)
        }
        Ok(seats)
    }

    fn id(&self) -> isize {
        (self.row as isize) * 8 + (self.col as isize)
    }
}


impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(anyhow::anyhow!("Invalid seat string"))
        }
        let bin_str = s
            .replace("R", "1")
            .replace("B", "1")
            .replace("L", "0")
            .replace("F", "0");
        let row = i8::from_str_radix(&bin_str[0..7], 2)?;
        let col = i8::from_str_radix(&bin_str[7..10], 2)?;
        Ok(Seat { row, col })
    }
}

fn main() {
    let path = env::args().nth(1).expect("Input file argument is required");
    let seats = Seat::from_file(&path).unwrap();
    println!("{}", seats.iter().map(Seat::id).max().unwrap());
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::Seat;

    #[test]
    fn seat_string_translation() {
        assert_eq!(Seat::from_str("FBFBBFFRLR").unwrap(), Seat { row: 44, col: 5 });
        assert_eq!(Seat::from_str("FFFBBBFRRR").unwrap(), Seat { row: 14, col: 7 });
        assert_eq!(Seat::from_str("BBFFBBFRLL").unwrap(), Seat { row: 102, col: 4 });
    }

    #[test]
    fn seat_id() {
        assert_eq!(Seat { row: 44, col: 5 }.id(), 357);
        assert_eq!(Seat { row: 14, col: 7 }.id(), 119);
        assert_eq!(Seat { row: 102, col: 4 }.id(), 820);
    }
}
