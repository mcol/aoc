function react(polymer, skip)
    reacted, last = [], " "
    for char in polymer
        if lowercase(char) == skip
            continue
        end
        if char != last && lowercase(char) == lowercase(last)
            deleteat!(reacted, length(reacted))
            last = length(reacted) > 0 ? reacted[length(reacted)] : " "
            continue
        end
        push!(reacted, char)
        last = char
    end
    reacted
end

polymer = readlines("data/input-05.txt")[1]

res = length(react(polymer, " "))
println("Part 1: $(res)")
@assert(res == 9296)

res = minimum([length(react(polymer, c)) for c in 'a':'z'])
println("Part 2: $(res)")
@assert(res == 5534)
