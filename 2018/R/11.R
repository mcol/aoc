library(foreach)
library(doParallel)
registerDoParallel(cores=10)

serialno <- scan("data/input-11.txt", quiet=TRUE)

power.level <- function(x, y, serialno) {
  rack.id <- x + 10
  power <- (rack.id * y + serialno) * rack.id
  power <- floor((power %% 1000) / 100) - 5
  return(power)
}

find.square <- function(pows, grid.size, size) {
  if (size == grid.size)
    return(list(x=1, y=1, power=sum(pows)))
  size <- size - 1
  vec <- 0:size
  max.val <- sum(pows[1 + vec, 1 + vec])
  x <- y <- 1
  for (row in 1:(grid.size - size)) {
    val <- sum(pows[row + vec, 1 + vec])
    for (col in 2:(grid.size - size)) {
      # slide right
      val <- val + sum(pows[row + vec, col + size] - pows[row + vec, col - 1])
      if (val > max.val) {
        max.val <- val
        x <- col
        y <- row
      }
    }
  }
  return(list(x=x, y=y, power=max.val))
}

grid.size <- 300
x <- rep(1:grid.size, each=grid.size)
y <- rep(1:grid.size, times=grid.size)

pows <- matrix(power.level(x, y, serialno), grid.size, grid.size)
max.square <- find.square(pows, grid.size, 3)
res <- with(max.square, paste(x, y, sep=","))
cat("Part 1: ", res, "\n")
stopifnot(res == "243,16")

max.squares <- foreach (size=1:grid.size) %dopar% {
  find.square(pows, grid.size, size)
}
idx <- which.max(lapply(max.squares, function(z) z$power))
res <- with(max.squares[[idx]], paste(x, y, idx, sep=","))
cat("Part 2: ", res, "\n")
stopifnot(res == "231,227,14")
