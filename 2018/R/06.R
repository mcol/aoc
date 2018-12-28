manhattan <- function(a, b) {
  sum(abs(a - b))
}

coords <- read.table("../data/input-06.txt", sep=",", col.names=c("col", "row"))
range.row <- range(coords$row) + c(-1, 1)
range.col <- range(coords$col) + c(-1, 1)
seq.row <- range.row[1]:range.row[2]
seq.col <- range.col[1]:range.col[2]
coords <- as.matrix(coords[, 2:1])

dists <- matrix(Inf, length(seq.row), length(seq.col))
owner <- matrix(NA, length(seq.row), length(seq.col))
region <- matrix(FALSE, length(seq.row), length(seq.col))
for (row in seq_along(seq.row)) {
  for (col in seq_along(seq.col)) {
    tot.dist <- 0
    for (idx in 1:nrow(coords)) {
      dist <- manhattan(c(seq.row[row], seq.col[col]), coords[idx, ])
      tot.dist <- tot.dist + dist
      if (dist < dists[row, col]) {
        dists[row, col] <- dist
        owner[row, col] <- idx
      }
      else if (dist == dists[row, col]) {
        owner[row, col] <- NA
      }
    }
    if (tot.dist < 10000)
      region[row, col] <- TRUE
  }
}

infinite.owners <- unique(c(owner[c(1, nrow(owner)), ],
                            owner[, c(1, ncol(owner))]))
owner[owner %in% infinite.owners] <- NA

cat("Part 1: ", max(table(owner)), "\n")

cat("Part 2: ", sum(region), "\n")
