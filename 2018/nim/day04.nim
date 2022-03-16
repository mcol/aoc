import algorithm, std/enumerate, sequtils, sets, strutils, sugar

type State = enum
    STARTED, AWAKE, ASLEEP

type Event = object
    guard: int
    state: State
    beg_sleep: int
    end_sleep: int

var vals = readFile("data/input-04.txt").strip().splitLines()
vals.sort()

var events: seq[Event]
var guards: HashSet[int]
var current_guard = 0
for idx, line in enumerate(vals):
    let minute = line[15..16].parseInt()
    if idx > 0 and events[idx - 1].state == State.ASLEEP:
        events[idx - 1].end_sleep = minute
    var state: State
    var beg_sleep = 0
    case line[19..23]
    of "falls":
        state = State.ASLEEP
        beg_sleep = minute
    of "wakes":
        state = State.AWAKE
    of "Guard":
        state = State.STARTED
        current_guard = line.split("#")[1].split()[0].parseInt()
        guards.incl(current_guard)
    else:
        assert(false, "unreacahble")
    events.add(Event(guard: current_guard, state: state,
                     beg_sleep: beg_sleep, end_sleep: 0))

let (num_guards, guards_seq) = (len(guards), toSeq(guards))
var sleep = newSeq[int](num_guards)
var minutes = newSeq[array[60, int]](num_guards)
for event in events:
    if event.state != ASLEEP:
        continue
    let idx_seq = collect(newSeq):
        for (idx, guard) in enumerate(guards_seq):
            if guard == event.guard: idx
    let idx = idx_seq[0]
    sleep[idx] += event.end_sleep - event.beg_sleep
    for mm in event.beg_sleep..event.end_sleep:
        minutes[idx][mm] += 1

var max_minute, max_times = newSeq[int](num_guards)
for (idx, minute) in enumerate(minutes):
    max_times[idx] = minute.max()
    max_minute[idx] = minute.find(max(minute))

let guard_id1 = sleep.find(max(sleep))
let res1 = guards_seq[guard_id1] * max_minute[guard_id1]
echo "Part 1: ", res1
assert(res1 == 12169)

let guard_id2 = max_times.find(max(max_times))
let res2 = guards_seq[guard_id2] * max_minute[guard_id2]
echo "Part 2: ", res2
assert(res2 == 16164)
