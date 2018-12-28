license <- scan("../data/input-08.txt", quiet=TRUE)

readnode <- function(vec) {
  num.children <- vec[1]
  len.metadata <- vec[2]
  metadata <- NULL
  vec <- vec[-c(1:2)]
  while (num.children > 0) {
    child <- readnode(vec)
    vec <- child$vec
    num.children <- num.children - 1
    metadata <- c(metadata, child$metadata)
  }
  if (num.children == 0) {
    metadata <- c(metadata, vec[1:len.metadata])
    vec <- vec[-c(1:len.metadata)]
  }
  return(list(metadata=metadata, vec=vec))
}

tree <- readnode(license)
cat("Part 1: ", sum(tree$metadata), "\n")

readnode.value <- function(vec) {
  num.children <- vec[1]
  len.metadata <- vec[2]
  children <- list()
  metadata <- NULL
  vec <- vec[-c(1:2)]
  if (num.children > 0) {
    while (num.children > 0) {
      child <- readnode.value(vec)
      children[[length(children) + 1]] <- child
      vec <- child$vec
      num.children <- num.children - 1
    }
    if (num.children == 0) {
      metadata <- vec[1:len.metadata]
      vec <- vec[-c(1:len.metadata)]
      value <- 0
      for (idx in metadata) {
        if (idx <= length(children))
          value <- value + sum(children[[idx]]$value)
      }
    }
  }
  else {
    metadata <- vec[1:len.metadata]
    vec <- vec[-c(1:len.metadata)]
    value <- sum(metadata)
  }
  return(list(metadata=metadata, value=value, children=children, vec=vec))
}

tree <- readnode.value(license)
cat("Part 2: ", tree$value, "\n")
