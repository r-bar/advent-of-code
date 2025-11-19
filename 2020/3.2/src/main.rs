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

    /// Returns the number of trees encountered with the given traversal trajectory
    fn traverse(&self, right: isize, down: isize) -> usize {
        let mut posx: isize = 0;
        let width = self.width() as isize;
        let mut trees = 0;
        for (_, lateral) in self.cells.iter().enumerate().filter(|(i, _)| i % down as usize == 0) {
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
    let traversals = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let trees_product = traversals
        .into_iter()
        .map(|(r, d)| map.traverse(r, d))
        .reduce(std::ops::Mul::mul).unwrap();
    println!("{} trees product", trees_product);
}
