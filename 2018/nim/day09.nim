import math, strutils

type Marble = object
    prev: int
    next: int

proc initMarble(prev: int, next: int): Marble =
    Marble(prev: prev, next: next)

type Ring = object
    curr: int
    vals: seq[Marble]

proc initRing(max_size: int): Ring =
    Ring(curr: 0, vals: newSeq[Marble](max_size + 1))

proc insert(self: var Ring, value: int) =
    let next_value = self.vals[self.curr].next
    self.vals[value] = initMarble(self.curr, next_value)
    self.vals[self.curr].next = value
    self.vals[next_value].prev = value

proc remove(self: var Ring): int =
    let (curr, rem) = (self.curr, self.vals[self.curr])
    let (prev_value, next_value) = (rem.prev, rem.next)
    self.vals[prev_value].next = next_value
    self.vals[next_value].prev = prev_value
    self.curr = prev_value
    return curr

proc step(self: var Ring, steps: int) =
    for _ in 1..steps:
        self.curr = self.vals[self.curr].next

proc back(self: var Ring, steps: int) =
    for _ in 1..steps:
        self.curr = self.vals[self.curr].prev

proc compute_scores(num_players: int, num_marbles: int): seq[int] =
    var (player, scores) = (0, newSeq[int](num_players))
    var marbles = initRing(num_marbles)
    for marble in 1..num_marbles:
        player = (player + 1) mod num_players
        marbles.step(2)
        if marble mod 23 == 0:
            marbles.back(8)
            scores[player] += marble + marbles.remove()
        else:
            marbles.insert(marble)
    return scores

let vals = readFile("data/input-09.txt").strip().splitLines()[0].split(" ")
let (num_players, num_marbles) = (vals[0].parseInt(), vals[6].parseInt())

let res1 = max(compute_scores(num_players, num_marbles))
echo "Part 1: ", res1
assert(res1 == 385820)

let res2 = max(compute_scores(num_players, num_marbles * 100))
echo "Part 2: ", res2
assert(res2 == 3156297594)
