library(data.table)

manhattan <- function(a, b) {
  colSums(abs(a - t(b)))
}

coords <- fread("data/input-06.txt", col.names=c("row", "col"))
row.range <- coords[, range(row)]
col.range <- coords[, range(col)]
infinite <- rep(FALSE, nrow(coords))
region.size <- 0
owner <- rep(0, nrow(coords))
for (row in row.range[1]:row.range[2]) {
  for (col in col.range[1]:col.range[2]) {
    dist <- manhattan(c(row, col), coords)
    region.size <- region.size + (sum(dist) < 10000)
    if (sum(dist == min(dist)) == 1) {
      idx <- which.min(dist)
      owner[idx] <- owner[idx] + 1
      if (row %in% row.range || col %in% col.range)
        infinite[idx] <- TRUE
    }
  }
}

res <- max(owner[!infinite])
cat("Part 1: ", res, "\n")
stopifnot(res == 3907)

cat("Part 2: ", region.size, "\n")
stopifnot(region.size == 42036)
tac <- Sys.time()
