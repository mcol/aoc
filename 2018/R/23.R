distance <- function(a, b) {
  rowSums(abs(a - rep(b, each=nrow(a))))
}

bots <- readLines("../data/input-23.txt")
bots <- as.integer(unlist(strsplit(gsub("pos=<|>| r=", "", bots), ",")))
bots <- matrix(bots, ncol=4, byrow=TRUE)
pos <- bots[, 1:3]
rng <- bots[, 4]

bot.idx <- which.max(rng)
dists <- distance(pos, pos[bot.idx, ])
cat("Part 1: ", sum(dists <= rng[bot.idx]), "\n")

in.range <- list()
for (idx in 1:nrow(bots)) {
  dists <- distance(pos, pos[idx, ])
  in.range[[idx]] <- which(dists <= rng[idx])
}
in.range <- table(unlist(in.range))
max.idx <- as.integer(names(in.range))[in.range == max(in.range)]
cur.dist <- max(in.range)

coord.ranges <- apply(pos[max.idx, , drop=FALSE], 2, range)
a1 <- seq(coord.ranges[1, 1], coord.ranges[2, 1])
a2 <- seq(coord.ranges[1, 2], coord.ranges[2, 2])
a3 <- seq(coord.ranges[1, 3], coord.ranges[2, 3])
a12 <- cbind(rep(a1, length(a2)), rep(a2, each=length(a1)))
new.pos <- cbind(a12[rep(1:nrow(a12), length(a3)), ], rep(a3, each=nrow(a12)))

pos <- rbind(pos, new.pos)
rng <- c(rng, rep(1, nrow(new.pos)))

in.range <- list()
for (idx in 1:nrow(bots)) {
  dists <- distance(pos, pos[idx, ])
  in.range[[idx]] <- which(dists <= rng[idx])
}
in.range <- table(unlist(in.range))
max.idx <- as.integer(names(in.range))[in.range == max(in.range)]

cat("Part 2: ", sum(abs(pos[max.idx, ])), "\n")

# 47095107 too low
