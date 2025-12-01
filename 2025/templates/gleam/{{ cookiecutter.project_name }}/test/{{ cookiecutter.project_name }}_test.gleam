import {{ cookiecutter.project_name }}
import gleeunit
import simplifile

pub fn main() -> Nil {
  gleeunit.main()
}

pub fn part1_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert {{ cookiecutter.project_name }}.part1(example) == Ok("")
}

pub fn part2_test() {
  let assert Ok(example) = simplifile.read("example.txt")
  assert {{ cookiecutter.project_name }}.part2(example) == Ok("")
}
