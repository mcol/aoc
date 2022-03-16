import sequtils, sets, strutils, sugar

let vals = readFile("data/input-02.txt").strip().splitLines()
let letters = collect(initHashSet()):
    for checksum in vals:
        for letter in checksum:
            {letter}

var num2, num3 = 0
for v in vals:
    var has2, has3 = false
    for l in letters:
        let count = v.count(l)
        has2 = has2 or count == 2
        has3 = has3 or count == 3
    num2 += int(has2)
    num3 += int(has3)
var res1 = num2 * num3
echo "Part 1: ", res1
assert(res1 == 6200)

var (left, res2) = (vals, "")
block loop:
    for v1 in vals:
        left = left.filter(x => x != v1)
        for v2 in left:
            var output_list = 0
            for (i, j) in zip(v1, v2):
                output_list += int(i != j)
            if output_list == 1:
                let seq = collect(newSeq):
                    for (i, j) in zip(v1, v2):
                        if i == j: i
                res2 = seq.join("")
                break loop
echo "Part 2: ", res2
assert(res2 == "xpysnnkqrbuhefmcajodplyzw")
