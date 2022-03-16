def parse(str):
    issue, vals = str.split(" @ ")
    coord, size = vals.split(": ")
    issue = int(issue.replace("#", ""))
    coord = [int(i) for i in coord.split(",")]
    size = [int(i) for i in size.split("x")]
    return (issue, coord, size)


file = open("data/input-03.txt")
vals = [parse(i) for i in file.readlines()]

cover, claim = {}, {}
for (issue, coord, size) in vals:
    lef, top = coord
    wid, hei = size
    for x in range(lef, lef + wid):
        for y in range(top, top + hei):
            val = (x, y)
            if val in cover.keys():
                cover[val] += 1
                claim[val].append(issue)
            else:
                cover[val] = 1
                claim[val] = [issue]

res = sum([i > 1 for i in cover.values()])
print(f"Part 1: {res}")
assert res == 111326

used = [j for i, j in zip(cover.values(), claim.values()) if i > 1]
used = set([i for inner in used for i in inner])  # unlist and unique
[left] = list(set([issue for issue, _, _ in vals]).difference(used))
print(f"Part 2: {left}")
assert left == 1019
