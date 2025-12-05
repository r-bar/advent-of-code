import gleam/bool
import argv
import gleam/set
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import gleam/option.{type Option, Some, None}
import gleither.{type Either, Left, Right}
import simplifile
import iv.{type Array}

pub fn part1(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  list.filter(input.ids, is_fresh(input.fresh, _))
  |> list.length()
  |> int.to_string()
  |> Ok
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  let work = list.sort(input.fresh, fn(a, b) { int.compare(a.lower, b.lower)})
  let merged_state = part2_help(State(work, iv.new()))
  iv.map(merged_state.merged, range_size)
  |> iv.fold(0, int.add)
  |> int.to_string()
  |> Ok
}

type State {
  State(
  work: List(Range),
  merged: Array(Range),
)
}

fn part2_help(state: State) -> State {
  use <- bool.guard(state.work == [], state)
  let assert [fresh, ..remaining] = state.work
  let new_merged = case iv.find_index(state.merged, overlap(fresh, _)) {
    Ok(overlap_index) -> {
      let assert Ok(overlap) = iv.get(state.merged, overlap_index)
      let assert Right(union) = union_ranges(fresh, overlap)
      iv.set(state.merged, overlap_index, union)
      |> result.lazy_unwrap(fn() { panic })
    }
    Error(_) -> iv.append(state.merged, fresh)
  }
  part2_help(State(remaining, new_merged))
}

/// The parsed input data structure
type Input {
  Input(
    parser_mode: ParserMode,
    fresh: List(Range),
    ids: List(Int),
  )
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  input_data
  |> string.trim()
  |> string.split("\n")
  |> list.index_map(pair.new)
  |> list.try_fold(Input(FreshRange, [], []), fn(accum, input) {
    let #(line, lineno) = input
    let parser = case accum.parser_mode {
      FreshRange -> {
        case parse_fresh_range_line(line) {
          Ok(v) -> Ok(Input(..accum, fresh: list.prepend(accum.fresh, v)))
          Error(EmptyLine) -> Ok(Input(..accum, parser_mode: Id))
          Error(InputError(_, message)) -> Error(InputError(lineno, message))
          Error(e) -> Error(e)
        }
      }
      Id -> {
        case parse_id(line) {
          Ok(v) -> Ok(Input(..accum, ids: list.prepend(accum.ids, v)))
          Error(EmptyLine) -> Ok(accum)
          Error(InputError(_, message)) ->
            Error(InputError(lineno, message))
          Error(e) -> Error(e)
        }
      }
    }
  })
  |> result.map(fn(input) { Input (
    ..input,
    fresh: list.reverse(input.fresh),
    ids: list.reverse(input.ids),
  )})
}

fn range_size(range: Range) -> Int {
  range.upper - range.lower + 1
}

fn parse_fresh_range_line(line: String) -> Result(Range, AppError) {
  case string.split(line, "-") |> list.try_map(int.parse) {
    Ok([lower, upper]) -> Ok(Range(lower, upper))
    Ok([]) -> Error(EmptyLine)
    Ok(_) -> Error(InputError(-1, "Invalid range format: " <> line))
    Error(_) if line == "" -> Error(EmptyLine)
    Error(_) -> Error(InputError(-1, "Invalid integer in range: " <> line))
  }
}

fn parse_id(line: String) -> Result(Int, AppError) {
  int.parse(line)
  |> result.replace_error(InputError(-1, "Invalid integer"))
}

fn is_fresh(fresh: List(Range), id: Int) -> Bool {
  use Range(lower, upper) <- list.any(fresh)
  lower <= id && id <= upper
}

fn overlap(a: Range, b: Range) -> Bool {
  a.lower <= b.lower && b.lower <= a.upper
  || b.lower <= a.lower && a.lower <= b.upper
}

fn union_ranges(a: Range, b: Range) -> Either(#(Range, Range), Range) {
  case overlap(a, b) {
    True -> {
      let lower = int.min(a.lower, b.lower)
      let upper = int.max(a.upper, b.upper)
      Right(Range(lower, upper))
    }
    False -> Left(#(a, b))
  }
}

type Range {
  Range(lower: Int, upper: Int)
}

type ParserMode {
  FreshRange
  Id
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
