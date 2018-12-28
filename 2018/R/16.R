opcodes <- list(addr=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] + regs[B + 1]
                       return(regs)
                     },
                addi=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] + B
                       return(regs)
                     },
                mulr=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] * regs[B + 1]
                       return(regs)
                     },
                muli=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] * B
                       return(regs)
                     },
                banr=function(regs, A, B, C) {
                       regs[C + 1] <- bitwAnd(regs[A + 1], regs[B + 1])
                       return(regs)
                     },
                bani=function(regs, A, B, C) {
                       regs[C + 1] <- bitwAnd(regs[A + 1], B)
                       return(regs)
                     },
                borr=function(regs, A, B, C) {
                       regs[C + 1] <- bitwOr(regs[A + 1], regs[B + 1])
                       return(regs)
                     },
                bori=function(regs, A, B, C) {
                       regs[C + 1] <- bitwOr(regs[A + 1], B)
                       return(regs)
                     },
                setr=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1]
                       return(regs)
                     },
                seti=function(regs, A, B, C) {
                       regs[C + 1] <- A
                       return(regs)
                     },
                gtir=function(regs, A, B, C) {
                       regs[C + 1] <- A > regs[B + 1]
                       return(regs)
                     },
                gtri=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] > B
                       return(regs)
                     },
                gtrr=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] > regs[B + 1] 
                       return(regs)
                     },
                eqir=function(regs, A, B, C) {
                       regs[C + 1] <- A == regs[B + 1]
                       return(regs)
                     },
                eqri=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] == B
                       return(regs)
                     },
                eqrr=function(regs, A, B, C) {
                       regs[C + 1] <- regs[A + 1] == regs[B + 1] 
                       return(regs)
                     }
                )

input <- readLines("../data/input-16.txt")
input <- input[input != ""]
input <- data.frame(matrix(input, ncol=3, byrow=TRUE), stringsAsFactors=FALSE)
colnames(input) <- c("before", "instr", "after")
input$before <- gsub("Before: \\[|\\]", "", input$before)
input$after  <- gsub("After:  \\[|\\]", "", input$after)

before <- lapply(strsplit(input$before, ","), as.integer)
after  <- lapply(strsplit(input$after,  ","), as.integer)
instr  <- lapply(strsplit(input$instr,  " "), as.integer)

sample.opcodes <- rep(NA, length(instr))
sample.opidx <- vector("list", length(instr))
for (idx in 1:length(instr)) {
  args <- c(list(regs=before[[idx]]),
            setNames(lapply(instr[[idx]][-1], c), c("A", "B", "C")))
  sample.opcodes[idx] <- instr[[idx]][1]
  for (op.idx in 1:length(opcodes)) {
    res <- do.call(opcodes[[op.idx]], args)
    if (all(res == after[[idx]])) {
      sample.opidx[[idx]] <- c(sample.opidx[[idx]], op.idx)
    }
  }
}
cat("Part 1: ", sum(sapply(sample.opidx, length) > 2), "\n")

final.opcodes <- rep(NA, length(opcodes))
repeat {
  op.one <- which(sapply(sample.opidx, length) == 1)
  op <- data.frame(idx=unlist(sample.opidx[op.one]), val=sample.opcodes[op.one])
  op <- unique(op)
  for (i in 1:nrow(op))
    final.opcodes[op$val[i] + 1] <- op$idx[i]
  sample.opidx <- lapply(sample.opidx, function(z) setdiff(z, op$idx))
  if (all(!is.na(final.opcodes)))
    break
}
input <- readLines("../data/input-16b.txt")
input <- lapply(strsplit(input, " "), as.integer)

regs <- c(0, 0, 0, 0)
for (idx in 1:length(input)) {
  args <- c(list(regs=regs),
            setNames(lapply(input[[idx]][-1], c), c("A", "B", "C")))
  regs <- do.call(opcodes[[final.opcodes[input[[idx]][1] + 1]]], args)
}
cat("Part 2: ", regs[1], "\n")
