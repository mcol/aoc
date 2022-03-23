library(data.table)

patterns <- fread("data/input-03.txt", sep=" ", header=FALSE, select=3:4)
patterns[, c("left", "top") := lapply(tstrsplit(sub(":", "", V3), ","),
                                      function(z) as.integer(z) + 1)]
patterns[, c("width", "height") := lapply(tstrsplit(V4, "x"), as.integer)]

x.y <- matrix(0, patterns[, max(left + width)], patterns[, max(top + height)])
for (idx in 1:nrow(patterns)) {
  rows <- patterns[idx, seq(top, top + height - 1)]
  cols <- patterns[idx, seq(left, left + width - 1)]
  x.y[rows, cols] <- x.y[rows, cols] + 1
}
res <- sum(x.y > 1)
cat("Part 1: ", res, "\n")
stopifnot(res == 111326)

for (idx in 1:nrow(patterns)) {
  rows <- patterns[idx, seq(top, top + height - 1)]
  cols <- patterns[idx, seq(left, left + width - 1)]
  if (all(x.y[rows, cols] == 1)) {
    res <- idx
    break
  }
}
cat("Part 2: ", res, "\n")
stopifnot(res == 1019)
