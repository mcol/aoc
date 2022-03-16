using DelimitedFiles

vals = readdlm("data/input-02.txt")[:, 1]
letters = unique([only(v) for val in vals for v in split(val, "")])

num2, num3 = 0, 0
for val in vals
    counts = [count(letter -> (letter == l), val) for l in letters]
    global num2 += 2 in counts
    global num3 += 3 in counts
end
res = num2 * num3
println("Part 1: $(res)")
@assert(res == 6200)

left = copy(vals)
for v1 in vals
    deleteat!(left, findall(x -> x == v1, left))
    for v2 in left
        if sum([i != j for (i, j) in zip(v1, v2)]) == 1
            global res = join([i for (i, j) in zip(v1, v2) if i == j])
        end
    end
end
println("Part 2: $(res)")
@assert(res == "xpysnnkqrbuhefmcajodplyzw")
