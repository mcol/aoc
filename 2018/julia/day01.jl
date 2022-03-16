using DelimitedFiles

nums = readdlm("data/input-01.txt", Int)[:, 1]

res = sum(nums)
println("Part 1: $(res)")
@assert(res == 587)

seen, found, cumsum = Dict(), false, 0
while !found
    for i in nums
        global cumsum += i
        if cumsum in keys(seen)
            global found = true
            break
        else
            seen[cumsum] = true
        end
    end
end
println("Part 2: $(cumsum)")
@assert(cumsum == 83130)
