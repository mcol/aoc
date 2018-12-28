remove <- function(x, value) {
  lapply(x, setdiff, value)
}

steps <- read.table("../data/input-07.txt", stringsAsFactors=FALSE)[, c(2, 8)]
colnames(steps) <- c("before", "after")
id <- sort(unique(as.vector(as.matrix(steps))))
tasks <- data.frame(id, time=60 + 1:length(id))

prev <- list()
for (lev in tasks$id)
  prev[[lev]] <- NA
for (idx in 1:nrow(steps)) {
  step <- steps[idx, ]
  prev[[step$after]] <- c(prev[[step$after]], step$before)
}

prev1 <- remove(prev, NA)
ordering <- NULL
repeat {
  lengths <- sapply(prev1, length)
  min.len <- min(lengths, na.rm=TRUE)
  rm <- names(lengths)[lengths == min.len]
  ordering <- c(ordering, rm[1])
  prev1 <- remove(prev1, rm[1])
  prev1[[rm[1]]] <- NULL
  if (length(prev1) == 0)
    break
}
cat("Part 1: ", paste(ordering, collapse=""), "\n")

time <- 0
max.workers <- 5
worker <- data.frame(task=rep(NA, max.workers), endtime=rep(NA, max.workers))
prev2 <- remove(prev, NA)
ordering <- NULL
repeat {
  ended.tasks <- which(worker$endtime <= time)
  if (length(ended.tasks) > 0) {
    rem <- worker$task[ended.tasks]
    ordering <- c(ordering, rem)
    prev2 <- remove(prev2, rem)
    prev2[[rem]] <- NULL
    if (length(prev2) == 0)
      break
    worker$endtime[ended.tasks] <- NA
    worker$task[ended.tasks] <- NA
  }
  lengths <- sapply(prev2, length)
  min.len <- min(lengths, na.rm=TRUE)
  next.tasks <- setdiff(names(lengths)[lengths == min.len],
                        worker$task)

  while (!is.na(match(NA, worker$endtime)) &&
         length(next.tasks) > 0 &&
         !next.tasks[1] %in% worker$task) {
    idx <- match(NA, worker$endtime)
    worker$endtime[idx] <- time + tasks$time[match(next.tasks[1], tasks$id)]
    worker$task[idx] <- next.tasks[1]
    next.tasks <- next.tasks[-1]
  }
  time <- min(worker$endtime, na.rm=TRUE)
}
cat("Part 2: ", time, "\n")
