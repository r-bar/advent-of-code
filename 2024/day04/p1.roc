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
    grid = parseInput input
    length = List.len grid
    width = List.first grid |> Result.withDefault [] |> List.len
    answer =
        List.walk (List.range { start: At 0, end: Before length }) [] \accum, y ->
            List.walk (List.range { start: At 0, end: Before width }) [] \rowAccum, x ->
                search grid (x, y) |> List.concat rowAccum
            |> List.concat accum
        |> List.map \{ coords, content } ->
            { coords, content: Str.fromUtf8 content |> Result.withDefault "?" }
        |> List.len
    Stdout.line! (Inspect.toStr answer)

parseLine : Str -> List U8
parseLine = \line ->
    Str.toUtf8 line

parseInput : Str -> Grid
parseInput = \input ->
    Str.splitOn input "\n"
    |> List.dropIf (\line -> line == "")
    |> List.map parseLine

Coord : (U64, U64)
Grid : List (List U8)
Match : { coords : List Coord, content : List U8 }

needles = [
    ['X', 'M', 'A', 'S'],
]

horizCoords : Coord, U64 -> Result (List Coord) [OutOfBounds]
horizCoords = \(x, y), len ->
    List.range { start: At 0, end: Before len }
    |> List.map (\i -> (x + i, y))
    |> Ok

vertCoords : Coord, U64 -> Result (List Coord) [OutOfBounds]
vertCoords = \(x, y), len ->
    List.range { start: At 0, end: Before len }
    |> List.map (\i -> (x, y + i))
    |> Ok

diagRCoords : Coord, U64 -> Result (List Coord) [OutOfBounds]
diagRCoords = \(x, y), len ->
    List.range { start: At 0, end: Before len }
    |> List.map (\i -> (x + i, y + i))
    |> Ok

diagLCoords : Coord, U64 -> Result (List Coord) [OutOfBounds]
diagLCoords = \(x, y), len ->
    List.range { start: At 0, end: Before len }
    |> List.mapTry \i ->
        newX = try Num.subChecked x i
        Ok (newX, y + i)
    |> Result.mapErr \_ -> OutOfBounds

getCoord : Grid, Coord -> Result U8 [OutOfBounds]
getCoord = \grid, (x, y) ->
    List.get grid y
    |> Result.try \row -> List.get row x

search : Grid, Coord -> List Match
search = \grid, rootCoord ->
    coordFns = [
        horizCoords,
        vertCoords,
        diagLCoords,
        diagRCoords,
    ]
    List.keepOks coordFns \genCoords ->
        # TODO: the length of 4 should come from the needles
        genCoords rootCoord 4
    |> List.keepOks \sliceCoords ->
        content = try List.mapTry sliceCoords \coord ->
            getCoord grid coord
        Ok { coords: sliceCoords, content }
    |> List.keepIf \{ content } ->
        List.contains needles content
        || List.contains needles (List.reverse content)
