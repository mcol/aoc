draw.map <- function(map) {
  map <- matrix(chartr("012", ".|#", map[map != -1]), num.y - 2, num.x - 2)
  map <- paste(apply(map, 2, function(z) paste0(paste(z, collapse=""), "\n")),
               collapse="")
  cat(map, "\n")
}

map <- readLines("../data/input-18.txt")
num.x <- nchar(map[1]) + 2
num.y <- length(map) + 2

map <- sapply(strsplit(map, ""), function(z) c("~", z, "~"))
map <- as.integer(chartr("~.|#", "0123", map)) - 1
map <- c(rep(-1, num.y), map, rep(-1, num.y))
map <- matrix(map, ncol=num.x)
map.ok <- map

adjs <- c(-num.y + c(-1, 0, 1), -1, 1, num.y + c(-1, 0, 1))
grow <- function(idx) {
  adj.idx <- idx + adjs
  current <- map[idx]
  if (current == -1)
    return(-1)
  if (current == 2) {
    if (any(map[adj.idx] == 1) && any(map[adj.idx] == 2))
      return(2)
    else
      return(0)
  }
  if (sum(map[adj.idx] == current + 1) > 2)
    return(current + 1)
  else
    return(current)
}

inner.idx <- num.x + 1 + 1:(length(map) - 2 * num.x - 2)

map <- map.ok
for (iter in 1:10) {
  map[inner.idx] <- sapply(inner.idx, grow)
}
cat("Part 1: ", sum(map == 1) * sum(map == 2), "\n")

lts <- c(letters, toupper(letters))
env <- new.env(parent=emptyenv())
start.loop <- NULL
map <- map.ok
for (iter in 1:10000) {
  map[inner.idx] <- sapply(inner.idx, grow)
  hash <- paste(paste(lts[rowSums(map)[-c(1, num.y)] + 3], collapse=""),
                paste(lts[colSums(map)[-c(1, num.x)] + 3], collapse=""), sep="_")
  if (is.null(env[[hash]])) {
    env[[hash]] <- 1
  }
  else if (is.null(start.loop)) {
    start.loop <- iter
    env[[hash]] <- start.loop
  }
  else if (env[[hash]] != start.loop)
    env[[hash]] <- start.loop
  else {
    end.loop <- iter
    break
  }
}
for (it in start.loop:end.loop) {
  map[inner.idx] <- sapply(inner.idx, grow)
  env[[as.character(it + 1)]] <- map
}

map.for.iter <- function(env, iter) {
  iter <- start.loop + (iter - start.loop) %% (end.loop - start.loop)
  env[[as.character(iter)]]
}

map <- map.for.iter(env, 1000000000)
cat("Part 2: ", sum(map == 1) * sum(map == 2), "\n")
