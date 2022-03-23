input <- scan("data/input-01.txt", quiet=TRUE)

cat("Part 1: ", sum(input), "\n")
stopifnot(sum(input) == 587)

find.frequency <- function(input) {
  cum.freq.change <- cumsum(input)
  len.freq.list <- length(cum.freq.change)
  freq.list <- rep(0, len.freq.list)
  env <- new.env(parent=emptyenv())
  repeat {
    freq.list <- freq.list[len.freq.list] + cum.freq.change
    vals <- as.character(freq.list)
    for (val in vals) {
      if (is.null(env[[val]]))
        env[[val]] <- 1
      else
        return(as.integer(val))
    }
  }
}

res <- find.frequency(input)
cat("Part 2: ", res, "\n")
stopifnot(res == 83130)
