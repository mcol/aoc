remove <- function(x, value) {
  lapply(x, setdiff, value)
}

steps <- fread("data/input-07.txt", sep=" ", select=c(2, 8),
               header=FALSE, col.names=c("before", "after"))
id <- steps[, sort(unique(c(before, after)))]

prev <- vector("list", length(id))
names(prev) <- id
for (idx in 1:nrow(steps)) {
  step <- steps[idx, ]
  prev[[step$after]] <- c(prev[[step$after]], step$before)
}

prev1 <- prev
ordering <- NULL
repeat {
  lengths <- sapply(prev1, length)
  min.idx <- which.min(lengths)
  rm <- names(min.idx)[1]
  ordering <- c(ordering, rm)
  prev1[[rm]] <- NULL
  prev1 <- remove(prev1, rm)
  if (length(prev1) == 0)
    break
}
res <- paste(ordering, collapse="")
cat("Part 1: ", res, "\n")
stopifnot(res == "BCADPVTJFZNRWXHEKSQLUYGMIO")

time <- 0
tasks <- data.frame(id, time=60 + 1:length(id))
worker <- data.table(task=rep(NA_character_, 5), endtime=rep(NA_integer_, 5))
prev2 <- prev
repeat {
  ended.tasks <- which(worker$endtime <= time)
  if (length(ended.tasks) > 0) {
    rem <- worker$task[ended.tasks]
    prev2[[rem]] <- NULL
    prev2 <- remove(prev2, rem)
    if (length(prev2) == 0)
      break
    worker$endtime[ended.tasks] <- NA
    worker$task[ended.tasks] <- NA
  }
  lengths <- sapply(prev2, length)
  min.len <- min(lengths, na.rm=TRUE)
  next.tasks <- setdiff(names(lengths)[lengths == min.len],
                        worker$task)
  for (ntask in next.tasks) {
    idx <- match(NA, worker$endtime)
    if (!is.na(idx) && ntask %in% worker$task)
      break
    worker$endtime[idx] <- time + tasks$time[match(ntask, tasks$id)]
    worker$task[idx] <- ntask
  }
  time <- min(worker$endtime, na.rm=TRUE)
}
cat("Part 2: ", time, "\n")
stopifnot(time == 973)
