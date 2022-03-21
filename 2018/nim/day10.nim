import sequtils, strutils, sugar

type Image = object
    row: seq[int]
    col: seq[int]
    hor_vel: seq[int]
    ver_vel: seq[int]

proc initImage(): Image =
     Image(row: newSeq[int](), col: newSeq[int](),
           hor_vel: newSeq[int](), ver_vel: newSeq[int]())

proc add_point(self: var Image, point: seq[int]) =
    self.row.add(point[0])
    self.col.add(point[1])
    self.hor_vel.add(point[2])
    self.ver_vel.add(point[3])

proc move(self: var Image, moves: int = 1) =
    for idx in 0..<len(self.row):
        self.row[idx] += self.hor_vel[idx] * moves
        self.col[idx] += self.ver_vel[idx] * moves

proc sizes(self: Image): (int, int) =
    let wid = max(self.row) - min(self.row)
    let hei = max(self.col) - min(self.col)
    (wid, hei)

proc compute_area(self: Image): int =
    let (wid, hei) = self.sizes()
    wid * hei

proc draw(self: Image) =
    let (wid, hei) = self.sizes()
    let (xmin, ymin) = (min(self.row), min(self.col))
    var render = collect(newSeq):
        for i in 1..hei + 1:
            newSeqWith(wid + 1, ' ')

    for val in zip(self.row, self.col):
        render[val[1] - ymin][val[0] - xmin] = '#'
    for row in 0..hei:
        echo render[row].join()

var image = initImage()
for line in readFile("data/input-10.txt").strip().splitLines():
    let line = line.replace("position=<", "")
        .replace("> velocity=<", ",")
        .replace(">", "")
        .replace(" ", "")
        .split(",")
        .map(parseInt)
    image.add_point(line)

var (min_area, min_iter) = (image.compute_area(), 0)
for iter in 1..11000:
    image.move()
    let area = image.compute_area()
    if area < min_area:
        (min_area, min_iter) = (area, iter)
    else:
        image.move(-1)
        break

echo "Part 1:"
image.draw()

let res2 = min_iter
echo "Part 2: ", res2
assert(res2 == 10888)
