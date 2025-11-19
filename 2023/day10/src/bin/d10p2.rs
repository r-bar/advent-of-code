use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashSet;
use std::collections::VecDeque;

use day10::{Grid, Point, Direction};

fn edges(grid: &Grid) -> Vec<Point> {
    let mut edges = Vec::new();
    for x in 0..grid.width() {
        edges.push(Point { x, y: 0 });
        edges.push(Point { x, y: grid.height() - 1 });
    }
    for y in 0..grid.height() {
        edges.push(Point { x: 0, y });
        edges.push(Point { x: grid.width() - 1, y });
    }
    edges
}


fn solve(grid: &Grid) -> anyhow::Result<usize> {
    log::debug!("grid:\n{grid}\n");
    let expanded = grid.expand();
    log::debug!("expanded:\n{expanded}\n");
    let path: HashSet<_> = expanded.path(expanded.start)
        .ok_or(anyhow::anyhow!("invalid path start"))?
        .collect();
    let mut to_check: VecDeque<_> = edges(&expanded).into_iter().collect();
    let mut outside: HashSet<Point> = HashSet::new();
    while let Some(pt) = to_check.pop_front() {
        if path.contains(&pt) || outside.contains(&pt) {
            continue;
        }
        outside.insert(pt);
        let neighbors = [
            expanded.rel_pt(pt, Direction::N),
            expanded.rel_pt(pt, Direction::S),
            expanded.rel_pt(pt, Direction::E),
            expanded.rel_pt(pt, Direction::W),
        ].into_iter().flatten();
        to_check.extend(neighbors);
    }
    let mut shown = grid.clone();
    let mut inside = 0;
    for top_left_y in (0..grid.height()).map(|y| y * 3) {
        for top_left_x in (0..grid.width()).map(|x| x * 3) {
            #[allow(clippy::identity_op)]
            let pts = [
                Point { x: top_left_x + 0, y: top_left_y + 0 },
                Point { x: top_left_x + 1, y: top_left_y + 0 },
                Point { x: top_left_x + 2, y: top_left_y + 0 },
                Point { x: top_left_x + 0, y: top_left_y + 1 },
                Point { x: top_left_x + 1, y: top_left_y + 1 },
                Point { x: top_left_x + 2, y: top_left_y + 1 },
                Point { x: top_left_x + 0, y: top_left_y + 2 },
                Point { x: top_left_x + 1, y: top_left_y + 2 },
                Point { x: top_left_x + 2, y: top_left_y + 2 },
            ];
            let path_node = pts.iter().any(|p| path.contains(p));
            let outside_node = pts.iter().any(|p| outside.contains(p));
            if outside_node {
                let ptr = shown.get_mut(top_left_x / 3, top_left_y / 3).unwrap();
                *ptr = 'O';
            } else if !path_node && !outside_node {
                inside += 1;
                *shown.get_mut(top_left_x / 3, top_left_y / 3).unwrap() = 'I';
            }
        }
    }
    log::debug!("solved:\n{shown}\n");
    Ok(inside)
}


fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let grid = Grid::from_str(&buffer)?;
    let inside = solve(&grid)?;
    println!("Inside: {inside}");
    Ok(())
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use std::collections::HashSet;
    use test_case::test_case;

    const EXAMPLE1: &str = "\
OOOOOOOOOOO
OS-------7O
O|F-----7|O
O||OOOOO||O
O||OOOOO||O
O|L-7OF-J|O
O|II|O|II|O
OL--JOL--JO
OOOOOOOOOOO
";

    const EXAMPLE2: &str = "\
OOOOOOOOOO
OS------7O
O|F----7|O
O||OOOO||O
O||OOOO||O
O|L-7F-J|O
O|II||II|O
OL--JL--JO
OOOOOOOOOO
";

    const EXAMPLE3: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE3_EXPECTED: &str = "\
OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO
";

    const EXAMPLE4: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    const EXAMPLE4_EXPECTED: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";


    #[test_case(EXAMPLE1, EXAMPLE1 ; "example1, basic case")]
    #[test_case(EXAMPLE2, EXAMPLE2 ; "example2, squeeze")]
    #[test_case(EXAMPLE3, EXAMPLE3_EXPECTED ; "example3, advanced")]
    #[test_case(EXAMPLE4, EXAMPLE4_EXPECTED ; "example4, advanced with debris")]
    fn test_example(grid: &str, expected: &str) {
        let grid = Grid::from_str(grid).unwrap();
        let expected = Grid::from_str(expected).unwrap();
        let expected_inside: HashSet<_> = expected.iter()
            .filter(|p| expected.get(p.x, p.y) == Some('I'))
            .collect();
        let steps = solve(&grid).unwrap();
        assert_eq!(steps, expected_inside.len());
    }

}
