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
        |> List.map diffLine
        |> List.countIf safe
    Stdout.line! (Inspect.toStr answer)

parseLine : Str -> List I64
parseLine = \line ->
    Str.splitOn line " "
    |> List.map \word ->
        when Str.toI64 word is
            Ok i -> i
            Err _ -> crash "invalid input"

parseInput : Str -> List (List I64)
parseInput = \input ->
    Str.splitOn input "\n"
    |> List.dropIf (\line -> line == "")
    |> List.map parseLine

diffLine : List I64 -> List I64
diffLine = \line ->
    when line is
        [] -> []
        [_] -> []
        [a, b, .. as rest] ->
            [b - a]
            |> List.concat (diffLine (List.prepend rest b))

safe : List I64 -> Bool
safe = \diffs ->
    allPositive = List.all diffs (\x -> x > 0)
    allNegative = List.all diffs (\x -> x < 0)
    lessThan4 = List.all diffs \x ->
        Num.abs x > 0 && Num.abs x < 4
    (allPositive || allNegative) && lessThan4
