readnode <- function(vec) {
  num.children <- vec[1]
  len.metadata <- vec[2]
  vec <- vec[-c(1:2)]
  children <- list()
  sum.meta <- 0
  for (it in seq_len(num.children)) {
    child <- readnode(vec)
    children[[length(children) + 1]] <- child
    vec <- child$vec
    sum.meta <- sum.meta + child$sum.meta
  }
  metadata <- vec[1:len.metadata]
  vec <- vec[-c(1:len.metadata)]
  if (num.children > 0) {
    value <- 0
    for (idx in metadata) {
      if (idx <= num.children)
        value <- value + children[[idx]]$value
    }
  } else {
    value <- sum(metadata)
  }
  return(list(vec=vec, value=value, sum.meta=sum.meta + sum(metadata)))
}

license <- scan("data/input-08.txt", quiet=TRUE)
tree <- readnode(license)

cat("Part 1: ", tree$sum.meta, "\n")
stopifnot(tree$sum.meta == 41849)

cat("Part 2: ", tree$value, "\n")
stopifnot(tree$value == 32487)
