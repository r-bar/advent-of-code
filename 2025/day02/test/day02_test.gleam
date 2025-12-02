import day02
import gleeunit
import simplifile

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn invalid_id_test() {
  assert day02.invalid_id(11)
  assert day02.invalid_id(22)
  assert day02.invalid_id(111) == False
  assert day02.invalid_id(123123)
  assert day02.invalid_id(12351234) == False
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day02.part1(example) == Ok("1227775554")
}

pub fn part2_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day02.part2(example) == Ok("")
}
