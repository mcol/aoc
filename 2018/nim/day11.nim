import strutils, sugar

proc power_level(x: int, y: int, serialno: int): int =
    let rack_id = x + 10
    var power = (rack_id * y + serialno) * rack_id
    int((power mod 1000) / 100) - 5

proc sum_square(pows: seq[seq[int]], first_idx: int, size: int): int =
    var sum = 0
    for x in first_idx..<first_idx + size:
        for y in 0..<size:
            sum += pows[x][y]
    sum

proc find_square(pows: seq[seq[int]], size: int): (int, int, int) =
    let last_idx = len(pows) - size + 1
    var (max_sum, x, y) = (pows.sum_square(0, size), 0, 0)
    for row in 0..<last_idx:
        var sum = pows.sum_square(row, size)
        for col in 1..<last_idx:
            for idx in row..<row + size:
                sum += pows[idx][col + size - 1] - pows[idx][col - 1]
            if sum > max_sum:
                (max_sum, x, y) = (sum, row + 1, col + 1)
    (max_sum, x, y)


let serialno = readFile("data/input-11.txt").strip().parseInt()
let grid_size = 300
let pows = collect(newSeq):
    for x in 1..grid_size:
        collect(newSeq):
            for y in 1..grid_size:
                power_level(x, y, serialno)


let (_, x, y) = pows.find_square(3)
let res1 = [x, y].join(",")
echo "Part 1: ", res1
assert(res1 == "243,16")

let max_squares = collect(newSeq):
    for size in 1..grid_size:
        (find_square(pows, size), size)
let (res, size) = max(max_squares)
let res2 = [res[1], res[2], size].join(",")
echo "Part 2: ", res2
assert(res2 == "231,227,14")
