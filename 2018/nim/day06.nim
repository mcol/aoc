import math, std/enumerate, sequtils, sets, strutils, sugar

proc manhattan(ax: int, ay: int, bx: int, by: int): int =
    return abs(ax - bx) + abs(ay - by)

let coords = collect(newSeq):
    for line in lines("data/input-06.txt"):
        line.split(", ").map(parseInt)
var min_row, min_col = high(int)
var max_row, max_col = 0
for row_col in coords:
    (min_row, max_row) = (min(min_row, row_col[0]), max(max_row, row_col[0]))
    (min_col, max_col) = (min(min_col, row_col[1]), max(max_col, row_col[1]))

var infinite: HashSet[int]
var (owner, region_size) = (newSeq[int](len(coords)), 0)
for row in min_row..max_row:
    for col in min_col..max_col:
        let dist = collect(newSeq):
            for i in coords:
                manhattan(i[0], i[1], row, col)
        region_size += int(sum(dist) < 10000)
        let all_min_idx = collect(newSeq):
            for (idx, d) in enumerate(dist):
                if d == min(dist):
                    idx
        if len(all_min_idx) == 1:
            let min_idx = all_min_idx[0]
            owner[min_idx] += 1
            if row in {min_row, max_row} or col in {min_col, max_col}:
                infinite.incl(min_idx)

let finite_owner = collect(newSeq):
    for idx, val in enumerate(owner):
        if idx notin infinite:
            val
let res1 = max(finite_owner)
echo "Part 1: ", res1
assert(res1 == 3907)

let res2 = region_size
echo "Part 2: ", res2
assert(res2 == 42036)
