tt <- read.delim("../data/input-04.txt", sep="]", header=FALSE,
                 col.names=c("date.time", "event"))
tt$date.time <- as.POSIXct(gsub("[", "", tt$date.time, fixed=TRUE))
tt$event <- trimws(tt$event)
tt$guard <- as.integer(gsub("Guard #([0-9]*).*|falls.*|wakes.*", "\\1",
                            tt$event))
tt$action <- gsub("Guard #[0-9]*", "", tt$event)
tt$minute <- as.integer(format(tt$date.time, "%M"))
tt <- tt[order(tt$date.time), ]
rownames(tt) <- NULL
actions <- sort(unique(tt$action))
actions.short <- c("B", "S", "W")
tt$action <- actions.short[match(tt$action, actions)]

idx <- grep("Guard #", tt$event)
num <- diff(c(idx, nrow(tt) + 1))
guard.idx <- rep(idx, num)
tt$guard <- tt$guard[guard.idx]

sleep <- data.frame(start=grep("S", tt$action),
                    end=grep("W", tt$action))
sleep$guard <- tt$guard[sleep$start]
sleep$length <- as.integer(tt$date.time[sleep$end] - tt$date.time[sleep$start])
stopifnot(all(sleep$end - sleep$start == 1))
sleep$from <- tt$date.time[sleep$start]

guards <- data.frame(id=sort(unique(sleep$guard)), sleep=NA)
for (i in 1:nrow(guards)) {
  guards$sleep[i] <- sum(sleep$length[sleep$guard == guards$id[i]])
}
max.guard.id <- with(guards, id[which.max(sleep)])

tt.sub <- subset(tt, guard == max.guard.id)
minutes <- rep(0, 60)
for (i in which(tt.sub$action == "S")) {
  idx <- (tt.sub$minute[i] + 1):tt.sub$minute[i + 1]
  minutes[idx] <- minutes[idx] + 1
}
max.guard.minute <- which.max(minutes) - 1

cat("Part 1: ", max.guard.id * max.guard.minute, "\n")

guards$minute <- NA
guards$times <- NA
for (id in guards$id) {
  tt.sub <- subset(tt, guard == id)
  minutes <- rep(0, 60)
  for (i in which(tt.sub$action == "S")) {
    idx <- (tt.sub$minute[i] + 1):tt.sub$minute[i + 1]
    minutes[idx] <- minutes[idx] + 1
  }
  idx <- which(guards$id == id)
  guards$minute[idx] <- which.max(minutes) - 1
  guards$times[idx] <- max(minutes)
}
idx <- which.max(guards$times)

cat("Part 2: ", with(guards, id[idx] * minute[idx]), "\n")
