class Marble:
    def __init__(self, prev, succ):
        self.prev = prev
        self.succ = succ


class Ring:
    def __init__(self, max_size):
        self.curr = 0
        self.vals = [Marble(0, 0)] * (max_size + 1)

    def insert(self, value):
        succ_value = self.vals[self.curr].succ
        self.vals[value] = Marble(self.curr, succ_value)
        self.vals[self.curr].succ = value
        self.vals[succ_value].prev = value

    def remove(self):
        curr, rem = self.curr, self.vals[self.curr]
        prev_value, succ_value = rem.prev, rem.succ
        self.vals[prev_value].succ = succ_value
        self.vals[succ_value].prev = prev_value
        self.curr = prev_value
        return curr

    def step(self, steps):
        for _ in range(steps):
            self.curr = self.vals[self.curr].succ

    def back(self, steps):
        for _ in range(steps):
            self.curr = self.vals[self.curr].prev


def compute_scores(num_players, num_marbles):
    player, scores = 0, [0] * num_players
    marbles = Ring(num_marbles)
    for marble in range(1, num_marbles + 1):
        player = (player + 1) % num_players
        marbles.step(2)
        if marble % 23 == 0:
            marbles.back(8)
            scores[player] += marble + marbles.remove()
        else:
            marbles.insert(marble)
    return scores


file = open("data/input-09.txt")
vals = [v for line in file.readlines() for v in line.split(" ")]
num_players, num_marbles = int(vals[0]), int(vals[6])

res = max(compute_scores(num_players, num_marbles))
print(f"Part 1: {res}")
assert res == 385820

res = max(compute_scores(num_players, num_marbles * 100))
print(f"Part 2: {res}")
assert res == 3156297594
