import argv
import gleam/bool
import gleam/float
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import simplifile

pub fn part1(input_data: String) -> Result(String, AppError) {
  let on_limit = 2
  use input <- result.try(parse_input(input_data))
  list.map(input.batteries, largest(_, [], 0, on_limit))
  |> sum_values
  |> int.to_string
  |> Ok
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  let on_limit = 12
  use input <- result.try(parse_input(input_data))
  list.map(input.batteries, largest(_, [], 0, on_limit))
  |> sum_values
  |> int.to_string
  |> Ok
}

fn slice(l: List(t), start: Int, stop: Int) -> List(t) {
  list.take(l, stop) |> list.drop(start)
}

/// The idea here is to get the highest digits into the largest position in the
/// final digit. A 9 in the 10s place is worth 10x a 9 in the 1s place. So long
/// as there is a buffer of digits at the end of the list
fn largest(
  bank: List(Int),
  accum: List(#(Int, Int)),
  start_index: Int,
  to_pick: Int,
) -> List(#(Int, Int)) {
  let len = list.length(bank)
  use <- bool.guard(len < to_pick, [])
  use <- bool.guard(to_pick < 1, accum)

  let assert Ok(first) = list.first(bank)
  let init_highest = #(first, start_index)

  let index_range = list.range(start_index, list.length(bank) + start_index)
  let #(highest, highest_index) =
    list.zip(bank, index_range)
    // Only pick the highest in the subset of the list that will allow us to
    // produce a final value even if the last digit in the list is selected.
    // The slice removes the value of the 1st digit which we use to initialize
    // the accumulator, and drops "reserved" values for lower ordinal positions.
    |> slice(1, len - { to_pick - 1 })
    |> list.fold(init_highest, fn(accum, i) {
      let #(value, index) = i
      let #(highest, _) = accum
      case highest < value {
        True -> #(value, index)
        False -> accum
      }
    })

  // Drop all values up to and including the highest found. Lower ordinal values
  // will be selected from the remaining list.
  let remaining = list.drop(bank, highest_index - start_index + 1)
  largest(
    remaining,
    list.append(accum, [#(highest, highest_index)]),
    highest_index,
    to_pick - 1,
  )
}

/// Turns a list of ints into a single integer comprised of those digits
/// 
/// Example:
///     assert int_list_to_digit([3, 5, 7]) == 357
fn int_list_to_digit(values: List(Int)) -> Int {
  use accum, i, index <- list.index_fold(values, 0)
  let power =
    list.length(values) - index - 1
    |> int.to_float
    |> int.power(10, _)
    |> result.lazy_unwrap(fn() { panic })
    |> float.truncate()
  accum + { i * power }
}

/// Consume the result produced by mapping `largest` over the list of banks,
/// turn each list of integers and their indexes into the resulting integer
/// value and total them.
fn sum_values(largest_result: List(List(#(Int, Int)))) {
  largest_result
  |> list.map(list.map(_, pair.first))
  |> list.map(int_list_to_digit)
  |> list.fold(0, int.add)
}

/// The parsed input data structure
type Input {
  Input(batteries: List(List(Int)))
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
      Error(InputError(_, message)) -> Error(InputError(lineno, message))
      Error(e) -> Error(e)
    }
  })
  |> result.map(list.reverse)
  |> result.map(Input)
}

fn parse_line(line: String) -> Result(List(Int), AppError) {
  use nums <- result.try(
    string.to_graphemes(line)
    |> list.try_map(int.parse)
    |> result.replace_error(InputError(-1, "Invalid integer in line: " <> line)),
  )
  case nums {
    [] -> Error(EmptyLine)
    words -> Ok(words)
  }
}

//
// AOC Runtime
//

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
