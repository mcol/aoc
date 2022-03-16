def manhattan(a, b):
    return sum([abs(x - y) for x, y in zip(a, b)])


def parse(str):
    v1, v2 = map(int, str.split(", "))
    return v1, v2


def minmax(vec):
    return min(vec), max(vec)


file = open("data/input-06.txt")
coords = [parse(i.strip()) for i in file.readlines()]
min_row, max_row = minmax([r for r, _ in coords])
min_col, max_col = minmax([c for _, c in coords])

owner, infinite, region_size = [0] * len(coords), set(), 0
for row in range(min_row, max_row + 1):
    for col in range(min_col, max_col + 1):
        dist = [manhattan(i, [row, col]) for i in coords]
        region_size += int(sum(dist) < 10000)
        min_idx = [idx for idx, d in enumerate(dist) if d == min(dist)]
        if len(min_idx) == 1:
            owner[min_idx[0]] += 1
            if (
                row == min_row
                or row == max_row
                or col == min_col
                or col == max_col
            ):
                infinite.add(min_idx[0])

res = max([val for idx, val in enumerate(owner) if not idx in infinite])
print(f"Part 1: {res}")
assert res == 3907

res = region_size
print(f"Part 2: {res}")
assert res == 42036
