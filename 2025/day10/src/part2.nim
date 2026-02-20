import std/sequtils
import std/math
import std/tables
import std/options
import std/algorithm
import std/cmdline
import std/enumerate
import std/intsets
import std/os
import std/strformat
import std/strutils
import std/syncio
import day10/machine
import fusion/matching

{.experimental: "caseStmtMacros".}


template todo* =
  raise newException(Defect, "Not implemented yet")


proc usage() =
    let progName = getAppFilename()
    echo(fmt"Usage: {progname} <inputFile>")


proc getInputFilename(): string =
    let params = commandLineParams()
    case params.len:
        of 1:
            return params[0]
        of 0:
            echo("Error: No input file provided")
            usage()
            quit(1)
        else:
            echo("Error: Too many arguments")
            usage()
            quit(1)


proc getInputData(inputFile: string): string =
    try:
        return readFile(inputFile)
    except IoError:
        echo("Error: Could not read the input file.")
        quit(1)


type Jstate = enum
    valid, invalid, equal


func checkJoltageState(m: Machine, state: seq[int]): Jstate =
    # debugEcho("checkJoltageState: ", state, " against target ", m.joltageReqs)
    if m.joltageReqs.len != state.len:
        debugEcho(fmt"Invalid state length: expected {m.joltageReqs.len}, got {state.len}")
        return invalid
    var allEqual = true
    var allValid = true
    for i in 0..m.joltageReqs.high:
        let r = m.joltageReqs[i]
        let s = state[i]
        allEqual = allEqual and s == r
        allValid = allValid and s <= r
    if allEqual:
        # debugEcho("State matches target joltage requirements: ", state)
        return equal
    elif allValid:
        # debugEcho("State is valid but does not match target joltage requirements: ", state)
        return valid
    else:
        # debugEcho("State is invalid: ", state)
        return invalid

proc cmpSchematicLen(a: uint16, b: uint16): int =
    let alen = onBits(a).len
    let blen = onBits(b).len
    return cmp(alen, blen)

proc scoreSchematic(state: openArray[int], target: openArray[int]): int =
    ## Score ranges from 0 to the sum of the target joltage values.
    ## Higher score is better. If any of the state values exceed the target,
    ## the state is invalid and -1 is returned.
    var score = 0
    for (s, t) in zip(state, target):
        let part = t - s
        if part < 0:
            return -1
        score += part
    return score

proc searchCacheKey(state: seq[int], schematic: uint16): string =
    return fmt"{state}+{schematic}"

proc search(
    machine: Machine,
    cache: var Table[string, int],
    startState: Option[seq[int]] = none(seq[int]),
    pressed: int = 0,
): int =
    var state = newSeq[int](machine.size)
    if startState.isSome():
        state = startState.get()

    proc scoreKey(a: uint16, b: uint16): int =
        let aState = japply(state, a)
        let bState = japply(state, b)
        let aScore = scoreSchematic(aState, machine.joltageReqs)
        let bScore = scoreSchematic(bState, machine.joltageReqs)
        return cmp(aScore, bScore)

    case checkJoltageState(machine, state)
    of equal: return pressed
    of invalid: return -1
    of valid: discard

    type SortedSchematic = (int, uint16, seq[int])

    var sortedSchematics = newSeqofCap[SortedSchematic](machine.schematics.len())
    for schematic in machine.schematics:
        let newState = japply(state, schematic)
        let score = scoreSchematic(newState, machine.joltageReqs)
        if score >= 0:
            sortedSchematics.add((score, schematic, newState))
    sortedSchematics.sort(proc (a, b: SortedSchematic): int = cmp(a[0], b[0]))

    for (score, schematic, newState) in sortedSchematics:
        let key = fmt"{state}+{schematic}"
        var presses: int

        # presses = search(machine, cache, some(newState), pressed + 1)
        if cache.hasKey(key):
            presses = cache[key]
        else:
            presses = search(machine, cache, some(newState), pressed + 1)
            cache[key] = presses

        if presses >= 0:
            return presses

    return -1


let inputFilename = getInputFilename()
let inputData = getInputData(inputFilename)

var machines: seq[Machine] = @[]
let lines = inputData.splitLines()
for i in 0..lines.high():
    let line = lines[i]
    if line == "":
        continue
    let machine = parseLine(line)
    # echo(fmt"Machine {i}: {line} -> {machine}")
    machines.add(machine)


type SolvedMsg = object
    threadId: int
    machineId: int
    presses: int


type ThreadControl = enum process, stop


type SearchMsg = object
    control: ThreadControl = process
    machineId: int = 0
    machine: Machine = Machine()


var tx: Channel[SearchMsg]
var rx: Channel[SolvedMsg]

proc searchWorker(threadId: int) {.thread, gcsafe.} =
    var searchCache = initTable[string, int]()
    while true:
        let msg = tx.recv()
        if msg.control == stop:
            break
        searchCache.clear()
        let presses = search(msg.machine, searchCache)
        rx.send(SolvedMsg(threadId: threadId, machineId: msg.machineId, presses: presses))
    echo(fmt"Worker {threadId} exited")

let threadCount = 4
var threads = newSeq[Thread[int]](threadCount)
rx.open()
tx.open()


for thread in 0..<threadCount:
    createThread[int](threads[thread], searchWorker, thread)

var idle = initIntSet()
for i in 0..<threadCount:
    idle.incl(i)

var outstanding = initIntSet()
for (i, machine) in enumerate(machines):
    outstanding.incl(i)
    tx.send(SearchMsg(machineId: i, machine: machine))

var sum = 0
while outstanding.len > 0:
    let msg = rx.recv()
    let recvdMachine = machines[msg.machineId]
    outstanding.excl(msg.machineId)
    echo(fmt"Presses: {msg.presses} Machine {msg.machineId}: {recvdMachine}")
    if msg.presses >= 0:
        sum += msg.presses

for threadId in 0..threads.high:
    tx.send(SearchMsg(control: stop, machineId: threadId))

for thread in threads:
    thread.joinThread()

rx.close()
tx.close()

echo(sum)

