library(data.table)

ids <- fread("data/input-02.txt", header=FALSE)
ids <- lapply(ids$V1, function(z) unlist(strsplit(z, "")))

n2 <- n3 <- 0
for (id in ids) {
  tab <- table(id)
  n2 <- n2 + 2 %in% tab
  n3 <- n3 + 3 %in% tab
}
cat("Part 1: ", n2 * n3, "\n")
stopifnot(n2 * n3 == 6200)

res <- NULL
while (length(res) == 0) {
  id1 <- ids[[1]]
  ids <- ids[-1]
  for (id2 in ids) {
    diffs <- id1 != id2
    if (sum(diffs) == 1) {
      res <- paste(id1[!diffs], collapse="")
      break
    }
  }
}
cat("Part 2: ", res, "\n")
stopifnot(res == "xpysnnkqrbuhefmcajodplyzw")
