ids <- as.character(read.table("../data/input-02.txt")$V1)

n2 <- n3 <- NULL
for (id in ids) {
  tables <- table(unlist(strsplit(id, "")))
  n2 <- c(n2, 2 %in% tables)
  n3 <- c(n3, 3 %in% tables)
}
cat("Part 1: ", sum(n2) * sum(n3), "\n")

checked <- NULL
for (id1 in ids) {
  checked <- c(checked, id1)
  id1 <- unlist(strsplit(id1, ""))
  for (id2 in setdiff(ids, checked)) {
    id2 <- unlist(strsplit(id2, ""))
    diffs <- id1 != id2
    if (sum(diffs) == 1) {
      cat("Part 2: ", paste(id1[!diffs], collapse=""), "\n")
      break
    }
  }
}
