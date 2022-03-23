library(foreach)
library(doParallel)
registerDoParallel(12)

strReverse <- function(x) {
  sapply(lapply(strsplit(x, NULL), rev), paste, collapse = "")
}
polymer <- scan("data/input-05.txt", character(), quiet=TRUE)
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
res <- nchar(react(polymer, patterns.both))
cat("Part 1: ", res, "\n")
stopifnot(res == 9296)

lengths <- foreach(pat=patterns, .combine=c) %dopar% {
  pat <- paste0("[", pat, "]")
  nchar(react(gsub(pat, "", polymer), patterns.both))
}
res <- min(lengths)
cat("Part 2: ", res, "\n")
stopifnot(res == 5534)
