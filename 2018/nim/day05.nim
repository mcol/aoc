import strutils, sugar

proc react(polymer: string, skip: char): string =
    var reacted: seq[char]
    var last = ' '
    for ch in polymer:
        if ch.toLowerAscii() == skip:
            continue
        if ch != last and ch.toLowerAscii() == last.toLowerAscii():
            reacted.delete(len(reacted))
            last = if len(reacted) > 0: reacted[len(reacted) - 1] else: ' '
            continue
        reacted.add(ch)
        last = ch
    return reacted.join("")

let polymer = readFile("data/input-05.txt").strip().splitLines()[0]

let res1 = len(react(polymer, ' '))
echo "Part 1: ", res1
assert(res1 == 9296)

let all = collect(newSeq):
    for c in 'a'..'z':
        len(react(polymer, c))
let res2 = min(all)
echo "Part 2: ", res2
assert(res2 == 5534)
