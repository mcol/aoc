gens <- scan("data/input-14.txt", character(), quiet=TRUE)
start.recipes <- c(3L, 7L)

make.recipes <- function(start.recipes, gens) {
  num.recipes <- num.elves <- length(start.recipes)
  elves <- 1:num.elves
  gens <- as.integer(gens)
  recipes <- rep(1L, num.recipes + gens + 10)
  recipes[1:num.recipes] <- start.recipes
  while (num.recipes < gens + 10) {
    val <- sum(recipes[elves])
    num.new <- (val > 9) + 1
    recipes[num.recipes + num.new] <- val %% 10L
    num.recipes <- num.recipes + num.new
#  cat(recipes, "\n")
    elves <- (elves + recipes[elves]) %% num.recipes + 1
  }
  return(recipes[gens + 1:10])
}
make.recipes.slow <- function(recipes, gens) {
  num.elves <- length(recipes)
  elves <- 1:num.elves
  gens <- as.integer(gens)
  while (length(recipes) < gens + 10) {
    new <- as.integer(unlist(strsplit(as.character(sum(recipes[elves])), "")))
    recipes <- c(recipes, new)
#  cat(recipes, "\n")
    for (idx in 1:num.elves) {
      elves[idx] <- (elves[idx] + recipes[elves[idx]]) %% length(recipes) + 1
    }
  }
  return(recipes[gens + 1:10])
}
cat("Part 1: ", paste(make.recipes(start.recipes, gens), collapse=""), "\n")

find.sequence <- function(start.recipes, sequence) {
  num.recipes <- num.elves <- length(start.recipes)
  elves <- 1:num.elves
  recipes <- rep(1L, as.integer(sequence) * 50)
  recipes[1:num.recipes] <- start.recipes
  sequence <- as.integer(unlist(strsplit(sequence, "")))
  len.sequence <- length(sequence)
  repeat {
    val <- sum(recipes[elves])
    num.new <- (val > 9) + 1
    recipes[num.recipes + num.new] <- val %% 10L
    num.recipes <- num.recipes + num.new
    elves <- (elves + recipes[elves]) %% num.recipes + 1
#    cat(recipes[1:num.recipes], "\n")
    if (num.recipes <= len.sequence)
      next
    last <- (num.recipes - len.sequence + 1):num.recipes
    if (all(recipes[last] == sequence))
      break
    else if (num.new == 2 && all(recipes[last - 1] == sequence)) {
      num.recipes <- num.recipes - 1
      break
    }
  }
  return(list(recipes=recipes[1:num.recipes], num=num.recipes - len.sequence))
}
find.sequence.slow <- function(start.recipes, sequence) {
  num.recipes <- num.elves <- length(start.recipes)
  elves <- 1:num.elves
  recipes <- integer(as.integer(sequence) * 10)
  recipes[1:num.recipes] <- start.recipes
  sequence <- as.integer(unlist(strsplit(sequence, "")))
  len.sequence <- length(sequence)
  repeat {
    new <- as.integer(unlist(strsplit(as.character(sum(recipes[elves])), "")))
    num.new <- length(new)
    recipes[(num.recipes + 1):(num.recipes + num.new)] <- new
    num.recipes <- num.recipes + num.new
#    cat(recipes[1:num.recipes], "\n")
    if (num.recipes >= len.sequence &&
        all(recipes[(num.recipes - len.sequence + 1):num.recipes] == sequence)) {
      break
    }
    for (idx in 1:num.elves) {
      elves[idx] <- (elves[idx] + recipes[elves[idx]]) %% num.recipes + 1
    }
  }
  return(num.recipes - len.sequence)
}
cat("Part 2: ", find.sequence(start.recipes, gens)$num, "\n")
