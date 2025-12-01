import day01
import simplifile
import gleam/io
import gleam/result
import gleeunit

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day01.part1(example) == Ok("3")
}
