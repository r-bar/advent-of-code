use std::hash::{Hash, Hasher};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Empty,
    Num(char),
    Symbol(char),
}

impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Point::Num(c),
            '.' => Point::Empty,
            '\n' => panic!("Newline cannot be parsed into a Point"),
            _ => Point::Symbol(c),
        }
    }
}

impl From<Point> for char {
    fn from(p: Point) -> Self {
        match p {
            Point::Empty => ' ',
            Point::Num(c) => c,
            Point::Symbol(c) => c,
        }
    }
}

pub struct Schematic {
    pub width: usize,
    pub data: Vec<Point>,
}

impl From<&str> for Schematic {
    fn from(s: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        for (i, char) in s.chars().enumerate() {
            match (width, char) {
                (0, '\n') => {
                    width = i;
                    continue;
                }
                (_, '\n') => continue,
                _ => {
                    data.push(Point::from(char))
                }
            }
        }
        Schematic { width, data }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    pub offset: usize,
    pub chars: Vec<char>,
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
    }
}

impl Number {

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    pub fn value(&self) -> u32 {
        let mut power = 1;
        let mut num = 0;
        for c in self.chars.iter().rev() {
            num += c.to_digit(10).unwrap() * power;
            power *= 10;
        }
        num
    }
}

impl Schematic {

    pub fn neighbors(&self, offset: usize, length: usize) -> Vec<usize> {
        let (x, y) = self.offset_to_coord(offset);
        let (x, y) = (x as isize, y as isize);
        let width = self.width as isize;
        let height = self.height() as isize;
        let mut result = Vec::new();
        for cy in (y - 1)..=(y + 1) {
            for cx in (x - 1)..(x + length as isize + 1) {
                if cx < 0 || cy < 0 || cx >= width || cy >= height {
                    continue;
                }
                if y == cy && x <= cx && cx < x + length as isize {
                    continue;
                }
                result.push(self.coord_to_offset((cx as usize, cy as usize)));
            }
        }
        result
    }

    pub fn offset_to_coord(&self, offset: usize) -> (usize, usize) {
        (offset % self.width, offset / self.width)
    }

    pub fn coord_to_offset(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn numbers(&self) -> Vec<Number> {
        let mut result = Vec::new();
        let mut iter = self.data.iter().enumerate();
        while let Some((offset, point)) = iter.next() {
            let (mut x, _) = self.offset_to_coord(offset);
            let mut chars = Vec::new();
            match point {
                Point::Num(n) => chars.push(*n),
                _ => continue,
            }
            x += 1;
            while x < self.width {
                match iter.next() {
                    Some((_, Point::Num(n))) => chars.push(*n),
                    _ => break,
                }
                x += 1;
            }
            result.push(Number { offset, chars });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn schematic_numbers() {
        let schematic = Schematic::from("123\n456\n789");
        let numbers = schematic.numbers();
        assert_eq!(schematic.width, 3);
        assert_eq!(schematic.height(), 3);
        //assert_eq!(numbers[0].value(), 123);
        assert_eq!(numbers[1].value(), 456);
        assert_eq!(numbers[2].value(), 789);
        assert_eq!(numbers.len(), 3);
    }

    #[test]
    fn example_schematic_numbers() {
        let schematic = Schematic::from(EXAMPLE);
        let raw_numbers = schematic.numbers();
        dbg!(&raw_numbers);
        let numbers: Vec<u32> = raw_numbers.into_iter().map(|n| n.value()).collect();
        assert_eq!(numbers, vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598]);
    }

    #[test]
    fn example_neighbors() {
        let schematic = Schematic::from(EXAMPLE);
        let neighbors = schematic.neighbors(0, 3);
        assert_eq!(neighbors, vec![3, 10, 11, 12, 13]);
        let neighbors = schematic.neighbors(22, 2);
        #[rustfmt::skip]
        assert_eq!(neighbors, vec![
            11, 12, 13, 14,
            21,         24,
            31, 32, 33, 34,
        ]);
    }
}
