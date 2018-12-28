frequency.change.list <- scan("../data/input-01.txt", quiet=TRUE)
cat("Part 1: ", sum(frequency.change.list), "\n")

find.frequency <- function(freq.change.list) {
  a <- new.env(parent=emptyenv())
  cum.freq.change <- cumsum(freq.change.list)
  len.freq.list <- length(cum.freq.change)
  freq.list <- rep(0, len.freq.list)
  repeat {
    freq.list <- freq.list[len.freq.list] + cum.freq.change
    vals <- as.character(freq.list)
    for (val in vals) {
      if (is.null(a[[val]]))
        a[[val]] <- 1
      else
        return(as.integer(val))
    }
  }
}

find.frequency.slow <- function(freq.change.list) {
  a <- list()
  freq.list <- 0
  len.freq.list <- 1
  repeat {
    freq.list <- freq.list[len.freq.list] + cumsum(freq.change.list)
    len.freq.list <- length(freq.list)
    cat(".")

    for (val in freq.list) {
      aval <- as.character(val)
      if (is.null(a[[aval]]))
        a[[aval]] <- 1
      else
        return(val)
    }
  }
}
cat("Part 2: ", find.frequency(frequency.change.list), "\n")
