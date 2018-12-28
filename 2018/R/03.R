patterns <- read.delim("../data/input-03.txt", sep=" ", header=FALSE,
                       stringsAsFactors=FALSE)
patterns$V3 <- gsub(":", "", patterns$V3)
lt <- t(matrix(as.integer(unlist(strsplit(patterns$V3, ","))), nrow=2))
colnames(lt) <- c("left", "top")
wh <- t(matrix(as.integer(unlist(strsplit(patterns$V4, "x"))), nrow=2))
colnames(wh) <- c("width", "height")
patterns <- data.frame(lt, wh)
patterns$left <- patterns$left + 1
patterns$top <- patterns$top + 1
max.size <- max(patterns$left + patterns$width,
                patterns$top + patterns$height)

x.y <- matrix(0, max.size, max.size)
for (i in 1:nrow(patterns)) {
  rows <- patterns$top[i]:(patterns$top[i] + patterns$height[i] - 1)
  cols <- patterns$left[i]:(patterns$left[i] + patterns$width[i] - 1)
  x.y[rows, cols] <- x.y[rows, cols] + 1
}
cat("Part 1: ", sum(x.y > 1), "\n")

x.y2 <- matrix(0, max.size, max.size)
for (i in 1:nrow(patterns)) {
  rows <- patterns$top[i]:(patterns$top[i] + patterns$height[i] - 1)
  cols <- patterns$left[i]:(patterns$left[i] + patterns$width[i] - 1)
  x.y2[rows, cols] <- x.y2[rows, cols] + i
}
sets <- data.frame(table(x.y2[x.y == 1]))
patterns <- merge(patterns, sets, by.x="row.names", by.y="Var1")
idx <- which(patterns$width * patterns$height == patterns$Freq)
cat("Part 2: ", patterns$Row.names[idx], "\n")

