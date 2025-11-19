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
    answer = parseInput input
    Stdout.line! (Inspect.toStr answer)

parseLine : Str -> List Str
parseLine = \line ->
    Str.splitOn line " "

parseInput : Str -> List List Str
parseInput = \input ->
    Str.splitOn input "\n"
    |> List.dropIf (\line -> line == "")
    |> List.map parseLine
