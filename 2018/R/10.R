library(data.table)

move.points <- function(points, moves=1) {
  points[, row := row + hor_vel * moves]
  points[, col := col + ver_vel * moves]
}
compute.area <- function(points) {
  points[, as.numeric(diff(range(row))) * diff(range(col))]
}
draw.points <- function(points) {
  points[, row := row - min(row) + 1]
  points[, col := col - min(col) + 1]
  wid = points[, max(row) + 1]
  hei = points[, max(col) + 1]
  render = matrix(" ", wid, hei)
  for (i in 1:nrow(points))
    render[points[i, row], points[i, col]] = "#"
  for (row in 1:nrow(render))
    cat(paste(render[row, ], collapse=""), "\n")
}

points <- fread("data/input-10.txt", header=FALSE, sep=NULL)
points[, V1 := gsub("(position|velocity)=<|>$", "", V1)]
points[, V1 := gsub(">", ",", V1)]
points[, c("col", "row", "ver_vel", "hor_vel") := lapply(tstrsplit(V1, ","), as.integer)]
points[, V1 := NULL]

min.area = Inf
min.iter = 0
for (iter in 1:11000) {
  move.points(points)
  area = compute.area(points)
  if (area < min.area) {
    min.area = area
    min.iter = iter
  } else {
    move.points(points, -1)
    break
  }
}

cat("Part 1:\n")
draw.points(points)

res = min.iter
cat("Part 2: ", res, "\n")
stopifnot(res == 10888)
