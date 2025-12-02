import day02
import gleeunit
import simplifile

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn invalid_id_pt1_test() {
  assert day02.invalid_id_pt1(11)
  assert day02.invalid_id_pt1(22)
  assert day02.invalid_id_pt1(111) == False
  assert day02.invalid_id_pt1(123123)
  assert day02.invalid_id_pt1(12351234) == False
}

pub fn invalid_id_pt2_test() {
  assert day02.invalid_id_pt2(11)
  assert day02.invalid_id_pt2(22)
  assert day02.invalid_id_pt2(111)
  assert day02.invalid_id_pt2(123123)
  assert day02.invalid_id_pt2(123412341234)
  assert day02.invalid_id_pt2(12341234123) == False
  assert day02.invalid_id_pt2(333333333333)
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day02.part1(example) == Ok("1227775554")
}

pub fn part2_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert day02.part2(example) == Ok("4174379265")
}
