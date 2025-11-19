use std::{env, fs};
use std::io::BufReader;
use std::io::prelude::*;

#[derive(PartialEq)]
enum Cell {
    Tree,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '#' => Cell::Tree,
            _ => panic!("unknown map element {}", c),
        }
    }
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut cells = Vec::new();
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            cells.push(line.trim().chars()
                .map(Cell::from)
                .collect()
            )
        }
        Ok(Map { cells })
    }

    fn width(&self) -> usize {
        self.cells[0].len()
    }

    /// Return true if the map can be successfully traversed with the given trajectory
    fn traverse(&self, down: isize, right: isize) -> usize {
        let mut posx: isize = 0;
        let width = self.width() as isize;
        let mut trees = 0;
        //let mut posy = 0;
        for lateral in &self.cells {
            if lateral[posx as usize] == Cell::Tree {
                trees += 1;
            }
            posx = if posx + right >= 0 {
                (posx + right) % width
            } else {
                width - ((posx + right) % width)
            }
        }
        trees
    }
}

fn main() {
    let path = env::args().nth(1).expect("Please provide an input file argument.");
    let map = Map::from_file(&path).expect("Could not read map file");
    let trees = map.traverse(1, 3);
    println!("{} trees encountered", trees);
}
