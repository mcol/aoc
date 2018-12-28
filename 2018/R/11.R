library(foreach)
library(doParallel)
registerDoParallel(cores=10)

serialno <- scan("../data/input-11.txt", quiet=TRUE)

power.level <- function(x, y, serialno) {
  rack.id <- x + 10
  power <- (rack.id * y + serialno) * rack.id
  power <- floor((power %% 1000) / 100) - 5
  return(power)
}
stopifnot(power.level(3, 5, 8) == 4)
stopifnot(power.level(122, 79, 57) == -5)
stopifnot(power.level(217, 196, 39) == 0)
stopifnot(power.level(101, 153, 71) == 4)

find.square <- function(pows, grid.size, size) {
  mpows <- matrix(pows, grid.size, grid.size)
  size <- size - 1
  vec <- 0:size
  max.val <- val <- sum(mpows[1 + vec, 1 + vec])
  x <- y <- row <- 1
  repeat {
    if (row >= grid.size - size)
      break
    for (col in 2:(grid.size - size)) {
      # slide right
      val <- val + sum(mpows[row + vec, col + size] - mpows[row + vec, col - 1])
      if (val > max.val) {
        max.val <- val
        x <- col
        y <- row
      }
    }
    row <- row + 1
    val <- sum(mpows[row + vec, 1 + vec])
    if (val > max.val) {
      max.val <- val
      x <- 1
      y <- row
    }
  }
  return(list(x=x, y=y, power=max.val))
}
find.square <- function(pows, grid.size, size) {
  if (size == grid.size)
    return(list(x=1, y=1, power=sum(pows)))
  mpows <- matrix(pows, grid.size, grid.size)
  size <- size - 1
  vec <- 0:size
  max.val <- sum(mpows[1 + vec, 1 + vec])
  x <- y <- 1
  for (row in 1:(grid.size - size)) {
    val <- sum(mpows[row + vec, 1 + vec])
    for (col in 2:(grid.size - size)) {
      # slide right
      val <- val + sum(mpows[row + vec, col + size] - mpows[row + vec, col - 1])
      if (val > max.val) {
        max.val <- val
        x <- col
        y <- row
      }
    }
  }
  return(list(x=x, y=y, power=max.val))
}

find.square.slow <- function(pows, grid.size, size) {
  max.val <- -Inf
  x <- y <- NA
  size <- size - 1
  mpows <- matrix(pows, grid.size, grid.size)
  for (row in 1:(nrow(mpows) - size)) {
    for (col in 1:(ncol(mpows) - size)) {
      val <- sum(mpows[row + 0:size, col + 0:size])
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

pows <- power.level(x, y, serialno)
max.square <- find.square(pows, grid.size, 3)
cat("Part 1: ", paste(max.square$x, max.square$y, sep=","), "\n")

max.squares <- foreach (size=1:grid.size) %dopar% {
  find.square(pows, grid.size, size)
}
idx <- which.max(lapply(max.squares, function(z) z$power))
cat("Part 2: ", with(max.squares[[idx]], paste(x, y, idx, sep=",")), "\n")
