library(data.table)

count.plants <- function(start, steps, gens) {
  zero.idx <- 1
  if (substring(start, nchar(start) - 3) != "0000")
    start <- paste0(start, "000")
  if (substring(start, 1, 4) != "0000") {
    start <- paste0("000", start)
    zero.idx <- zero.idx + 3
  }
  last.idx <- nchar(start) - 2

  env <- new.env(parent=emptyenv())
  start.loop <- end.loop <- NULL
  pots <- start
  gen <- 0
  while (gen < gens) {
    idx <- 1:(nchar(pots) - 4)
    chunks <- substring(pots, idx, idx + 4)
    newpots <- c(0, 1)[chunks %in% steps$pattern + 1]
    substring(pots, 3) <- paste(newpots, collapse="")
    if (chunks[4] == "00000" && chunks[1] == chunks[4]) {
      pots <- substring(pots, 6)
      zero.idx <- zero.idx - 5
    }
    if (substring(pots, nchar(pots) - 3) != "0000") {
      pots <- paste0(pots, "000")
    }

    if (is.null(env[[pots]]))
      env[[pots]] <- 1
    else if (is.null(start.loop)) {
      start.loop <- gen
      env[[pots]] <- start.loop
      env[[as.character(gen)]] <- pots
      env[[paste0("zeros.", gen)]] <- zero.idx
    }
    else if (env[[pots]] != start.loop) {
      env[[pots]] <- start.loop
      env[[as.character(gen)]] <- pots
    }
    else if (pots == env[[as.character(start.loop)]]) {
      end.loop <- gen
      break
    }
    gen <- gen + 1
  }
  if (!is.null(end.loop)) {
    iter <- start.loop + (gens - start.loop) %% (end.loop - start.loop)
    pots <- env[[as.character(iter)]]
    zero.idx <- env[[paste0("zeros.", start.loop)]] - ((gens - nchar(start)) %/% 5) * 5 + 1
  }

  nums <- which(unlist(strsplit(pots, "")) == "1") - zero.idx
  return(list(pots=pots, value=sum(nums), zero.idx=zero.idx))
}

input.file <- "data/input-12.txt"
start <- readLines(input.file, n=1)
start <- chartr(".#", "01", gsub("initial state: ", "", start))
steps <- fread(input.file, header=FALSE, skip=2,
               select=c(1, 3), col.names=c("pattern", "change"))
steps <- steps[change != "."]
steps[, pattern := chartr(".#", "01", pattern)]

res <- count.plants(start, steps, 20)$value
cat("Part 1: ", res, "\n")
stopifnot(res == 2840)

res <- count.plants(start, steps, 50000000000)$value
cat("Part 2: ", sprintf("%.0f", res), "\n")
stopifnot(res == 2000000001684)
