function power_level(x, y, serialno)
    rack_id = x .+ 10
    power = (rack_id .* y .+ serialno) .* rack_id
    return floor((power % 1000) / 100) - 5
end

function find_square(pows, size)
    grid_size = Base.size(pows, 1)
    last_idx = grid_size - size + 1
    square_idx = 0:size - 1
    max_val, x, y = sum(pows[1 .+ square_idx, 1 .+ square_idx]), 1, 1
    for row in 1:last_idx
        row_idx = row .+ square_idx
        val = sum(pows[row_idx, 1 .+ square_idx])
        for col in 2:last_idx
            val += sum(pows[row_idx, col + size - 1] - pows[row_idx, col - 1])
            if val > max_val
                max_val, x, y = val, col, row
            end
        end
    end
    return (max_val, x, y)
end

serialno = parse(Int, readlines("data/input-11.txt")[1])
grid_size = 300
pows = [power_level(x, y, serialno) for x in 1:grid_size for y in 1:grid_size]
pows = reshape(pows, grid_size, grid_size)

_, x, y = find_square(pows, 3)
res = join([x, y], ",")
println("Part 1: $(res)")
@assert res == "243,16"

max_squares = [find_square(pows, size) for size in 1:grid_size]
(_, x, y), max_idx = findmax(max_squares)
res = join([x, y, max_idx], ",")
println("Part 2: $(res)")
@assert res == "231,227,14"
