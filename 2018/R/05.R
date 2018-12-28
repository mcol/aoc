library(foreach)
library(doParallel)
registerDoParallel(12)

strReverse <- function(x) {
  sapply(lapply(strsplit(x, NULL), rev), paste, collapse = "")
}
polymer <- scan("../data/input-05.txt", character(), quiet=TRUE)
types <- sort(unique(unlist(strsplit(polymer, ""))))
patterns <- data.frame(matrix(types, nrow=2))
patterns <- sapply(patterns, paste, collapse="")
patterns.both <- c(patterns, strReverse(patterns))

react <- function(polymer, pats) {
  repeat {
    len <- nchar(polymer)
    for (pat in pats)
      polymer <- gsub(pat, "", polymer, perl=TRUE)
    if (nchar(polymer) == len)
      return(polymer)
  }
}
react.slow <- function(polymer, pats) {
  pats.flat <- paste(pats, collapse="|", perl=TRUE)
  repeat {
    len <- nchar(polymer)
    polymer <- gsub(pats.flat, "", polymer)
    if (nchar(polymer) == len)
      return(polymer)
  }
}

tic <- Sys.time()
cat("Part 1: ", nchar(react(polymer, patterns.both)), "\n")

loop.patterns <- function() {
  lengths <- foreach(pat=patterns, .combine=c) %dopar% {
    pat <- paste(c("[", pat, "]"), collapse="")
    polymer1 <- react(gsub(pat, "", polymer), patterns.both)
    nchar(polymer1)
  }
  return(lengths)
}

loop.patterns.slow <- function() {
  lengths <- NULL
  for (pat in patterns) {
    pat <- paste(c("[", pat, "]"), collapse="")
    polymer1 <- react(gsub(pat, "", polymer), patterns.both)
    lengths <- c(lengths, nchar(polymer1))
  }
  return(lengths)
}

cat("Part 2: ", min(loop.patterns()), "\n")
toc <- Sys.time()
print(toc - tic)
