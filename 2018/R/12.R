steps <- readLines("../data/input-12.txt")
start <- gsub("initial state: ", "", steps[1])
start <- chartr(".#", "01", start)
steps <- steps[-c(1:2)]
steps <- data.frame(t(data.frame(strsplit(steps, " => "))), stringsAsFactors=FALSE)
colnames(steps) <- c("pattern", "change")
steps <- subset(steps, change != ".")
steps$pattern <- chartr(".#", "01", steps$pattern)
rownames(steps) <- NULL

count.plants <- function(start, steps, gens) {
  zero.idx <- 1
  if (grepl("10{0,3}$", start))
    start <- paste0(start, "000")
  if (grepl("^0{0,3}1", start)) {
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
    newpots <- c("0", "1")[chunks %in% steps$pattern + 1]
    substring(pots, 3) <- paste(newpots, collapse="")
    if (chunks[1] == "00000" && chunks[1] == chunks[4]) {
      pots <- substring(pots, 6)
      zero.idx <- zero.idx - 5
    }
    if (grepl("10{0,3}$", pots)) {
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
      env[[paste0("zeros.", gen)]] <- zero.idx
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
    zero.idx <- env[[paste0("zeros.", start.loop)]] - ((gens - 109) %/% 5) * 5 + 1
  }

  nums <- which(unlist(strsplit(pots, "")) == "1") - zero.idx
  return(list(pots=pots, value=sum(nums), zero.idx=zero.idx))
}
count.plants.slow <- function(start, steps, gens) {
  start <- chartr("01", ".#", start)
  steps$pattern <-  chartr("01", ".#", steps$pattern)
  zero.idx <- 1
  if (grepl("#\\.{0,3}$", start))
    start <- paste0(start, "...")
  if (grepl("^\\.{0,3}#", start)) {
    start <- paste0("...", start)
    zero.idx <- zero.idx + 3
  }
  last.idx <- nchar(start) - 2

  pots <- start
  gen <- 0
#  cat(sprintf("%2d:", gen), pots, "\n")
  while (gen < gens) {
    idx <- 1:(nchar(pots) - 4)
    chunks <- substring(pots, idx, idx + 4)
    newpots <- c(".", "#")[chunks %in% steps$pattern + 1]
    substring(pots, 3) <- paste(newpots, collapse="")
    if (grepl("#\\.{0,3}$", pots))
      pots <- paste0(pots, "...")
    if (grepl("^\\.{0,3}#", pots)) {
      pots <- paste0("...", pots)
      zero.idx <- zero.idx + 3
    }
    gen <- gen + 1
#    cat(sprintf("%2d:", gen), pots, "\n")
  }
   nums <- which(unlist(strsplit(pots, "")) == "#") - zero.idx
   return(list(pots=pots, value=sum(nums)))
}
count.plants.slowest <- function(start, steps, gens) {
  start <- chartr("01", ".#", start)
  steps$pattern <-  chartr("01", ".#", steps$pattern)
  zero.idx <- 1
  if (grepl("#\\.{0,3}$", start))
    start <- paste0(start, "...")
  if (grepl("^\\.{0,3}#", start)) {
    start <- paste0("...", start)
    zero.idx <- zero.idx + 3
  }
  last.idx <- nchar(start) - 2

  pots <- start
  gen <- 0
#  cat(sprintf("%2d:", gen), pots, "\n")
  while (gen < gens) {
    wip <- pots
    for (idx in 2:(nchar(pots) - 2)) {
      chunk <- substr(pots, idx - 2, idx + 2)
      new <- steps$change[match(chunk, steps$pattern)]
      substring(wip, idx) <- ifelse(is.na(new), ".", new)
    }
    pots <- wip
    if (grepl("#\\.{0,3}$", pots))
      pots <- paste0(pots, "...")
    if (grepl("^\\.{0,3}#", pots)) {
      pots <- paste0("...", pots)
      zero.idx <- zero.idx + 3
    }
    gen <- gen + 1
#    cat(sprintf("%2d:", gen), pots, "\n")
  }
  nums <- 1:nchar(pots) - zero.idx
  return(sum(nums[which(unlist(strsplit(pots, "")) == "#")]))
}
cat("Part 1: ", count.plants(start, steps, 20)$value, "\n")

res <- count.plants(start, steps, 50000000000)
cat("Part 2: ", sprintf("%.0f", res$value), "\n")
