iterator counter(max = -1): int =
    var i = 0
    while max == -1 or i < max:
        yield i
        i += 1
    

proc search*(machine: Machine): uint16 =
    let initScore = -1
    let initState = newSeq[int](machine.lightsReq.len)
    let initial: (uint16, uint16, int) = (0, initState, initScore)
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
            # let score = matchingLights(machine.lightsReq, state)
            let pressed = prevPressed + 1
            # echo("prevState: ", showBits(prevState))
            # echo("Button:    ", showBits(button))
            # echo("State:     ", showBits(state))
            # echo("Target:    ", showBits(machine.lightsReq))
            # echo("Pressed:   ", pressed)
            # echo("Score:     ", score)
            if state == machine.lightsReq:
                return pressed
            # if score >= minScore:
            work.addLast((pressed, state, initScore))

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
