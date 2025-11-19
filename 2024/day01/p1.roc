app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import pf.File
import pf.Arg

main =
    args = Arg.list! {}
    inputFile =
        when args is
            [_progname, filename] -> filename
            [progname, ..] -> crash "Usage: $(progname) <input_file>"
            [] -> crash "unreachable"
    input = File.readUtf8! inputFile
    answer =
        parseInput input
        |> leftAndRight List.sortAsc
        |> \{ left, right } -> List.map2 left right (\a, b -> Num.abs (a - b))
        |> List.sum
    Stdout.line! (Inspect.toStr answer)

leftAndRight : { left : a, right : a }, (a -> b) -> { left : b, right : b }
leftAndRight = \{ left, right }, f -> { left: f left, right: f right }

parseLine : Str -> { left : I64, right : I64 }
parseLine = \line ->
    Str.splitOn line "   "
    |> List.map Str.toI64
    |> \parts ->
        when parts is
            [Ok a, Ok b] -> { left: a, right: b }
            _ -> crash "invalid input"

rotate : List { left : a, right : b } -> { left : List a, right : List b }
rotate = \pairs ->
    left = List.map pairs (\pair -> pair.left)
    right = List.map pairs (\pair -> pair.right)
    { left, right }

parseInput : Str -> { left : List I64, right : List I64 }
parseInput = \input ->
    Str.splitOn input "\n"
    |> List.dropIf (\line -> line == "")
    |> List.map parseLine
    |> rotate
