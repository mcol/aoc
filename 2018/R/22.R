draw.map <- function(map) {
  map <- matrix(chartr("012", ".=|", map), nrow(map))
  map <- paste(apply(map, 2, function(z) paste0(paste(z, collapse=""), "\n")),
               collapse="")
  cat(map, "\n")
}
rock.type <- function(target, depth) {
  num.rows <- target[2]
  num.cols <- target[1]
  map <- matrix(0L, num.rows, num.cols)
  map[, 1] <- (48271 * (1:num.rows - 1) + depth) %% 20183
  map[1, ] <- (16807 * (1:num.cols - 1) + depth) %% 20183
  for (row in 2:num.rows) {
    for (col in 2:num.cols) {
      map[row, col] <- (map[row - 1, col] * map[row, col - 1] + depth) %% 20183
    }
  }
  map[target[2], target[1]] <- (0 + depth) %% 20183
  map <- map %% 3
  return(map)
}

input <- readLines("data/input-22.txt")
depth <- as.integer(gsub("depth: ", "", input[1]))
target <- as.integer(unlist(strsplit(gsub("target: ", "", input[2]), ","))) + 1

map <- rock.type(target, depth)
cat("Part 1: ", sum(map), "\n")
