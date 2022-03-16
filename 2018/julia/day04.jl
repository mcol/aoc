@enum State STARTED AWAKE ASLEEP

mutable struct Event
    guard::Int
    state::State
    beg_sleep::Int
    end_sleep::Int
    Event(guard, state, beg_sleep) = new(guard, state, beg_sleep, 0)
end

vals = sort([i for i in readlines("data/input-04.txt")])

events, guards, current_guard = [], Set{Int}(), 0
for (idx, line) in enumerate(vals)
    minute = parse(Int, line[16:17])
    if idx > 1 && events[idx - 1].state == ASLEEP
        events[idx - 1].end_sleep = minute
    end
    case = line[20:24]
    if case == "falls"
        state = ASLEEP
        beg_sleep = minute
    elseif case == "wakes"
        state = AWAKE
        beg_sleep = 0
    elseif case == "Guard"
        state = STARTED
        global current_guard = parse(Int, match(r"#([0-9]+)", line).captures[1])
        push!(guards, current_guard)
        beg_sleep = 0
    else
        @assert(false, "unreachable")
    end
    push!(events, Event(current_guard, state, beg_sleep))
end

guards, sleep, minutes = collect(guards), zeros(Int, length(guards)), []
for _ in guards
    push!(minutes, zeros(Int, 60))
end
for event in events
    if event.state != ASLEEP
        continue
    end
    idx = [idx for (idx, guard) in enumerate(guards) if guard == event.guard][1]
    sleep[idx] += event.end_sleep - event.beg_sleep
    for mm in event.beg_sleep:event.end_sleep
        minutes[idx][mm + 1] += 1
    end
end

max_minute = zeros(Int, length(guards))
max_times  = zeros(Int, length(guards))
for (idx, minute) in enumerate(minutes)
    max_times[idx], max_minute[idx] = findmax(minute)
end

guard_id = argmax(sleep)
res = guards[guard_id] * (max_minute[guard_id] - 1)
println("Part 1: $(res)")
@assert(res == 12169)

guard_id = argmax(max_times)
res = guards[guard_id] * (max_minute[guard_id] - 1)
println("Part 2: $(res)")
@assert(res == 16164)
