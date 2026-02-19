import dataclasses as dc
import functools as fn
import itertools as it
import logging
import operator as op
import re
import sys
import typing as t
from collections import defaultdict

Vec3: t.TypeAlias = tuple[int, int, int]
Connection: t.TypeAlias = tuple[Vec3, Vec3]


@dc.dataclass(frozen=True)
class Input:
    boxes: list[Vec3]


def parse_input(content: str) -> Input:
    boxes = []
    for line in content.splitlines():
        x, y, z = line.strip().split(",")
        box = int(x), int(y), int(z)
        boxes.append(box)
    return Input(boxes)


def part1(content: str, connection_limit=1000) -> int:
    input = parse_input(content)
    boxes = {*input.boxes}
    network = Network()
    distances = sorted((distance(a, b), pair(a, b)) for a, b in it.combinations(boxes, 2))
    connections_made = 0
    # while len(network.connections()) <= connection_limit:
    while connections_made < connection_limit:
        if not distances:
            raise Exception("Ran out of junction boxes to connect")
        for index, (dist, (a, b)) in enumerate(distances):
            if network.can_connect(a, b):
                network.connect(a, b)
                distances.pop(index)
                connections_made += 1
                logging.debug(f"Connected {a} to {b} at distance {dist}")
                break
            else:
                connections_made += 1
                distances.pop(index)
    circuits = network.circuits()
    circuit_sizes = sorted((len(circuit) for circuit in circuits), reverse=True)
    logging.debug(f"{circuit_sizes=}")
    return fn.reduce(op.mul, circuit_sizes[:3])


def part2(content: str) -> int:
    input = parse_input(content)
    boxes = {*input.boxes}
    network = Network()
    distances = sorted((distance(a, b), pair(a, b)) for a, b in it.combinations(boxes, 2))
    connections_made = 0
    last_connection = None
    while distances:
        for index, (dist, (a, b)) in enumerate(distances):
            if network.can_connect(a, b):
                network.connect(a, b)
                distances.pop(index)
                connections_made += 1
                last_connection = a, b
                logging.debug(f"Connected {a} to {b} at distance {dist}")
                break
            else:
                connections_made += 1
                distances.pop(index)
        if len(network.connected) == len(input.boxes) and len(network.circuits()) == 1:
            break
    logging.debug(f"Finished {connections_made=} {last_connection=} {len(distances)=}")
    assert last_connection is not None
    (ax, _, _), (bx, _, _) = last_connection
    return ax * bx


def test_part1(caplog) -> None:
    with open("example.txt") as f:
        example = f.read()
    with caplog.at_level(logging.DEBUG):
        answer = part1(example, connection_limit=10)
    assert re.match(r"Connected \(162, 817, 812\) to \(425, 690, 689\)", caplog.records[0].message)
    assert re.match(r"Connected \(162, 817, 812\) to \(431, 825, 988\)", caplog.records[1].message)
    assert re.match(r"Connected \(805, 96, 715\) to \(906, 360, 560\)", caplog.records[2].message)
    assert answer == 40


def test_part2(caplog) -> None:
    with open("example.txt") as f:
        example = f.read()
    with caplog.at_level(logging.DEBUG):
        answer = part2(example)
    assert answer == 25272


def distance(a: Vec3, b: Vec3) -> int:
    ax, ay, az = a
    bx, by, bz = b
    return abs(ax - bx) ** 2 + abs(ay - by) ** 2 + abs(az - bz) ** 2


@dc.dataclass
class Network:
    connected: dict[Vec3, set[Vec3]] = dc.field(default_factory=lambda: defaultdict(set))  # type: ignore[call-overload]

    def connections(self) -> set[Connection]:
        return set(pair(a, b) for a, bset in self.connected.items() for b in bset)

    def connect(self, a: Vec3, b: Vec3) -> None:
        self.connected[a].add(b)
        self.connected[b].add(a)

    def can_connect(self, a: Vec3, b: Vec3) -> bool:
        if not (a in self.connected and b in self.connected):
            return True
        # if both junctions are members of the set of connections then we have to
        # check the network of junctions and see if the 2 junctions are members of
        # the same circuit
        return not self.same_circut(a, b)

    def same_circut(self, a: Vec3, b: Vec3) -> bool:
        """Returns True if a and b are reachable on the same circuit of the network"""
        start, end = None, None
        connections = self.connections()
        for left, right in connections:
            if left == a or right == a:
                start, end = a, b
                break
            if left == b or right == b:
                start, end = b, a
                break
        visited = set()
        work = [start]
        while work:
            left = work.pop()
            visited.add(left)
            rights = self.connected[left]
            if end in rights:
                return True
            work.extend(rights - visited)
        return False

    def circuits(self) -> frozenset[frozenset[Vec3]]:
        visited = set()
        circuits = set()
        connected = set(self.connected)
        this_circuit = set()
        work = [next(iter(self.connected))]
        while work:
            start = work.pop()
            visited.add(start)
            this_circuit.add(start)
            work.extend(self.connected[start] - visited)
            if not work:
                circuits.add(frozenset(this_circuit))
                this_circuit = set()
                if connected - visited:
                    work = [next(iter(connected - visited))]
        return frozenset(circuits)

        




def pair(a: Vec3, b: Vec3) -> Connection:
    return tuple(sorted((a, b)))  # type: ignore[return-value]


def main():
    logging.basicConfig(level=logging.DEBUG)
    progname = sys.argv[0]
    answer: int
    match sys.argv[1:]:
        case ["1", filename] | ["part1", filename]:
            with open(filename, "r") as f:
                answer = part1(f.read())
        case ["2", filename] | ["part2", filename]:
            with open(filename, "r") as f:
                answer = part2(f.read())
        case _:
            print(f"{progname} <1|2> <filename>", file=sys.stderr)
            sys.exit(1)
    print(answer)


if __name__ == "__main__":
    main()
