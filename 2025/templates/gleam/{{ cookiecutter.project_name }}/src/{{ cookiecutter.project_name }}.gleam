import argv
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import simplifile

pub fn part1(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  Ok(string.inspect(input))
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  Ok(string.inspect(input))
}

/// The parsed input data structure
type Input {
  Input(lines: List(List(String)))
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  input_data
  |> string.trim()
  |> string.split("\n")
  |> list.index_map(pair.new)
  |> list.try_fold([], fn(accum, input) {
    let #(line, lineno) = input
    case parse_line(line) {
      Ok(v) -> Ok([v, ..accum])
      Error(EmptyLine) -> Ok(accum)
      Error(InputError(_, message)) ->
        Error(InputError(lineno, message))
      Error(e) -> Error(e)
    }
  })
  |> result.map(list.reverse)
  |> result.map(Input)
}

fn parse_line(line: String) -> Result(List(String), AppError) {
  case string.split(line, " ") {
    [] -> Error(EmptyLine)
    ["error"] -> Error(InputError(-1, "Invalid input"))
    words -> Ok(words)
  }
}

type App {
  App(input_file: String, part: Part)
}

type Part {
  Part1
  Part2
}

pub type AppError {
  EmptyLine
  InputError(lineno: Int, message: String)
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
    Error(InputError(lineno, message)) ->
      io.println_error("Line " <> int.to_string(lineno) <> ": " <> message)
    Error(e) -> io.println_error(string.inspect(e))
  }
}
