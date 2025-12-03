import day03
import gleeunit
import simplifile

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day03.part1(example) == Ok("357")
}

pub fn part2_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day03.part2(example) == Ok("3121910778619")
}
