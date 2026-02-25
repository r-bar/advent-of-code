import std/deques
import std/bitops
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


proc getInputData*(inputFile: string): string =
    try:
        return readFile(inputFile)
    except IoError:
        echo("Error: Could not read the input file.")
        quit(1)


proc loadInput*(inputFilename: string): seq[Machine] =
    let inputData = getInputData(inputFilename)
    let lines = inputData.splitLines()
    for i in 0..lines.high():
        let line = lines[i]
        if line == "":
            continue
        let machine = parseLine(line)
        # echo(fmt"Machine {i}: {line} -> {machine}")
        result.add(machine)


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

func select[T](s: openArray[T], indices: openArray[int]): seq[T] =
    for i in indices:
        result.add(s[i])

iterator combinations*[T](pool: openArray[T]): seq[T] {.closure.} =
    var i = 1'u16
    let last: uint16 = cast[uint16](2 ^ pool.len)
    while i < last:
        yield select(pool, onBits(i))
        i += 1

proc parity(s: openArray[int]): uint16 =
    ## Returns a 16-bit integer where each bit i indicates whether the number of
    ## odd values in s is odd (1) or even (0) at position i.
    var parity = 0'u16
    for (i, n) in enumerate(0, s):
        let even = cast[uint16](n mod 2)
        parity = bitxor(parity, even shl i)
    return parity


proc reduce[T](s: openArray[T], op: proc (a: T, b: T): T): T =
    if s.len == 0:
        return default(T)
    if s.len == 1:
        return s[0]
    var a = s[0]
    for i in 1..s.high:
        let b = s[i]
        a = op(a, b)
    return a


proc map[T, R](a: seq[T], f: proc (x: T): R): seq[R] =
    var output = newSeq[R](a.len)
    for i in 0..a.high:
        output[i] = f(a[i])
    return output


proc mapMut[T, R](a: var openArray[T], f: proc (x: T): R): void =
    for i in 0..a.high:
        a[i] = f(a[i])


proc negSeq(a: openArray[int]): seq[int] =
    var output = newSeq[int](a.len)
    for i in 0..a.high:
        output[i] = -1 * a[i]
    return output

proc binopSeq[T](left: openArray[T], right: openArray[T], op: proc (x: T, y: T): T): seq[T] =
    for (l, r) in zip(left, right):
        result.add(op(l, r))


proc addSeq[T](a: openArray[T], b: openArray[T]): seq[T] =
    return binopSeq(a, b) do (x, y: T) -> T:
        x + y

proc sum[T](a: openArray[T]): T =
    if a.len == 0:
        return default(T)
    var total = a[0]
    for x in a[1..a.high]:
        total += x
    return total

proc extendFirst[T](d: var Deque[T], a: openArray[T]): void =
    for x in reversed(a):
        d.addFirst(x)

proc extendLast[T](d: var Deque[T], a: openArray[T]): void =
    for x in a:
        d.addLast(x)

proc subFromState(state: seq[int], pressed: openArray[uint16]): seq[int] =
    var state: seq[int] = state
    for button in pressed:
        let pressedValues = negSeq(japply(newSeq[int](state.len), button))
        state = addSeq(state, pressedValues)
    return state
    

# proc bitxorp(a: uint16, b: uint16): uint16 =
#     a bitxor b

proc phase1Search*(
    machine: Machine,
    cache: var TableRef[string, seq[seq[uint16]]],
    target: Option[seq[int]] = none(seq[int]),
): seq[seq[uint16]] =
    var reqs: seq[int]
    if target.isSome():
        reqs = target.get()
    else:
        reqs = machine.joltageReqs
    let key = fmt"{reqs}"
    if cache.contains(key):
        return cache[key]
    var validPresses = newSeq[seq[uint16]]()
    for pressed in combinations(machine.schematics):
        let combinedPress = pressed.reduce() do (a, b: uint16) -> uint16:
            a xor b
        # let pressedValues = japply(newSeq[int](machine.size), combinedPress)
        # let remainder = addSeq(reqs, negSeq(pressedValues))
        let remainder = subFromState(reqs, pressed)
        let positive = all(remainder, proc (x: int): bool = x >= 0)
        if positive and parity(remainder) == 0:
            validPresses.add(pressed)
    validPresses.sort() do (a, b: seq[uint16]) -> int:
        cmp(a.len, b.len)
    cache[key] = validPresses
    return validPresses


proc phase2Search(
    machine: Machine,
    cache: TableRef[string, int],
): int =
    todo


proc search*(
    machine: Machine,
): int =
    var pressed = 0
    var cache = newTable[string, seq[seq[uint16]]]()
    var work = initDeque[(int, seq[uint16], seq[int], int)]()
    if sum(machine.joltageReqs) == 0:
        return pressed

    # phase 1

    if parity(machine.joltageReqs) == 0:
        work.addLast((0, @[], machine.joltageReqs, 0))
    else:
        for initPressed in phase1Search(machine, cache):
            var pressedState = subFromState(machine.joltageReqs, initPressed)
            work.addLast((initPressed.len, initPressed, pressedState, 0))

    # phase 2

    while work.len > 0:
        var (prevPresses, prevPressed, state, depth) = work.popFirst()
        if work.len > 5000:
            echo((prevPresses, state, depth))
            quit(1)
        if sum(state) == 0:
            echo(fmt"prevPressed ", prevPressed)
            return prevPresses
        let halfState = map(state) do (x: int) -> int:
            floordiv(x, 2)
        var nextPresses = phase1Search(machine, cache, some(halfState))
        for press in nextPresses:
            let nextState = subFromState(halfState, press)
            if any(nextState, proc (x: int): bool = x < 0):
                echo("Exited due to negative state!!")
                echo("halfstate: ", halfState)
                echo("nextState: ", nextState)
                echo("state: ", state)
                echo("nextPresses: ", nextPresses)
                quit(1)
            work.addFirst((
                prevPresses + 2 * press.len,
                prevPressed & press & press,
                nextState,
                depth + 1,
            ))

    return -1

proc main() =

    let inputFilename = getInputFilename()
    let machines = loadInput(inputFilename)

    var answer = 0
    var machineId = 0
    for machine in machines:
        var cache = newTable[uint16, seq[uint16]]()
        let presses = search(machine)
        echo(fmt"Presses: {presses} <- Machine ({machineId}) {machine}")
        answer += presses
        machineId += 1

    echo answer

if isMainModule:
    main()
