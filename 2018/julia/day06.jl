function manhattan(a, b)
    sum([abs(x - y) for (x, y) in zip(a, b)])
end

coords = [map(x -> parse(Int, x), split(str, ", "))
          for str in readlines("data/input-06.txt")]
min_row, max_row = extrema([r for (r, _) in coords])
min_col, max_col = extrema([c for (_, c) in coords])
owner, infinite, region_size = zeros(Int, length(coords)), Set(), 0
for row in min_row:max_row
    for col in min_col:max_col
        dist = [manhattan(i, [row, col]) for i in coords]
        global region_size += sum(dist) < 10000
        min_idx = [idx for (idx, d) in enumerate(dist) if d == minimum(dist)]
        if length(min_idx) == 1
            owner[min_idx[1]] += 1
            if row in [min_row, max_row] || col in [min_col, max_col]
                push!(infinite, min_idx[1])
            end
        end
    end
end

res = maximum([val for (idx, val) in enumerate(owner) if !(idx in infinite)])
println("Part 1: $(res)")
@assert(res == 3907)

res = region_size
println("Part 2: $(res)")
@assert(res == 42036)
