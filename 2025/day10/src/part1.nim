import std/deques
import std/sequtils
import std/enumerate
import std/strutils
import std/strformat
import std/os
import std/syncio
import std/cmdline
import day10/machine


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


iterator counter(max = -1): int =
    var i = 0
    while max == -1 or i < max:
        yield i
        i += 1
    

proc search*(machine: Machine): uint16 =
    if machine.target == machine.lights:
        return 0
    let initialScore = -1
    let initial: (uint16, uint16, int) = (0, machine.lights, initialScore)
    var work = initDeque[typeof(initial)]()
    work.addLast(initial)
    let maxDepth = 100'u16
    while work.len > 0:
        let (prevPressed, prevState, minScore) = work.popFirst()
        if prevPressed >= maxDepth:
            raise newException(Exception, fmt"No solution found inside depth {maxDepth}")

        for buttonIdx in 0..machine.schematics.high:
            let button = machine.schematics[buttonIdx]
            let state = apply(prevState, button)
            # let score = matchingLights(machine.target, state)
            let pressed = prevPressed + 1
            # echo("prevState: ", showBits(prevState))
            # echo("Button:    ", showBits(button))
            # echo("State:     ", showBits(state))
            # echo("Target:    ", showBits(machine.target))
            # echo("Pressed:   ", pressed)
            # echo("Score:     ", score)
            if state == machine.target:
                return pressed
            # if score >= minScore:
            work.addLast((pressed, state, initialScore))

    raise newException(Exception, "Search queue exhaused")


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

# echo machines.len

var sum = 0
for machine in machines.items:
    let solution = search(machine)
    # echo("Machine:  ", machine)
    # echo("Solution: ", solution)
    sum += cast[int](solution)

echo sum
