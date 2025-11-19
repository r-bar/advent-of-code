use std::hash::Hash;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Clone)]
pub struct Grid {
    pub start: Point,
    grid: Vec<Vec<char>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        for (lineno, line) in s.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let mut row = Vec::new();
            for (colno, c) in line.trim().chars().enumerate() {
                if c == 'S' {
                    start = Point {
                        x: colno,
                        y: lineno,
                    };
                }
                row.push(c);
            }
            grid.push(row);
        }
        Ok(Grid { start, grid })
    }
}

struct GridIterator<'a> {
    grid: &'a Grid,
    current: Point,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.width() > self.current.x + 1 {
            self.current.x += 1;
            return Some(self.current);
        }
        if self.grid.height() > self.current.y + 1 {
            self.current.x = 0;
            self.current.y += 1;
            return Some(self.current);
        }
        None
    }
}

struct PathIterator<'a> {
    grid: &'a Grid,
    current: Point,
    prev: Point,
    step: usize,
    start: Point,
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.step {
            0 => {
                self.step += 1;
                return Some(self.prev);
            }
            1 => {
                self.step += 1;
                return Some(self.current);
            }
            _ => {
                self.step += 1;
            }
        }
        let from_direction = self.grid.rel_direction(self.current, self.prev)?;
        let current_char = self.grid.get(self.current.x, self.current.y)?;
        let next = match (from_direction, current_char) {
            (Direction::N, 'L') => Point {
                x: self.current.x + 1,
                y: self.current.y,
            },
            (Direction::E, 'L') => Point {
                x: self.current.x,
                y: self.current.y - 1,
            },
            (Direction::N, 'J') => Point {
                x: self.current.x - 1,
                y: self.current.y,
            },
            (Direction::W, 'J') => Point {
                x: self.current.x,
                y: self.current.y - 1,
            },
            (Direction::S, 'F') => Point {
                x: self.current.x + 1,
                y: self.current.y,
            },
            (Direction::E, 'F') => Point {
                x: self.current.x,
                y: self.current.y + 1,
            },
            (Direction::S, '7') => Point {
                x: self.current.x - 1,
                y: self.current.y,
            },
            (Direction::W, '7') => Point {
                x: self.current.x,
                y: self.current.y + 1,
            },
            (Direction::N, '|') => Point {
                x: self.current.x,
                y: self.current.y + 1,
            },
            (Direction::S, '|') => Point {
                x: self.current.x,
                y: self.current.y - 1,
            },
            (Direction::E, '-') => Point {
                x: self.current.x - 1,
                y: self.current.y,
            },
            (Direction::W, '-') => Point {
                x: self.current.x + 1,
                y: self.current.y,
            },
            _ => return None,
        };
        if next == self.start {
            return None;
        }
        if self.grid.get(next.x, next.y).is_some() {
            self.prev = self.current;
            self.current = next;
            Some(next)
        } else {
            None
        }
    }
}

impl Grid {
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn size(&self) -> usize {
        self.width() * self.height()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        self.grid.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        self.grid.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn rel_direction(&self, origin: Point, adjacent: Point) -> Option<Direction> {
        let dx = adjacent.x as isize - origin.x as isize;
        let dy = adjacent.y as isize - origin.y as isize;
        match (dx, dy) {
            (0, 0) => None,
            (0, 1) => Some(Direction::S),
            (0, -1) => Some(Direction::N),
            (1, 0) => Some(Direction::E),
            (-1, 0) => Some(Direction::W),
            _ => None,
        }
    }

    pub fn rel_pt(&self, origin: Point, direction: Direction) -> Option<Point> {
        let pt = match direction {
            Direction::N => origin.y.checked_sub(1).map(|y| Point { x: origin.x, y }),
            Direction::S => Some(Point {
                x: origin.x,
                y: origin.y + 1,
            }),
            Direction::E => Some(Point {
                x: origin.x + 1,
                y: origin.y,
            }),
            Direction::W => origin.x.checked_sub(1).map(|x| Point { x, y: origin.y }),
        };
        pt.and_then(|p| {
            if self.get(p.x, p.y).is_some() {
                Some(p)
            } else {
                None
            }
        })
    }

    pub fn start_points(&self, start: Point) -> Vec<Point> {
        let mut points = Vec::new();
        let n_coord = start.y.checked_sub(1).map(|y| Point { x: start.x, y });
        let n_chr = n_coord.and_then(|p| self.get(p.x, p.y));
        let s_coord = Point {
            x: start.x,
            y: start.y + 1,
        };
        let s_chr = self.get(s_coord.x, s_coord.y);
        let e_coord = Point {
            x: start.x + 1,
            y: start.y,
        };
        let e_chr = self.get(e_coord.x, e_coord.y);
        let w_coord = start.x.checked_sub(1).map(|x| Point { x, y: start.y });
        let w_chr = w_coord.and_then(|p| self.get(p.x, start.y));
        if n_chr == Some('|') || n_chr == Some('F') || n_chr == Some('7') {
            points.push(n_coord.unwrap());
        }
        if s_chr == Some('|') || s_chr == Some('J') || s_chr == Some('L') {
            points.push(s_coord);
        }
        if e_chr == Some('-') || e_chr == Some('7') || e_chr == Some('J') {
            points.push(e_coord);
        }
        if w_chr == Some('-') || w_chr == Some('L') || w_chr == Some('F') {
            points.push(w_coord.unwrap());
        }
        points
    }

    pub fn get_next(&self, prev: Point, current: Point) -> Option<Point> {
        PathIterator {
            grid: self,
            current,
            prev,
            step: 2,
            start: prev,
        }
        .next()
    }

    pub fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        GridIterator {
            grid: self,
            current: Point { x: 0, y: 0 },
        }
    }

