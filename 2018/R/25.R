points <- readLines("../data/input-25.txt")
points <- matrix(as.integer(unlist(strsplit(points, ","))), ncol=4, byrow=TRUE)

distance <- function(a, b) {
  rowSums(abs(a - rep(b, each=nrow(a))))
}

find.constellations <- function(points) {
  consts <- list(rbind(points[1, ]))
  points <- points[-1, ]
  while (nrow(points) > 0) {
    found <- FALSE
    for (i in 1:length(consts)) {
      for (j in 1:nrow(consts[[i]])) {
        dist.idx <- which(distance(points, consts[[i]][j, ]) < 4)
        if (length(dist.idx) > 0) {
          consts[[i]] <- rbind(consts[[i]], points[dist.idx, ])
          points <- points[-dist.idx, ]
          found <- TRUE
          break
        }
      }
      if (found)
        break
    }
    if (!found) {
      consts[[length(consts) + 1]] <- rbind(points[1, ])
      points <- points[-1, , drop=FALSE]
    }
  }
  return(consts)
}

consts <- find.constellations(points)
cat("Part 1: ", length(consts), "\n")
