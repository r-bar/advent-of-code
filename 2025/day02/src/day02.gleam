import gleam/bool
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
  input.ranges
  |> list.flat_map(fn(id_range) {
    let #(min, max) = id_range
    range_fold(min, max + 1, [], fn(accum, id) {
      case invalid_id(id) {
        True -> [id, ..accum]
        False -> accum
      }
    })
  })
  |> list.fold(0, int.add)
  |> int.to_string
  |> Ok
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  Ok(string.inspect(input))
}

/// The parsed input data structure
type Input {
  Input(ranges: List(#(Int, Int)))
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  input_data
  |> string.trim()
  |> string.split(",")
  |> list.index_map(pair.new)
  |> list.try_fold([], fn(accum, input) {
    let #(range, setno) = input
    case parse_range(range) {
      Ok(v) -> Ok([v, ..accum])
      Error(EmptyRange) -> Ok(accum)
      Error(InputError(_, message)) -> Error(InputError(setno, message))
      Error(e) -> Error(e)
    }
  })
  |> result.map(list.reverse)
  |> result.map(Input)
}

fn parse_range(input: String) -> Result(#(Int, Int), AppError) {
  case string.split(input, "-") {
    [left_str, right_str] -> {
      use left <- result.try(
        int.parse(left_str)
        |> result.replace_error(InputError(
          -1,
          "Bad integer on left side of range: " <> left_str,
        )),
      )
      use right <- result.try(
        int.parse(right_str)
        |> result.replace_error(InputError(
          -1,
          "Bad integer on right side of range: " <> right_str,
        )),
      )
      Ok(#(left, right))
    }
    [] -> Error(EmptyRange)
    _ -> Error(InputError(-1, "Bad range format: " <> input))
  }
}

fn range_fold(start: Int, stop: Int, init: t, pred: fn(t, Int) -> t) -> t {
  case start < stop {
    True -> range_fold(start + 1, stop, pred(init, start), pred)
    False -> init
  }
}

pub fn invalid_id(id: Int) -> Bool {
  let id_str = int.to_string(id)
  let len = string.length(id_str)
  use <- bool.guard(len % 2 != 0, False)
  let half = len / 2
  let left_half = string.slice(id_str, 0, half)
  let right_half = string.slice(id_str, half, half)
  left_half == right_half
}

type App {
  App(input_file: String, part: Part)
}

type Part {
  Part1
  Part2
}

pub type AppError {
  EmptyRange
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
