mutable struct Workers
    MAX::Int
    tasks::Array{Tuple{Char, Int}}
    time::Int
    Workers() = new(5, [], 0)
end

function add(self::Workers, task, task_length)
    if length(self.tasks) == self.MAX ||
        any([tt == task for (tt, _) in self.tasks])
        return
    end
    push!(self.tasks, (task, self.time + task_length))
end

function ended_tasks(self::Workers)
    self.time = minimum([time for (_, time) in self.tasks])
    rem = [task for (task, time) in self.tasks if time == self.time]
    self.tasks = [(task, time) for (task, time) in self.tasks if !(task in rem)]
    return rem
end

function remove_task(task_list, task)
    delete!(task_list, task)
    for (k, v) in task_list
        task_list[k] = setdiff(v, task)
    end
end

prec = Dict{Char, Array{Char}}()
for line in readlines("data/input-07.txt")
    bef, aft = line[6], line[37]
    if !(bef in keys(prec))
        prec[bef] = []
    end
    if !(aft in keys(prec))
        prec[aft] = []
    end
    push!(prec[aft], bef)
end

prec1, res = copy(prec), ""
while !isempty(prec1)
    _, task = minimum([(length(v), k) for (k, v) in prec1])
    remove_task(prec1, task)
    global res *= task
end
println("Part 1: $(res)")
@assert(res == "BCADPVTJFZNRWXHEKSQLUYGMIO")

workers, res = Workers(), 0
while !isempty(prec)
    next_tasks = [k for (k, v) in prec if length(v) == 0]
    for task in next_tasks
        add(workers, task, 60 + task - 'A' + 1)
    end
    for task in ended_tasks(workers)
        remove_task(prec, task)
        global res = workers.time
    end
end
println("Part 2: $(res)")
@assert(res == 973)
