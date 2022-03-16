function parseLine(str)
    issue, vals = split(str, " @ ")
    coord, size = split(vals, ": ")
    issue = parse(Int, replace(issue, "#" => ""))
    coord = [parse(Int, i) for i in split(coord, ",")]
    size  = [parse(Int, i) for i in split(size, "x")]
    issue, coord, size
end

vals = [parseLine(i) for i in readlines("data/input-03.txt")]
cover, claim = Dict(), Dict()
for (issue, coord, size) in vals
    lef, top = coord
    wid, hei = size
    for x in lef:lef + wid - 1
        for y in top:top + hei - 1
            val = (x, y)
            if val in keys(cover)
                global cover[val] += 1
                push!(claim[val], issue)
            else
                global cover[val] = 1
                global claim[val] = [issue]
            end
        end
    end
end

res = sum([i > 1 for i in values(cover)])
println("Part 1: $(res)")
@assert(res == 111326)

used = [j for (i, j) in zip(values(cover), values(claim)) if i > 1]
used = Set([i for inner in used for i in inner])
left = collect(setdiff(Set([issue for (issue, _, _) in vals]), used))[1]
println("Part 2: $(left)")
@assert(left == 1019)
