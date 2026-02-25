# This is just an example to get you started. You may wish to put all of your
# tests into a single file, or separate them into multiple `test1`, `test2`
# etc. files (better names are recommended, just make sure the name starts with
# the letter 't').
#
# To run these tests, simply execute `nimble test`.

import unittest
import std/tables

import part2

test "correct welcome":
  check 1 + 1 == 2


test "combinations":
    var combos = newSeq[seq[int]]()
    for combo in combinations([1, 2, 3]):
        combos.add(combo)
    check combos == @[
        @[1],
        @[2],
        @[1, 2],
        @[3],
        @[1, 3],
        @[2, 3],
        @[1, 2, 3],
    ]

type Phase1Results = seq[seq[uint16]]

test "phase1Search":
    let machines = loadInput("example.txt")
    check machines.len == 3
    var results = newSeq[Phase1Results]()
    for machine in machines:
        var cache = newTable[string, Phase1Results]()
        results.add(phase1Search(machine, cache))
    check results.len == machines.len
    check results[0] == @[
        @[1'u16, 12'u16],
        @[5'u16, 2'u16, 10'u16],
        @[2'u16, 3'u16, 12'u16],
        @[1'u16, 5'u16, 3'u16, 10'u16],
    ]
    check results[1] == @[
        @[6'u16, 28'u16],
        @[23'u16, 17'u16, 28'u16],
    ]
    check results[2] == @[
        @[38'u16, 59'u16],
        @[62'u16, 59'u16, 24'u16],
    ]
