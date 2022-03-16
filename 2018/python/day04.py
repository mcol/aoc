from enum import Enum, auto
from re import findall


class State(Enum):
    STARTED = auto()
    AWAKE = auto()
    ASLEEP = auto()


class Event:
    def __init__(self, guard, state, beg_sleep):
        self.guard = guard
        self.state = state
        self.beg_sleep = beg_sleep
        self.end_sleep = 0


file = open("data/input-04.txt")
vals = [i.strip() for i in file.readlines()]
vals.sort()

events, guards = [], set()
for idx, line in enumerate(vals):
    minute = int(line[15:17])
    if idx > 0 and events[idx - 1].state == State.ASLEEP:
        events[idx - 1].end_sleep = minute

    case = line[19:24]
    if case == "falls":
        state = State.ASLEEP
        beg_sleep = minute
    elif case == "wakes":
        state = State.AWAKE
        beg_sleep = 0
    elif case == "Guard":
        state = State.STARTED
        [current_guard] = findall("#([0-9]+)", line)
        guards.add(current_guard)
        beg_sleep = 0
    else:
        assert False, "unreachable"

    events.append(Event(current_guard, state, beg_sleep))

guards, sleep, minutes = list(guards), [0] * len(guards), []
for i in range(len(guards)):
    minutes.append([0] * 60)
for event in events:
    [idx] = [idx for idx, guard in enumerate(guards) if guard == event.guard]
    sleep[idx] += event.end_sleep - event.beg_sleep
    for ii in range(event.beg_sleep, event.end_sleep):
        minutes[idx][ii] += 1

max_minute, max_times = [0] * len(guards), [0] * len(guards)
for idx, minute in enumerate(minutes):
    max_minute[idx] = minute.index(max(minute))
    max_times[idx] = max(minute)

guard_id = sleep.index(max(sleep))
res = int(guards[guard_id]) * max_minute[guard_id]
print(f"Part 1: {res}")
assert res == 12169

guard_id = max_times.index(max(max_times))
res = int(guards[guard_id]) * max_minute[guard_id]
print(f"Part 2: {res}")
assert res == 16164
