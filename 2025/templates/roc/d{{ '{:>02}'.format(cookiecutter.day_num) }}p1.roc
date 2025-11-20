app [main!] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.20.0/X73hGh05nNTkDHU06FHC0YfFaQB1pimX7gncRcao5mU.tar.br" }

import pf.Stdout
import pf.File
import pf.Arg

main! = |args|
    input_file =
        when List.map(args, Arg.display) is
            [_progname, filename] -> filename
            [progname, ..] -> crash "Usage: ${progname} <input_file>"
            [] -> crash "unreachable"
    input = File.read_utf8!(input_file)?
    answer = parse_input(input)
    Stdout.line!(Inspect.to_str answer)

parse_line : Str -> List Str
parse_line = |line|
    Str.split_on line " "

parse_input : Str -> List List Str
parse_input = |input|
    Str.split_on input "\n"
    |> List.drop_if(|line| line == "")
    |> List.map parse_line
