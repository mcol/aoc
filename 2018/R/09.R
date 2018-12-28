num.players <- 468
num.marbles <- 71843
#num.players <- 9;  num.marbles <- 25
#num.players <- 10; num.marbles <- 1618
#num.players <- 13; num.marbles <- 7999
#num.players <- 17; num.marbles <- 1104
#num.players <- 21; num.marbles <- 6111
#num.players <- 30; num.marbles <- 5807

addMarble <- function(idx, prev) {
  marble <- new.env(parent=emptyenv())
  marble$idx <- idx
  marble$succ <- marble
  if (is.null(prev)) {
    marble$prev <- marble
  }
  else {
    marble$prev <- prev
    marble$succ <- prev$succ
    prev$succ$prev <- marble
    prev$succ <- marble
    if (identical(prev$prev, prev))
      prev$prev <- marble
  }
  return(marble)
}

delMarble <- function(marble) {
  marble$prev$succ <- marble$succ
  marble$succ$prev <- marble$prev
  rm(marble)
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
  marble <- 0
  player <- 0
  scores <- rep(0, num.players)
  marbles <- addMarble(marble, NULL)
  current <- marbles

  while (marble < num.marbles) {
#    cat(player, ":", printMarbles(marbles[[1]]), "\n")
    marble <- marble + 1
    player <- player %% num.players + 1
    for (i in 1:2)
      current <- current$succ
    if (marble %% 23 == 0) {
      for (i in 1:7)
        current <- current$prev
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

tic <- Sys.time()
scores <- find.score(num.players, num.marbles)
cat("Part 1: ", max(scores), "\n")
toc <- Sys.time()
print(toc - tic)

tic <- Sys.time()
scores <- find.score(num.players, num.marbles * 100)
cat("Part 2: ", max(scores), "\n")
toc <- Sys.time()
print(toc - tic)
