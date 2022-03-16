mutable struct Marble
    prev::Int
    next::Int
end

mutable struct Ring
    curr::Int
    vals::Array{Marble}
    Ring(max_size) = new(1, [Marble(1, 1)  for _ in 1:max_size + 1])
end

function insert!(self::Ring, value)
    next_value = self.vals[self.curr].next
    self.vals[value] = Marble(self.curr, next_value)
    self.vals[self.curr].next = value
    self.vals[next_value].prev = value
end

function remove!(self::Ring)
    curr, rem = self.curr, self.vals[self.curr]
    prev_value, next_value = rem.prev, rem.next
    self.vals[prev_value].next = next_value
    self.vals[next_value].prev = prev_value
    self.curr = prev_value
    return curr
end

function step!(self::Ring, steps)
    for _ in 1:steps
        self.curr = self.vals[self.curr].next
    end
end

function back!(self::Ring, steps)
    for _ in 1:steps
        self.curr = self.vals[self.curr].prev
    end
end

function compute_scores(num_players, num_marbles)
    player, scores = 0, zeros(Int, num_players)
    marbles = Ring(num_marbles)
    for marble in 1:num_marbles
        player = player % num_players + 1
        step!(marbles, 2)
        if marble % 23 == 0
            back!(marbles, 8)
            scores[player] += marble + remove!(marbles) - 1
        else
            insert!(marbles, marble + 1)
        end
    end
    return scores
end

vals = [v for line in readlines("data/input-09.txt") for v in split(line, " ")]
num_players, num_marbles = parse(Int, vals[1]), parse(Int, vals[7])

res = maximum(compute_scores(num_players, num_marbles))
println("Part 1: $(res)")
@assert(res == 385820)

res = maximum(compute_scores(num_players, num_marbles * 100))
println("Part 2: $(res)")
@assert(res == 3156297594)
