import argv
import gleam/bool
import gleam/float
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/pair
import gleam/result
import gleam/string
import gleither.{type Either, Left, Right}
import iv.{type Array}
import simplifile

pub fn part1(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  iv.map(input.ops, fn(item) {
    let #(op, column) = item
    let nums =
      iv.map(input.nums, fn(row) {
        iv_drop(row, column)
        |> first_row_int
      })
    // use op <- result.try(iv.get(input.ops, index))
    // use col_nums <- result.try(iv.try_map(input.nums, fn(row) { iv.get(row, index) }))
    let #(init, op_fn) = case op {
      Add -> #(0, int.add)
      Mul -> #(1, int.multiply)
    }
    iv.fold(nums, init, op_fn)
  })
  |> iv.fold(0, int.add)
  |> int.to_string
  |> Ok
  // |> result.replace_error(CalcError("Calculation error"))
}

pub fn part2(input_data: String) -> Result(String, AppError) {
  use input <- result.try(parse_input(input_data))
  iv.map(input.ops, fn(item) {
    let #(op, column) = item
    let nums = col_ints(input, column)
    // use op <- result.try(iv.get(input.ops, index))
    // use col_nums <- result.try(iv.try_map(input.nums, fn(row) { iv.get(row, index) }))
    let #(init, op_fn) = case op {
      Add -> #(0, int.add)
      Mul -> #(1, int.multiply)
    }
    list.fold(nums, init, op_fn)
  })
  |> iv.fold(0, int.add)
  |> int.to_string
  |> Ok
}

type Operator {
  Add
  Mul
}

/// The parsed input data structure
type Input {
  Input(nums: Array(Array(Either(Int, String))), ops: Array(#(Operator, Int)))
}

fn iv_drop(arr: Array(t), drop: Int) -> Array(t) {
  iv.slice(arr, drop, iv.length(arr) - drop)
  |> result.unwrap(arr)
}

fn col_ints(input: Input, start_column: Int) -> List(Int) {
  let col_nums = iv.filter_map(input.nums, iv.get(_, start_column))
  use <- bool.guard(iv.all(col_nums, gleither.is_right), [])
  let col_int =
    iv.filter_map(col_nums, fn(item) {
      gleither.get_left(item) |> option.to_result(Nil)
    })
    |> iv.reverse()
    |> iv.index_map(fn(digit, index) {
      let power =
        int.power(10, int.to_float(index))
        |> result.lazy_unwrap(fn() { panic })
        |> float.truncate()
      digit * power
    })
    |> iv.fold(0, int.add)
  [col_int, ..col_ints(input, start_column + 1)]
}

fn first_row_int(row: Array(Either(Int, String))) -> Int {
  iv.to_list(row)
  |> list.fold_until([], fn(accum, item) {
    case item, accum {
      Left(d), _ -> list.Continue(list.prepend(accum, d))
      Right(_), [] -> list.Continue(accum)
      Right(_), _ -> list.Stop(accum)
    }
  })
  |> list.index_map(fn(digit, index) {
    let power =
      int.power(10, int.to_float(index))
      |> result.lazy_unwrap(fn() { panic })
      |> float.truncate()
    digit * power
  })
  |> list.fold(0, int.add)
}

fn parse_input(input_data: String) -> Result(Input, AppError) {
  let init = Input(iv.new(), iv.new())
  input_data
  |> string.trim()
  |> string.split("\n")
  |> list.index_map(pair.new)
  |> list.try_fold(init, fn(accum, input) {
    let #(line, _lineno) = input
    result.map(parse_num_line(line), fn(num_line) {
      Input(..accum, nums: iv.append(accum.nums, num_line))
    })
    |> result.lazy_or(fn() {
      use ops <- result.try(parse_op_line(line))
      Ok(Input(..accum, ops:))
    })
  })
}

fn parse_num_line(line: String) -> Result(Array(Either(Int, String)), AppError) {
  let parsed =
    string.to_graphemes(line)
    |> iv.from_list
    |> iv.index_map(pair.new)
    |> iv.map(fn(i) {
      let #(char, _idx) = i
      case int.parse(char) {
        Ok(d) -> Left(d)
        Error(_) -> Right(char)
      }
    })
  let num_ints =
    iv.fold(parsed, 0, fn(accum, i) {
      case i {
        Left(_) -> accum + 1
        Right(_) -> accum
      }
    })
  case num_ints {
    0 -> Error(InputError(-1, "No numbers parsed in line"))
    _ -> Ok(parsed)
  }
}

fn parse_op_line(line: String) -> Result(Array(#(Operator, Int)), AppError) {
  let parsed =
    string.to_graphemes(line)
    |> iv.from_list
    |> iv.index_fold(iv.new(), fn(accum, char, index) {
      case char {
        "+" -> iv.append(accum, #(Add, index))
        "*" -> iv.append(accum, #(Mul, index))
        _ -> accum
      }
    })
  use <- bool.guard(
    iv.length(parsed) == 0,
    Error(InputError(-1, "No operators parsed in line")),
  )
  Ok(parsed)
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
