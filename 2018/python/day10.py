class Image:
    def __init__(self, points):
        self.row = []
        self.col = []
        self.hor_vel = []
        self.ver_vel = []
        for row, col, hor_vel, ver_vel in points:
            self.row.append(row)
            self.col.append(col)
            self.hor_vel.append(hor_vel)
            self.ver_vel.append(ver_vel)

    def move(self, moves=1):
        for idx in range(len(self.row)):
            self.row[idx] += self.hor_vel[idx] * moves
            self.col[idx] += self.ver_vel[idx] * moves

    def __sizes(self):
        xmin, ymin = min(self.row), min(self.col)
        wid, hei = max(self.row) - xmin, max(self.col) - ymin
        return (wid, hei, xmin, ymin)

    def area(self):
        wid, hei, _, _ = self.__sizes()
        return wid * hei

    def draw(self):
        wid, hei, xmin, ymin = self.__sizes()
        render = [[" " for _ in range(wid + 1)] for _ in range(hei + 1)]
        for i, j in zip(self.row, self.col):
            render[j - ymin][i - xmin] = "#"
        for row in render:
            print("".join(row))


def parse(str):
    str = (
        str.strip()
        .replace("position=<", "")
        .replace("> velocity=<", ",")
        .replace(">", "")
    )
    row, col, hor_vel, ver_vel = map(int, str.split(","))
    return (row, col, hor_vel, ver_vel)


file = open("data/input-10.txt")
image = Image([parse(line) for line in file.readlines()])

min_area = image.area()
for it in range(1, 11000):
    image.move()
    area = image.area()
    if area < min_area:
        min_area, min_iter = area, it
    else:
        image.move(-1)
        break

print(f"Part 1:")
image.draw()

res = min_iter
print(f"Part 2: {res}")
assert res == 10888
