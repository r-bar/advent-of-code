app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br" }

import pf.Stdout
import pf.Stderr
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
    #printerData : PrinterData
    parseResult = parseInput input
    #dbg parseResult
    when parseResult is
        Ok printerData ->
            answer = solve printerData
            Stdout.line! (Inspect.toStr answer)
        Err error ->
            Stderr.line! "Parse error:"
            Task.err! (Exit 1 (Inspect.toStr error))


PrinterData : {
    pageOrdering: List (I64, I64),
    pageProduction: List (List I64),
    section: [Ordering, Production],
}
ParseResult : Result PrinterData [ParseError Str U64 Str]

emptyPrinterData : PrinterData
emptyPrinterData = {
    pageOrdering: [],
    pageProduction: [],
    section: Ordering,
}

solve : PrinterData -> _
solve = \printerData ->
    sorted = try topoSort printerData.pageOrdering
    List.keepIf printerData.pageProduction \line ->
        start = {prevIndex: -1, isCorrect: Bool.true}
        List.walkUntil line start \{isCorrect, prevIndex}, page ->
            foundIndex =
                List.findFirstIndex sorted \i -> i == page
                |> Result.map \i -> Num.toI64 i
            #dbg {line, prevIndex, foundIndex}
            when foundIndex is
                Err NotFound -> Continue {isCorrect, prevIndex}
                Ok pageIndex if prevIndex < pageIndex -> Continue {isCorrect, prevIndex: pageIndex}
                Ok pageIndex -> Break {isCorrect: Bool.false, prevIndex: pageIndex}
        |> .isCorrect
    |> \correctLines -> dbg correctLines
    |> List.map middleNumber
    |> List.sum
    |> Ok
            
parsePrinterLines : ParseResult, Str, U64 -> [Continue ParseResult, Break ParseResult]
parsePrinterLines = \accum, line, lineno ->
    when (accum, line) is
        (Ok ({section : Ordering} as printerData), "") ->
            Ok { printerData & section: Production }
            |> Continue

        (Ok ({section : Ordering} as printerData), _) -> 
            Str.splitOn line "|"
            |> List.mapTry Str.toI64
            |> \ordering ->
                when ordering is
                    Ok [left, right] -> 
                        Ok {
                            printerData &
                            pageOrdering: List.append printerData.pageOrdering (left, right)
                        }
                        |> Continue
                    _ ->
                        Err (ParseError "Invalid page ordering" lineno line)
                        |> Break

        (Ok {section: Production}, "") ->
            Continue accum

        (Ok ({section: Production} as printerData), _) -> 
            Str.splitOn line ","
            |> List.mapTry Str.toI64
            |> \r ->
                when r is
                    Ok production ->
                        Ok {
                            printerData &
                            pageProduction: List.append printerData.pageProduction production
                        }
                        |> Continue
                    Err _ ->
                        Err (ParseError "Invalid production line" lineno line)
                        |> Break

        (Err _, _) -> Break accum

parseInput : Str -> ParseResult
parseInput = \input ->
    Str.splitOn input "\n"
    #|> List.dropIf (\line -> line == "")
    |> List.walkWithIndexUntil (Ok emptyPrinterData) parsePrinterLines

TopoSortState a : {
    sorted: List a,
    incoming: Dict a (Set a),
    outgoing: Dict a (Set a),
    roots: List a,
} where a implements Eq & Hash

# implementation adapted from Kahn's algorithm
# https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
topoSort : List (a, a) -> Result (List a) [CycleDetected (TopoSortState a)] where a implements Eq & Hash & Inspect
topoSort = \edges ->
    # Outgoing keys will contain all nodes in the graph
    outgoing =
        List.walk edges (Dict.empty {}) \accum, (left, right) ->
            Dict.update accum left \existing ->
                when existing is
                    Ok set -> Ok (Set.insert set right)
                    Err Missing -> Ok (Set.single right)
            |> Dict.update right \existing ->
                when existing is
                    Ok set -> Ok set
                    Err Missing -> Ok (Set.empty {})
    nodeCount = Dict.len outgoing
    incoming =
        List.walk edges (Dict.withCapacity nodeCount) \accum, (left, right) ->
            Dict.update accum right \existing ->
                when existing is
                    Ok set -> Ok (Set.insert set left)
                    Err Missing -> Ok (Set.single left)
    roots = List.keepIf (Dict.keys outgoing) \node ->
        when Dict.get incoming node is
            Err KeyNotFound -> Bool.true
            Ok set if Set.isEmpty set -> Bool.true
            _ -> Bool.false
    sorted = List.withCapacity nodeCount
    state = {sorted, incoming, outgoing, roots}
    dbg state
    topoSortHelp state

topoSortHelp : TopoSortState a -> Result (List a) [CycleDetected (TopoSortState a)] where a implements Eq & Hash & Inspect
topoSortHelp = \state ->
    when state.roots is
        [] if Dict.isEmpty state.incoming -> Ok state.sorted
        [] -> Err (CycleDetected state)
        [root, .. as remainingRoots] ->
            rootOutgoing =
                Dict.get state.outgoing root
                # should be unreachable unless state is corrupted
                |> Result.withDefault (Set.empty {})
            newState = {
                state &
                roots: remainingRoots,
                sorted: List.append state.sorted root,
            }
            Set.walk rootOutgoing newState \accum, inboundNode ->
                incoming =
                    Dict.update accum.incoming inboundNode \existing ->
                        when existing is
                            Err Missing -> Err Missing
                            Ok set if Set.isEmpty (Set.remove set root) -> Err Missing
                            Ok set -> Ok (Set.remove set root)
                roots =
                    when Dict.get incoming inboundNode is
                        Err KeyNotFound -> List.append accum.roots inboundNode
                        _ -> accum.roots
                { accum & incoming, roots }
            |> topoSortHelp

middleNumber : List a -> a
middleNumber = \l ->
    index = List.len l // 2
    when List.get l index is
        Ok x -> x
        Err _ -> crash "unreachable"
