mutable struct Image
    row::Array{Int}
    col::Array{Int}
    hor_vel::Array{Int}
    ver_vel::Array{Int}
    Image() = new([], [], [], [])
end

function add_point!(self::Image, point::Array{Int})
    push!(self.row, point[1])
    push!(self.col, point[2])
    push!(self.hor_vel, point[3])
    push!(self.ver_vel, point[4])
end

function move!(self::Image, moves=1)
    self.row += self.hor_vel * moves
    self.col += self.ver_vel * moves
end

function compute_area(self::Image)
    abs(reduce(-, extrema(self.row)) * reduce(-, extrema(self.col)))
end

function draw(self::Image)
    row = self.row .- minimum(self.row) .+ 1
    col = self.col .- minimum(self.col) .+ 1
    wid, hei = maximum(row) + 1, maximum(col) + 1
    render = [[' ' for _ in 1:wid] for _ in 1:hei]
    for (i, j) in zip(row, col)
        render[j][i] = '#'
    end
    for row in 1:hei
        println(join(render[row]))
    end
end

function parseLine(str)
    str = replace(str, "position=<" => "")
    str = replace(str, "> velocity=<" => ",")
    str = replace(str, ">" => "")
    [parse(Int, i) for i in split(str, ",")]
end

image = Image()
for line in readlines("data/input-10.txt")
    add_point!(image, parseLine(line))
end

min_area, min_iter = compute_area(image), 0
for it in 1:11000
    move!(image)
    area = compute_area(image)
    if area < min_area
        global min_area, min_iter = area, it
    else
        move!(image, -1)
    end
end

println("Part 1:")
draw(image)

res = min_iter
println("Part 2: $(res)")
@assert(res == 10888)