    pub fn path(&self, start: Point) -> Option<impl Iterator<Item = Point> + '_> {
        self.start_points(start)
            .first()
            .map(|current| PathIterator {
                grid: self,
                current: *current,
                prev: start,
                step: 0,
                start,
            })
    }

    pub fn expand(&self) -> Self {
        let width = self.width() * 3;
        let height = self.height() * 3;
        let start = Point {
            x: self.start.x * 3 + 1,
            y: self.start.y * 3 + 1,
        };
        let mut grid = Grid {
            start,
            grid: vec![vec!['.'; width]; height],
        };
        for point in self.iter() {
            let top_left_x = point.x * 3;
            let top_left_y = point.y * 3;
            match self.get(point.x, point.y) {
                Some('F') => {
                    grid.grid[top_left_y + 1][top_left_x + 1] = 'F';
                    grid.grid[top_left_y + 1][top_left_x + 2] = '-';
                    grid.grid[top_left_y + 2][top_left_x + 1] = '|';
                }
                Some('7') => {
                    grid.grid[top_left_y + 1][top_left_x + 1] = '7';
                    grid.grid[top_left_y + 1][top_left_x] = '-';
                    grid.grid[top_left_y + 2][top_left_x + 1] = '|';
                }
                Some('J') => {
                    grid.grid[top_left_y + 1][top_left_x + 1] = 'J';
                    grid.grid[top_left_y + 1][top_left_x] = '-';
                    grid.grid[top_left_y][top_left_x + 1] = '|';
                }
                Some('L') => {
                    grid.grid[top_left_y + 1][top_left_x + 1] = 'L';
                    grid.grid[top_left_y + 1][top_left_x + 2] = '-';
                    grid.grid[top_left_y][top_left_x + 1] = '|';
                }
                Some('|') => {
                    grid.grid[top_left_y][top_left_x + 1] = '|';
                    grid.grid[top_left_y + 1][top_left_x + 1] = '|';
                    grid.grid[top_left_y + 2][top_left_x + 1] = '|';
                }
                Some('-') => {
                    grid.grid[top_left_y + 1][top_left_x] = '-';
                    grid.grid[top_left_y + 1][top_left_x + 1] = '-';
                    grid.grid[top_left_y + 1][top_left_x + 2] = '-';
                }
                _ => {}
            }
        }

        // conditionally fill in spaces surrounding start point 
        grid.grid[start.y][start.x] = 'S';
        [
            (Some(start.x), Some(start.y + 2), '|'),
            (Some(start.x), start.y.checked_sub(2), '|'),
            (start.x.checked_sub(2), Some(start.y), '-'),
            (Some(start.x + 2), Some(start.y), '-'),
        ]
        .into_iter()
        .filter_map(|check| match check {
            (Some(x), Some(y), c) => Some((x, y, c)),
            _ => None,
        })
        .for_each(|(x, y, c)| {
            if grid.get(x, y) == Some(c) {
                let mid_x = (start.x + x) / 2;
                let mid_y = (start.y + y) / 2;
                if let Some(ptr) = grid.get_mut(mid_x, mid_y) {
                    *ptr = c;
                }
            }
        });

        grid
    }

    #[allow(private_interfaces)]
    pub fn pp(&self) -> GridPP {
        GridPP(self)
    }
}

struct GridPP<'a>(&'a Grid);

impl<'a> std::fmt::Display for GridPP<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rowno, row) in self.0.grid.iter().enumerate() {
            for c in row {
                match c {
                    'F' => write!(f, "┌")?,
                    '7' => write!(f, "┐")?,
                    'J' => write!(f, "┘")?,
                    'L' => write!(f, "└")?,
                    '|' => write!(f, "│")?,
                    '-' => write!(f, "─")?,
                    '.' => write!(f, ".")?,
                    _ => write!(f, "{c}")?,
                }
            }
            if rowno < self.0.grid.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rowno, row) in self.grid.iter().enumerate() {
            for c in row {
                write!(f, "{c}")?;
            }
            if rowno < self.grid.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_path() {
        let maze = "\
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        ";
        let grid = Grid::from_str(maze).unwrap();
        let path: Vec<_> = grid.path(grid.start).unwrap().zip(0..100).collect();
        assert_eq!(path.len(), 8);
    }

    #[test]
    fn test_expanded_path() {
        let maze = "\
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
        ";
        let grid = Grid::from_str(maze).unwrap();
        println!("grid\n{grid}");
        let expanded = grid.expand();
        println!("expanded\n{expanded}");
        let path: Vec<_> = grid.path(grid.start).unwrap().collect();
        let expanded_path: Vec<_> = expanded.path(expanded.start).unwrap().collect();
        assert_eq!(path.len() * 3, expanded_path.len());
    }
}
