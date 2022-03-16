import math, sets, strutils, sugar

let nums = collect(newSeq):
    for line in lines("data/input-01.txt"):
        parseInt(line)

var res = sum(nums)
echo "Part 1: ", res
assert(res == 587)

var seen: HashSet[int]
var (found, cumsum) = (false, 0)
while not found:
    for i in nums:
        cumsum += i
        if seen.containsOrIncl(cumsum):
            found = true
            break
echo "Part 2: ", cumsum
assert(cumsum == 83130)
