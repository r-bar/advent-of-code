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
    parsed = parseInput input
    counts = itemCounts parsed.right
    answer = List.walk parsed.left 0 \accum, item ->
        when Dict.get counts item is
            Err KeyNotFound -> accum
            Ok count ->
                (Num.toI64 count)
                * item
                |> Num.add accum
    Stdout.line! (Inspect.toStr answer)

itemCounts : List a -> Dict a U64 where a implements Hash & Eq
itemCounts = \list ->
    List.walk list (Dict.empty {}) \accum, item ->
        Dict.update accum item updateCount

updateCount : Result U64 [Missing] -> Result U64 [Missing]
updateCount = \entry ->
    when entry is
        Err Missing -> Ok 1
        Ok count -> Ok (count + 1)

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
