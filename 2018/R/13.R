tonum <- function(chars) {
  old <- c(" ", "^", "v", "|", ">", "<", "-", "+", "/", "\\")
  new <- c( 0,   1,   1,   1,   2,   2,   2,   3  , 4,   5)
  num <- new[match(chars, old)]
  return(num)
}
tochr <- function(nums) {
  old <- c( 0 ,  1,   2,   3,   4,   5)
  new <- c(" ", "|", "-", "+", "/", "\\")
  chr <- new[match(nums, old)]
  return(chr)
}
todir <- function(arr) {
  old <- c("^", ">", "v", "<")
  dir <- dirs[match(arr, old)]
  return(dir)
}
toarr <- function(dir) {
  arr <- c("^", ">", "v", "<")[dir]
  return(arr)
}
toxy <- function(idx) {
  x <- idx %% num.cols - 1
  y <- idx %/% num.cols
  paste(c(x, y), collapse=",")
}
draw.track <- function(track, carts) {
  track <- tochr(track)
  track[carts$idx] <- toarr(carts$dir)
  track <- matrix(track, nrow=num.rows, byrow=TRUE)
  cat(paste(apply(track, 1, paste, collapse=""), collapse="\n"))
  cat("\n\n")
}
turn.cart <- function(track, cart) {
  pos <- track[cart$idx]
  if (pos == 3) {
    mov <- cart$mov
    if (mov == 1)
      cart$dir <- c(4,1,2,3)[cart$dir]
    else if (mov == 3)
      cart$dir <- c(2,3,4,1)[cart$dir]
    cart$mov <- c(2,3,1)[mov]
  }
  else if (pos == 4) {
    cart$dir <- c(2,1,4,3)[cart$dir]
  }
  else if (pos == 5) {
    cart$dir <- c(4,3,2,1)[cart$dir]
  }
  return(cart)
}
move.cart <- function(track, cart) {
  moves <- c(N=-num.cols, E=1, S=num.cols, W=-1)
  mov <- moves[cart$dir]
  cart$idx <- cart$idx + mov
  cart <- turn.cart(track, cart)
  return(cart)
}

input.file <- "data/input-13.txt"
tracks <- readLines(input.file)
num.rows <- length(tracks)
num.cols <- nchar(tracks[1])
track <- unlist(strsplit(tracks, ""))

dirs <-c(N=1, E=2, S=3, W=4)
movs <- c(L=1, S=2, R=3)
carts <- data.frame(dir=todir(grep(">|v|<|\\^", track, value=TRUE)),
                    idx=grep(">|v|<|\\^", track),
                    mov=1, stringsAsFactors=FALSE)
track <- tonum(track)

loop.carts <- function(track, carts, num.ticks, remove.crashes) {
  crash.idx <- NA
  for (tick in 1:num.ticks) {
#    draw.track(track, carts)
    carts <- carts[order(carts$idx), ]
    for (i in 1:nrow(carts)) {
      if (is.na(carts$idx[i]))
        next
      carts[i, ] <- move.cart(track, carts[i, ])
      if (any(duplicated(carts$idx))) {
        crash.idx <- which(duplicated(carts$idx))
        if (remove.crashes) {
          carts[carts$idx %in% carts$idx[crash.idx], ] <- NA
        }
        else {
          return(list(carts=carts, crash.idx=crash.idx, tick=tick))
        }
      }
    }
    carts <- carts[!is.na(carts$idx), ]
    if (nrow(carts) == 1)
      break
  }
  return(list(carts=carts, crash.idx=crash.idx, tick=tick))
}
tic <- Sys.time()
res <- loop.carts(track, carts, 500, remove.crashes=FALSE)
cat("Part 1: ", toxy(res$carts$idx[res$crash.idx]), "\n")
toc <- Sys.time()
print(toc - tic)

tic <- Sys.time()
res <- loop.carts(track, carts, 15000, remove.crashes=TRUE)
cat("Part 2: ", toxy(res$carts$idx), "\n")
toc <- Sys.time()
print(toc - tic)
