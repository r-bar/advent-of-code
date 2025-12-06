import gleam/bool
import argv
import gleam/int
import gleam/io
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import simplifile
import iv.{type Array}

pub fn part1(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  iv.range(0, iv.length(input.ops) - 1)
  |> iv.map(fn(index) {
    let assert Ok(op) = iv.get(input.ops, index)
    let assert Ok(col_nums) = iv.try_map(input.nums, iv.get(_, index))
    // use op <- result.try(iv.get(input.ops, index))
    // use col_nums <- result.try(iv.try_map(input.nums, fn(row) { iv.get(row, index) }))
    let #(init, op_fn) = case op {
      Add -> #(0, int.add)
      Mul -> #(1, int.multiply)
    }
    iv.fold(col_nums, init, op_fn)
  })
  |> iv.fold(0, int.add)
  |> int.to_string
  |> Ok
  // |> result.replace_error(CalcError("Calculation error"))
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  Ok(string.inspect(input))
}

type Operator {
  Add
  Mul
}

/// The parsed input data structure
type Input {
  Input(nums: Array(Array(Int)), ops: Array(Operator))
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  input_data
  |> string.trim()
  |> string.split("\n")
  |> list.index_map(pair.new)
  |> list.try_fold(Input(iv.new(), iv.new()), fn(accum, input) {
    let #(line, lineno) = input
    result.map(parse_num_line(line), fn(num_line) { Input(..accum, nums: iv.append(accum.nums, num_line))})
    |> result.lazy_or(fn() {
      use ops <- result.try(parse_op_line(line))
      Ok(Input(..accum, ops:))
    })
  })
  // |> result.try(check_input)
}

fn parse_num_line(line: String) -> Result(Array(Int), AppError) {
  string.to_graphemes(line)
  |> iv.from_list
  |> iv.filter(fn(s) { s != "" })
  |> iv.try_map(int.parse)
  |> result.replace_error(InputError(-1, "Invalid input"))
}

fn parse_op_line(line: String) -> Result(Array(Operator), AppError) {
  string.split(line, " ")
  |> iv.from_list
  |> iv.filter(fn(s) { s != "" })
  |> iv.try_map(fn(s) {
    case s {
      "*" -> Ok(Mul)
      "+" -> Ok(Add)
      op -> Error(InputError(-1, "Invalid operator: " <> op))
    }
  })
}

fn check_input(input: Input) -> Result(Input, AppError) {
  use <- bool.guard(result.is_error(iv.first(input.nums)), Error(InputError(-1, "No numbers parsed")))
  let assert [nums_head, ..nums_tail] = iv.to_list(input.nums)
  use <- bool.guard(!list.all(nums_tail, fn(nums) { iv.length(nums) != iv.length(nums_head)}), Error(InputError(-1, "Mismatched num line lengths")))
  use <- bool.guard(iv.length(nums_head) != iv.length(input.ops), Error(InputError(-1, "Mismatched number and operator lenghts")))
  Ok(input)
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
  CalcError(message: String)
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
