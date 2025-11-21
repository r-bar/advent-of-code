import argv
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

fn part1(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  input.lines
  |> list.map(string.join(_, " "))
  |> string.join("\n")
  |> Ok
}

fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  input.lines
  |> list.map(string.join(_, " "))
  |> string.join("\n")
  |> Ok
}

/// The parsed input data structure
type Input {
  Input(lines: List(List(String)))
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  input_data
  |> string.trim()
  |> string.split("\n")
  |> list.map(string.split(_, " "))
  |> Input
  |> Ok
}


type App {
  App(input_file: String, part: Part)
}

type Part {
  Part1
  Part2
}

pub type AppError {
  ArgumentError(message: String)
  FileError(simplifile.FileError)
}

fn usage() -> String {
  "Usage: day02 <part> <input_file>"
}

fn parse_args(argv: argv.Argv) -> Result(App, AppError) {
  case argv.arguments {
    ["1", input_file] | ["part1", input_file] -> {
      Ok(App(input_file:, part: Part1))
    }
    ["2", input_file] | ["part2", input_file] -> {
      Ok(App(input_file:, part: Part2))
    }
    [bad_part, _input_file] -> {
      Error(ArgumentError("Unknown part: " <> bad_part))
    }
    _ -> Error(ArgumentError("Bad arguments"))
  }
}

fn run() -> Result(String, AppError) {
  use app <- result.try(parse_args(argv.load()))
  use input_data <- result.try(
    simplifile.read(app.input_file)
    |> result.map_error(FileError),
  )
  case app.part {
    Part1 -> part1(input_data)
    Part2 -> part2(input_data)
  }
}

pub fn main() {
  case run() {
    Ok(res) -> io.println(res)
    Error(ArgumentError(msg)) -> {
      io.println_error(msg)
      io.println_error(usage())
    }
    Error(e) -> io.println_error(string.inspect(e))
  }
}
