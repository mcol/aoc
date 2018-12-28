library(Matrix)
library(png)

points <- read.delim("../data/input-10.txt", header=FALSE)
points$V1 <- gsub("(position|velocity)=<", "", points$V1)
points$V1 <- gsub(">$", "", points$V1)
points$V1 <- gsub(">", ",", points$V1)
points <- t(matrix(as.integer(unlist(strsplit(points$V1, ","))), nrow=4))
points <- data.frame(points[, c(2, 1, 4, 3)])
colnames(points) <- c("row", "col", "Vhor", "Vver")

draw.points <- function(points, size, filename) {
  points <- subset(points, row > 0 & row < size & col > 0 & col < size, 1:2)
  if (nrow(points) < 200)
    return()
  sm <- sparseMatrix(points$row, points$col, x=1, dims=c(size, size))
  writePNG(as.matrix(sm), paste0(filename, ".png"))
}

move.points <- function(points) {
  points$row <- points$row + points$Vhor
  points$col <- points$col + points$Vver
  return(points)  
}

for (i in 1:11000) {
  filename <- sprintf("msg-%05d", i)
  draw.points(points, 400, filename)
  points <- move.points(points)
}

system("Part 2: ", 10888, "\n")
