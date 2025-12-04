import day04
import gleeunit
import simplifile

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day04.part1(example) == Ok("13")
}

pub fn part2_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day04.part2(example) == Ok("")
}
