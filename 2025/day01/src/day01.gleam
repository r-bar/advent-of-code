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
  input.lines
  |> list.fold([50], fn(accum, i) {
    let #(direction, magnitude) = i
    let vec = case direction {
      Left -> magnitude * -1
      Right -> magnitude
    }
    let last = list.first(accum) |> result.unwrap(50)
    // Turns out int.modulo and the % operator give different results for
    // negative numbers. int.modulo(-18, 100) == Ok(82) while -18 % 100 == -18
    int.modulo(vec + last, 100)
    |> result.lazy_unwrap(fn() {panic})
    |> list.prepend(accum, _)
  })
  |> list.count(fn(i) { i == 0 })
  |> int.to_string()
  |> Ok
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  todo
  // input.lines
  // |> list.map(string.join(_, " "))
  // |> string.join("\n")
  // |> Ok
}

/// The parsed input data structure
type Input {
  Input(lines: List(#(Direction, Int)))
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
      Error("Empty line") -> Ok(accum)
      Error(err) ->
        Error(InputError("Line " <> int.to_string(lineno + 1) <> ": " <> err))
    }
  })
  |> result.map(list.reverse)
  |> result.map(Input)
}

type Direction {
  Left
  Right
}

fn parse_line(line: String) -> Result(#(Direction, Int), String) {
  use #(direction, int_chars) <- result.try(case string.to_graphemes(line) {
    ["L", ..int_chars] -> Ok(#(Left, int_chars))
    ["R", ..int_chars] -> Ok(#(Right, int_chars))
    [c, ..] -> Error("Invalid direction: " <> c)
    [] -> Error("Empty line")
  })
  let int_str = string.join(int_chars, "")
  use i <- result.try(
    int.parse(int_str)
    |> result.replace_error("Invalid magnitude: " <> int_str),
  )
  Ok(#(direction, i))
}

type App {
  App(input_file: String, part: Part)
}

type Part {
  Part1
  Part2
}

pub type AppError {
  InputError(message: String)
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
  // echo int.modulo(-18, 100)
  // echo -18 % 100
  case run() {
    Ok(res) -> io.println(res)
    Error(ArgumentError(msg)) -> {
      io.println_error(msg)
      io.println_error(usage())
    }
    Error(e) -> io.println_error(string.inspect(e))
  }
}
