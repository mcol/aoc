from string import ascii_lowercase


def react(polymer, skip):
    reacted, last = [], " "
    for char in polymer:
        if char.lower() == skip:
            continue
        if char != last and char.lower() == last.lower():
            del reacted[-1]
            try:
                last = reacted[-1]
            except:
                last = " "
            continue
        reacted.append(char)
        last = char
    return reacted


file = open("data/input-05.txt")
[polymer] = [i.strip() for i in file.readlines() if len(i) > 1]

res = len(react(polymer, " "))
print(f"Part 1: {res}")
assert res == 9296

res = min([len(react(polymer, c)) for c in ascii_lowercase])
print(f"Part 2: {res}")
assert res == 5534
