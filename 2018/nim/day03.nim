import sequtils, sets, strutils, sugar, tables

let vals = collect(newSeq):
    for line in lines("data/input-03.txt"):
        var split = line.split(" @ ")
        let issue = split[0].replace("#", "").parseInt()
        split = split[1].split(": ")
        let coor = split[0].split(",").map(parseInt)
        let size = split[1].split("x").map(parseInt)
        (issue, coor, size)

var cover: CountTable[(int, int)]
var claim: Table[(int, int), seq[int]]
for (issue, coor, size) in vals:
    let (lef, top) = (@coor[0], @coor[1])
    let (wid, hei) = (@size[0], @size[1])
    for x in lef..<lef + wid:
        for y in top ..< top + hei:
            let val = (x, y)
            if cover.contains(val):
                cover.inc(val)
                claim[val].add(issue)
            else:
                cover.inc(val)
                claim[val] = @[issue]

var res1 = 0
for i in cover.values():
    res1 += int(i > 1)
echo "Part 1: ", res1
assert(res1 == 111326)

let used = collect(initHashSet()):
    for (k, v) in cover.pairs():
        if v > 1:
            for c in claim[k]: {c}
let issues = collect(initHashSet()):
    for v in vals:
        {v[0]}
let res2 = issues.difference(used).toSeq()[0]
echo "Part 2: ", res2
assert(res2 == 1019)
