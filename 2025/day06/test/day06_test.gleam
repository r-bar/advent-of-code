import day06
import gleeunit
import simplifile

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day06.part1(example) == Ok("4277556")
}

pub fn part2_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day06.part2(example) == Ok("3263827")
}
