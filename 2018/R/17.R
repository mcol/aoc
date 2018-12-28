get.range <- function(val) {
  x <- as.integer(unlist(strsplit(val, ":")))
  if (length(x) == 1)
    x <- c(x, x)
  return(x[1]:x[2])
}
draw.map <- function(map,  pos=NULL) {
  if (!is.null(pos))
    map[pos] <- 9
  mm <- paste0(apply(map, 1, paste, collapse=""), "\n", collapse="")
  mm <- chartr("012359", ".#~|+X", mm)
  cat(mm, "\n")
}

clay <- readLines("../data/example-17b.txt")
clay <- trimws(unlist(strsplit(clay, ",")))
clay <- gsub("..", ":", clay, fixed=TRUE)
clay <- data.frame(x=gsub("x=", "", grep("x", clay, value=TRUE)),
                   y=gsub("y=", "", grep("y", clay, value=TRUE)),
                   stringsAsFactors=FALSE)

spring <- c(0, 500)
x <- lapply(clay$x, get.range)
y <- lapply(clay$y, get.range)

range.x <- range(sapply(x, range))
range.y <- range(sapply(y, range))
spring <- spring - c(0, range.x[1]) + c(1, 2)
x <- lapply(x, function(z) z - range.x[1] + 2)
y <- lapply(y, function(z) z - range.y[1] + 2)

num.x <- diff(range.x) + 3
num.y <- diff(range.y) + 2

map <- matrix(0, num.y, num.x)
for (i in 1:length(x)) {
  map[y[[i]], x[[i]]] <- 1
}
map[spring[1], spring[2]] <- 5

down <- c(1, 0)
left <- c(0, -1)

draw.image <- function(map, pos, txt) {
  png(paste0("map_", txt, ".png"), height=5000, width=3000)
  map[pos] <- 5
  rotate <- function(x) t(apply(x, 2, rev))
  image(rotate(map), col=c("#DDDDDDFF", "#000000FF", "#0000FFFF", "#00FF00FF", "#FF0000FF", "#FF0000FF"))
  dev.off()
}

findflat <- function(map, pos) {
  tovisit <- list(pos)
  visited <- matrix(FALSE, nrow(map), ncol(map))
  while (length(tovisit) > 0) {
    pos <- tovisit[[1]]
    tovisit <- tovisit[-1]
    visited[pos] <- TRUE
    while(pos[1] < num.y && map[pos + down] %in% c(0, 3)) {
      pos <- pos + down
      map[pos] <- 3
    }
    repeat {
      ll <- finddir(map, pos, left)
      map <- ll$map
      rr <- finddir(map, pos, -left)
      map <- rr$map
      coords <- cbind(pos[1], ll$pos[2]:rr$pos[2])
      if (ll$wall && rr$wall) {
        map[coords] <- 2
        pos <- pos - down
      }
      else {
        map[coords] <- 3
        if (pos[1] == num.y)
          break
        if (!ll$wall && !visited[ll$pos])
          tovisit[[length(tovisit) + 1]] <- ll$pos
        if (!rr$wall && !visited[rr$pos] && any(rr$pos != ll$pos))
          tovisit[[length(tovisit) + 1]] <- rr$pos
        tovisit <- unique(tovisit)
        break
      }
    }
  }
  return(list(map=map, pos=pos))
}
finddir <- function(map, pos, dir) {
  while (all(pos < c(num.y, num.x) & pos > c(1, 1)) &&
         map[pos + dir] %in% c(0, 3) &&
         map[pos + down] %in% c(1, 2))
    pos <- pos + dir
#  if (pos[1] < num.y && map[pos + down] %in% c(0, 3))
#    return(list(map=map, pos=pos, wall=FALSE))
  if (pos[1] == num.y || map[pos + down] %in% c(0, 3))
    return(list(map=map, pos=pos, wall=FALSE))
  return(list(map=map, pos=pos, wall=TRUE))
}

findflat.slow <- function(map, pos) {
#  draw.map(map, pos)
  #browser()
  #draw.map(map[max(1, pos[1] - 10):min(num.y, pos[1] + 10),
  #             max(1, pos[2] - 10):min(num.x, pos[2] + 10)])
  while(map[pos + down] %in% c(0, 3)) {
    pos <- pos + down
    map[pos] <- 3
    if (pos[1] == num.y)
      break
  }
  repeat {
#  draw.map(map, pos)
    ll <- finddir.slow(map, pos, left)
    map <- ll$map
    rr <- finddir.slow(map, pos, -left)
    coords <- cbind(pos[1], ll$pos[2]:rr$pos[2])
    map <- rr$map
    if (all(map[coords] %in% c(0, 3)))
      map[coords] <- 3 - (ll$wall && rr$wall)
    pos <- pos - down # could be moved after if
    if (!(ll$wall && rr$wall) || pos[1] < 1)
      break
  }
  return(list(map=map, pos=pos))
}
finddir.slow <- function(map, pos, dir) {
#  draw.map(map, pos)
  while (all(pos < c(num.y, num.x) & pos > c(1, 1)) &&
         map[pos + dir] %in% c(0, 3) &&
         map[pos + down] %in% c(1, 2))
    pos <- pos + dir
  if (pos[1] < num.y && map[pos + down] %in% c(0, 3)) {
    mp <- findflat.slow(map, pos)
    return(list(map=mp$map, pos=mp$pos, wall=FALSE))
  }
  else if (pos[1] == num.y)
    return(list(map=map, pos=pos, wall=FALSE))
  return(list(map=map, pos=pos, wall=TRUE))
}

bfs <- function(map, pos) {
  map <- findflat(map, rbind(pos))$map
#  draw.map(map)
  return(map)
}

dfs <- function(map, pos) {
  mp <- findflat.slow(map, rbind(pos))
  map <- mp$map
#  draw.map(map)
  return(map)
}

mp <- bfs(map, spring)
#mp2 <- dfs(map, spring)
#stopifnot(all.equal(mp, mp2))
#draw.image(mp, spring, "bfs")
cat("Part 1: ", sum(mp %in% c(2, 3)), "\n")
# 31641

cat("Part 2: ", sum(mp == 2), "\n")
