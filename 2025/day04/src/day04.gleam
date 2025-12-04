import gleam/bool
import argv
import gleam/set
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import iv.{type Array}
import simplifile

pub fn part1(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  let movable = input_fold(input, 0, fn(accum, coord, val) {
    use <- bool.guard(val == 0, accum)
    let full_neighbors =
      neighbors(coord)
      |> list.map(lookup(input, _))
      |> list.fold(0, int.add)
    case full_neighbors < 4 {
      True -> accum + 1
      False -> accum
    }
  })
  Ok(string.inspect(movable))
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  let removed = part2_help(input, 0)
  Ok(string.inspect(removed))
}

fn part2_help(input: Input, removed: Int) -> Int {
  let movable = input_fold(input, set.new(), fn(accum, coord, val) {
    use <- bool.guard(val == 0, accum)
    let full_neighbors =
      neighbors(coord)
      |> list.map(lookup(input, _))
      |> list.fold(0, int.add)
    case full_neighbors < 4 {
      True -> set.insert(accum, coord)
      False -> accum
    }
  })
  case set.size(movable) {
    0 -> removed
    _ -> {
      let new_input = input_map(input, fn(coord, val) {
        case set.contains(movable, coord) {
          True -> 0
          False -> val
        }
      })
      part2_help(new_input, removed + set.size(movable))
    }
  }
  
}

/// The parsed input data structure
type Input {
  Input(grid: Array(Array(Int)))
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  input_data
  |> string.trim()
  |> string.split("\n")
  |> iv.from_list
  |> iv.index_map(pair.new)
  |> iv.try_fold(iv.new(), fn(accum, input) {
    let #(line, lineno) = input
    case parse_line(line) {
      Ok(v) -> Ok(iv.append(accum, v))
      Error(EmptyLine) -> Ok(accum)
      Error(InputError(_, message)) -> Error(InputError(lineno, message))
      Error(e) -> Error(e)
    }
  })
  |> result.map(Input)
}

fn input_map(input: Input, pred: fn(Coordinate, Int) -> Int) -> Input {
  iv.index_map(input.grid, fn(row, y) {
    iv.index_map(row, fn(val, x) {
      pred(Coordinate(x, y), val)
    })
  })
  |> Input
}

fn input_fold(input: Input, init: t, pred: fn(t, Coordinate, Int) -> t) -> t {
  use row_accum, row, y <- iv.index_fold(input.grid, init)
  use val_accum, val, x <- iv.index_fold(row, row_accum)
  pred(val_accum, Coordinate(x, y), val)
}

fn parse_line(line: String) -> Result(Array(Int), AppError) {
  use char <- iv.try_map(iv.from_list(string.split(line, "")))
  case char {
    "." -> Ok(0)
    "@" -> Ok(1)
    bad -> Error(InputError(-1, "Invalid input: " <> bad))
  }
}

fn neighbors(coord: Coordinate) -> List(Coordinate) {
  [
    n_of(coord),
    s_of(coord),
    e_of(coord),
    w_of(coord),
    nw_of(coord),
    ne_of(coord),
    sw_of(coord),
    se_of(coord),
  ]
}

fn lookup(input: Input, coord: Coordinate) -> Int {
  input.grid
  |> iv.get(coord.y)
  |> result.lazy_unwrap(iv.new)
  |> iv.get(coord.x)
  |> result.unwrap(0)
}

type Coordinate {
  Coordinate(x: Int, y: Int)
}

fn n_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x, coord.y - 1)
}

fn s_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x, coord.y + 1)
}

fn e_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x + 1, coord.y)
}

fn w_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x - 1, coord.y)
}

fn se_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x + 1, coord.y + 1)
}

fn sw_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x - 1, coord.y + 1)
}

fn nw_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x - 1, coord.y - 1)
}

fn ne_of(coord: Coordinate) -> Coordinate {
  Coordinate(coord.x + 1, coord.y - 1)
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
