library(data.table)

dt <- fread("data/input-04.txt", sep="]", header=FALSE,
            col.names=c("date.time", "event"))
dt[, date.time := as.POSIXct(gsub("[", "", date.time, fixed=TRUE))]
setorder(dt, date.time)
dt[, minute := as.integer(format(date.time, "%M"))]
dt[, event := gsub(".*(Guard #[0-9]* begins|asleep|wakes).*", "\\1", event)]
dt[, guard := nafill(as.integer(gsub(".*#([0-9]*).*|.*", "\\1", event)),
                     "locf")]
dt[, event := gsub("Guard #[0-9]* ", "", event)]
dt[, length := as.integer(shift(date.time, type="lead") - date.time)]

guards <- dt[event == "asleep", .(sleep=sum(length)), keyby=guard]
for (id in guards$guard) {
  minutes <- rep(0, 60)
  dt.sub <- dt[guard == id]
  for (i in dt.sub[, which(event == "asleep")]) {
    idx <- dt.sub[, seq(minute[i] + 1, minute[i + 1])]
    minutes[idx] <- minutes[idx] + 1
  }
  guards[guard == id, minute := which.max(minutes) - 1]
  guards[guard == id, times := max(minutes)]
}
res <- guards[which.max(sleep), guard * minute]
cat("Part 1: ", res, "\n")
stopifnot(res == 12169)

res <- guards[which.max(times), guard * minute]
cat("Part 2: ", res, "\n")
stopifnot(res == 16164)
