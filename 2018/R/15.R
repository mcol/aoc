draw.map <- function(map) {
  map <- apply(map, 2, function(z) chartr("0123", ".#EG", z))
  map <- paste(apply(map, 2, function(z) paste0(paste(z, collapse=""), "\n")),
               collapse="")
  cat(map, "\n")
}
toxy <- function(idx) {
  x <- idx %% num.cols
  y <- idx %/% num.cols + 1
  return(cbind(x, y))
}
distance <- function(idx1, idx2) {
  xy1 <- toxy(idx1)
  xy2 <- toxy(idx2)
  unname(abs(xy1[, 1] - xy2[, 1]) + abs(xy1[, 2] - xy2[, 2]))
}
adjs <- function(map, idx) {
  adj <- idx + c(-1, -num.cols, 1, num.cols)
  adj <- adj[map[adj] == 0]
  return(adj)
}
find.target.idx <- function(map, pos, i) {
  target.dist <- Inf
  for (j in 1:nrow(pos)) {
    if (pos$elf[i] == pos$elf[j])
      next
    j.adj <- sort(c(pos$idx[j], adjs(map, pos$idx[j])))
    if (length(j.adj) == 1) # target is surrounded
      return(NULL)
    dist <- distance(j.adj, pos$idx[i])
    min.dist <- min(dist)
    if (min.dist < target.dist ||
        min.dist == 1 && j.adj[which.min(dist)] < target) {
      target <- j.adj[which.min(dist)]
      target.dist <- min.dist
    }
  }
  return(target)
}
find.adj.target <- function(map, pos, i) {
  idx <- pos$idx[i]
  adj <- idx + c(-1, -num.cols, 1, num.cols)
  adj <- c(idx, adj[map[adj] == 0])
  idx.target <- find.target.idx(map, pos, i)
  if (is.null(idx.target))
    return(NULL)
  dist <- distance(adj, idx.target)
  return(adj[order(dist, adj)])
}
move <- function(map, pos) {
  pos <- pos[order(pos$idx), ]
  for (i in 1:nrow(pos)) {
    adj <- find.adj.target(map, pos, i)
    if (is.null(adj))
      next
    map[pos$idx[i]] <- 0
    pos$idx[i] <- adj[1]
    map[pos$idx[i]] <- 3 - pos$elf[i]
  }
  return(list(map=map, pos=pos))
}
attack <- function(idx, pos) {
}
pathfinding <- function(map, current.idx, other.idx) {
  prnt <- rep(NA, length(map))
  prnt[current.idx] <- 0
  frontier <- current.idx
  while (length(frontier) > 0) {
    current.idx <- frontier[1]
    adj.idx <- current.idx + c(-1, -num.cols, 1, num.cols)
    adj.idx <- adj.idx[map[adj.idx] != 1]
    adj.idx <- adj.idx[is.na(prnt[adj.idx])]
    frontier <- c(frontier[-1], adj.idx)
    prnt[adj.idx] <- current.idx
  }  
  return(prnt)
}
distance.pf <- function(parent, idx) {
  target.idx <- which(parent == 0)
  dist <- 0
  while (idx != target.idx) {
    idx <- parent[idx]
    dist <- dist + 1
  }
  return(dist)
}

map <- scan("../data/input-15.txt", character(), quiet=TRUE)
num.rows <- num.cols <- length(map)
map <- chartr(".#EG", "0123", unlist(strsplit(map, "")))

## this is the transpose of what the map looks like, but makes it easier to
## order elements
map <- matrix(as.integer(map), nrow=num.rows)

pos <- data.frame(elf=TRUE, idx=grep(2, map), stringsAsFactors=FALSE)
pos <- rbind(pos, data.frame(elf=FALSE, idx=grep(3, map)))
pos <- pos[order(pos$idx), ]
pos$hp <- 200
pos$ap <- 3

mp <- list(map=map, pos=pos)
for (gen in 1:5) {
  draw.map(mp$map)
  mp <- move(mp$map, mp$pos)
}
