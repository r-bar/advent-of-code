use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use day10::Grid;

const MAX_STEPS: usize = 1000000;


fn solve(grid: &Grid) -> anyhow::Result<usize> {
    let mut ends = grid.start_points(grid.start);
    let mut prev = vec![grid.start; ends.len()];
    for step in 1..MAX_STEPS {
        for i in 0..ends.len() {
            let current_pt = ends[i];
            let prev_pt = prev[i];
            let next_pt = grid.get_next(prev_pt, current_pt).ok_or(anyhow::anyhow!("No next point"))?;
            if ends.contains(&next_pt) {
                return Ok(step + 1);
            }
            prev[i] = current_pt;
            ends[i] = next_pt;
        }
    }
    Err(anyhow::anyhow!("No solution found in {MAX_STEPS} steps"))
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
    let steps = solve(&grid)?;
    println!("Steps: {steps}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    const EXAMPLE: &str = include_str!("../../example.txt");
    const EXAMPLE2: &str = include_str!("../../example2.txt");

    #[test]
    fn test_example() {
        let grid = Grid::from_str(EXAMPLE).unwrap();
        let steps = solve(&grid).unwrap();
        assert_eq!(steps, 8);
    }

    #[test]
    fn test_example2() {
        let grid = Grid::from_str(EXAMPLE2).unwrap();
        let steps = solve(&grid).unwrap();
        assert_eq!(steps, 4);
    }
}
