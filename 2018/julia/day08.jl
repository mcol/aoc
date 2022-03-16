struct NodeSummary
    vec::Array{Int}
    value::Int
    sum_meta::Int
end

function eval_node(vec)
    num_children, len_metadata, vec = vec[1], vec[2], vec[3:end]
    children, sum_meta = [], 0
    for _ in 1:num_children
        child = eval_node(vec)
        vec = child.vec
        sum_meta += child.sum_meta
        push!(children, child)
    end
    metadata, vec = vec[1:len_metadata], vec[len_metadata + 1:end]
    sum_meta += sum(metadata)
    if num_children > 0
        value = 0
        for idx in metadata
            if 0 < idx <= length(children)
                value += children[idx].value
            end
        end
    else
        value = sum(metadata)
    end
    return NodeSummary(vec, value, sum_meta)
end

vals = [parse(Int, v) for line in readlines("data/input-08.txt")
            for v in split(line, " ") if length(line) > 1]
root = eval_node(vals)

res = root.sum_meta
println("Part 1: $(res)")
@assert(res == 41849)

res = root.value
println("Part 2: $(res)")
@assert(res == 32487)
