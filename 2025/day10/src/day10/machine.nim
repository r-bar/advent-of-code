import std/bitops
import std/enumerate
import std/strformat
import std/strutils
import std/pegs

template todo* =
  raise newException(Defect, "Not implemented yet")


type Machine* = object
    size*: uint8 = 0
    lightsReq*: uint16 = 0
    schematics*: seq[uint16] = @[]
    joltageReqs*: seq[int] = @[]


func showBits*(x: uint16): string =
    var s = "................"
    for i in 0..15:
        if bitand(x shr i, 1'u16) == 1:
            s[^(i + 1)] = '#'
    return s


func onBits*(x: uint16): seq[int] =
    for i in 0..15:
        if bitand(x shr i, 1'u16) == 1:
            result.add(i)


func countOnBits*(x: uint16): int =
    for i in 0..15:
        if bitand(x shr i, 1'u16) == 1:
            result += 1
    


func `$`* (m: Machine): string =
    var s = "Machine("
    s &= fmt"size: {m.size}"
    s &= ", "
    s &= fmt"lightsReq: ({m.lightsReq}) {showBits(m.lightsReq)}"
    s &= ", "
    var schematics = newSeq[seq[int]](m.schematics.len)
    for (i, schematic) in enumerate(0, m.schematics):
        schematics[i] = onBits(schematic)
    s &= fmt"schematics: {schematics}"
    s &= ", "
    s &= fmt"joltageReqs: {m.joltageReqs}"
    s &= ")"
    return s


func apply*(initial: uint16, schematic: uint16): uint16 =
    bitxor(initial, schematic)


func japply*(initial: openArray[int], schematic: uint16): seq[int] =
    let size = initial.len
    var output = newSeq[int](initial.len)
    for i in 0..initial.high:
        # Check bit at reversed position to match parsing
        if bitand(schematic shr (size - 1 - i), 1'u16) == 1:
            output[i] = initial[i] + 1
        else:
            output[i] = initial[i]
    return output


func matchingLights*(target: uint16, state: uint16): int =
    for i in 0..15:
        let t = bitand(target shr i, 1'u16)
        let s = bitand(state shr i, 1'u16)
        if t == s:
            result += 1


proc copyAnd[T](base: openArray[T], elm: T): seq[T] =
    var new = newSeq[int](base.len + 1)
    for (i, val) in enumerate(base):
        new[i] = val
    new[base.len] = elm
    return new

let lineParser: Peg = peg"""
start <- line
line <- lights ig (schematic ig)+ joltage

lights <- '[' {light}+ ']'
schematic <- '(' ({num} ','?)+ ')'
joltage <- '{' ({num} ','?)+ '}'

num <- \d+
light <- '#' / '.'

ig <- \s*
"""

type Section = enum
    lightsSection, schematicSection, joltageSection

proc parseLine*(input: string): Machine {.raises: [Exception].}=
    var section = lightsSection
    var machine = Machine()
    var schematic = 0'u16
    var exceptions: seq[string] = @[]

    let parser = lineParser.eventParser:
        pkNonTerminal:
            enter:
                case p.nt.name
                of "lights":
                    section = lightsSection
                of "schematic":
                    section = schematicSection
                of "joltage":
                    section = joltageSection
            leave:
                if length < 0:
                    return
                case p.nt.name
                of "schematic":
                    machine.schematics.add(schematic)
                    schematic = 0'u16
        pkCapture:
            leave:
                if length < 0:
                    return
                let cap = s[start..start + length - 1]
                # echo("Start ", start)
                # echo("Length ", length)
                # echo("Cap ", cap)
                try:
                    case section:
                    of lightsSection:
                        machine.size += 1
                        machine.lightsReq = machine.lightsReq shl 1
                        if cap == "#":
                            machine.lightsReq += 1
                    of schematicSection:
                        let bit = cast[int](machine.size) - 1 - cap.parseInt()
                        flipbit(schematic, bit)
                    of joltageSection:
                        machine.joltageReqs.add(cap.parseInt())
                except ValueError:
                    exceptions.add("Invalid integer: " & s)

    if parser(input) < 0:
        raise newException(ValueError, "Input did not match expected format")

    if exceptions.len > 0:
        let details = exceptions.join("\n")
        raise newException(ValueError, "Errors while parsing line:\n" & details)

    assert machine.joltageReqs.len == cast[int](machine.size)

    return machine
