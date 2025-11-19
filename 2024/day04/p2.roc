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
    ys = List.range { start: At 0, end: Before length }
    xs = List.range { start: At 0, end: Before width }
    answer =
        List.walk ys [] \accum, y ->
            List.walk xs [] \rowAccum, x ->
                search grid (x, y) |> List.concat rowAccum
            |> List.concat accum
        # only used for display
        # |> List.map \{ coords, content } ->
        #    { coords, content: Str.fromUtf8 content |> Result.withDefault "?" }
        # |> dbg
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
Matcher : { genCoords : Coord, U64 -> Result (List Coord) [OutOfBounds], match : List U8, List U8 -> Bool }

needles = [
    ['M', 'A', 'S'],
]

matchers : List Matcher
matchers = [
    { genCoords: genCoordsX, match: matchX },
]

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

genCoordsX : Coord, U64 -> Result (List Coord) [OutOfBounds]
genCoordsX = \(origX, origY), len ->
    rightOrigX = origX + (len - 1)
    left = try diagLCoords (rightOrigX, origY) len
    right = try diagRCoords (origX, origY) len
    Ok (List.concat left right)

matchX : List U8, List U8 -> Bool
matchX = \haystack, needle ->
    left = List.sublist haystack { start: 0, len: List.len needle }
    right = List.sublist haystack { start: List.len needle, len: List.len needle }
    (left == needle || List.reverse left == needle)
    && (right == needle || List.reverse right == needle)

getCoord : Grid, Coord -> Result U8 [OutOfBounds]
getCoord = \grid, (x, y) ->
    List.get grid y
    |> Result.try \row -> List.get row x

search : Grid, Coord -> List Match
search = \grid, rootCoord ->
    List.keepOks matchers \matcher ->
        # TODO: the length of 3 should come from the needles
        coords = try matcher.genCoords rootCoord 3
        Ok { coords, matcher }
    # |> dbg
    |> List.keepOks \{ coords, matcher } ->
        content = try List.mapTry coords \coord ->
            getCoord grid coord
        matches = List.any needles \needle -> matcher.match content needle
        if matches then
            Ok { coords, content }
        else
            Err NotFound
