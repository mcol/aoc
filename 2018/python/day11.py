import math


def power_level(x, y, serialno):
    rack_id = x + 10
    power = (rack_id * y + serialno) * rack_id
    power = math.floor((power % 1000) / 100) - 5
    return power


def find_square(pows, size):
    last_idx = len(pows) - size + 1
    max_val, x, y = sum([sum(v) for v in pows]), 0, 0
    for row in range(last_idx):
        sub = pows[row : row + size]
        square_sum = sum([sum(v[:size]) for v in sub])
        for col in range(1, last_idx):
            square_sum += sum(
                [
                    sub[row][col + size - 1] - sub[row][col - 1]
                    for row in range(size)
                ]
            )
            if square_sum > max_val:
                max_val, x, y = square_sum, col + 1, row + 1
    return max_val, x, y


file = open("data/input-11.txt")
[serialno] = [int(i.strip()) for i in file.readlines()]
grid_size = 300
pows = [
    [power_level(x + 1, y + 1, serialno) for x in range(grid_size)]
    for y in range(grid_size)
]

_, x, y = find_square(pows, 3)
res = f"{x},{y}"
print(f"Part 1: {res}")
assert res == "243,16"

max_squares = [find_square(pows, size) for size in range(1, 300)]
max_idx, (_, x, y) = max(enumerate(max_squares), key=lambda z: z[1])
res = f"{x},{y},{max_idx + 1}"
print(f"Part 2: {res}")
assert res == "231,227,14"
