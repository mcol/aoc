addMarble <- function(idx, prev) {
  marble <- new.env(parent=emptyenv())
  marble$idx <- idx
  if (is.null(prev)) {
    marble$succ <- marble
    marble$prev <- marble
  } else {
    succ <- prev$succ
    marble$succ <- succ
    marble$prev <- prev
    succ$prev <- marble
    prev$succ <- marble
    if (identical(prev$prev, prev))
      prev$prev <- marble
  }
  return(marble)
}

delMarble <- function(marble) {
  marble$prev$succ <- marble$succ
  marble$succ$prev <- marble$prev
}

printMarbles <- function(marble) {
  current <- marble
  indices <- NULL
  repeat {
    indices <- c(indices, current$idx)
    current <- current$succ
    if (identical(current, marble))
      break
  }
  return(indices)
}

find.score <- function(num.players, num.marbles) {
  player <- 0
  scores <- rep(0, num.players)
  current <- addMarble(0, NULL)
  for (marble in seq_len(num.marbles)) {
    player <- player %% num.players + 1
    # cat(player, ":", printMarbles(current), "\n")
    current <- current$succ$succ
    if (marble %% 23 == 0) {
      for (i in 0:7)
        current <- current$prev
      scores[player] <- scores[player] + marble + current$idx
      delMarble(current)
    }
    else {
      addMarble(marble, current)
    }
  }
  return(scores)
}

input <- scan("data/input-09.txt", what="", quiet=TRUE)
num.players <- as.integer(input[1])
num.marbles <- as.integer(input[7])

tic <- Sys.time()
scores <- find.score(num.players, num.marbles)
cat("Part 1: ", max(scores), "\n")
stopifnot(max(scores) == 385820)
print(Sys.time() - tic)

scores <- find.score(num.players, num.marbles * 100)
cat("Part 2: ", max(scores), "\n")
stopifnot(max(scores) == 3156297594)
print(Sys.time() - tic)
