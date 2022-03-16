file = open("data/input-01.txt")
nums = [int(i.strip()) for i in file.readlines()]

res = sum(nums)
print(f"Part 1: {res}")
assert res == 587

seen, found, cumsum = {}, False, 0
while not found:
    for i in nums:
        cumsum += i
        if cumsum in seen:
            found = True
            break
        else:
            seen[cumsum] = True

print(f"Part 2: {cumsum}")
assert cumsum == 83130
