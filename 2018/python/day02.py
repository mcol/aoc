file = open("data/input-02.txt")
vals = [list(i.strip()) for i in file.readlines()]
letters = set([letter for checksum in vals for letter in checksum])

has2, has3 = [False] * len(vals), [False] * len(vals)
for l in letters:
    for idx, count in [(idx, v.count(l)) for (idx, v) in enumerate(vals)]:
        has2[idx] |= count == 2
        has3[idx] |= count == 3

res = sum(has2) * sum(has3)
print(f"Part 1: {res}")
assert res == 6200

left = vals.copy()
for v1 in vals:
    left.remove(v1)
    for v2 in left:
        output_list = [i != j for i, j in zip(v1, v2)]
        if sum(output_list) == 1:
            break
    else:
        continue
    break
out = [i for i, j in zip(v1, v2) if i == j]
res = "".join(out)
print(f"Part 2: {res}")
assert res == "xpysnnkqrbuhefmcajodplyzw"
