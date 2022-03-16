import sequtils, sugar, tables

type Workers = object
    MAX: int
    tasks: seq[(char, int)]
    time: int

proc initWorkers(max_workers: int): Workers =
    Workers(MAX: max_workers, time: 0)

proc add(self: var Workers, task: char, task_length: int) =
    if len(self.tasks) == self.MAX:
        return
    for (tt, time) in self.tasks:
        if tt == task:
            return
    self.tasks.add((task, self.time + task_length))

proc ended_tasks(self: var Workers): seq[char] =
    var min_time = high(int)
    for (task, time) in self.tasks:
        min_time = min(time, min_time)
    self.time = min_time
    let rem = collect(newSeq):
        for (task, time) in self.tasks:
            if time == self.time:
                task
    self.tasks = collect(newSeq):
        for (task, time) in self.tasks:
            if task notin rem:
                (task, time)
    return rem

proc remove_task(task_list: var Table[char, seq[char]], task: char) =
    task_list.del(task)
    for k, v in task_list.pairs():
        task_list[k] = v.filter(t => t != task)

var prec = initTable[char, seq[char]]()
for line in lines("data/input-07.txt"):
    let (bef, aft) = (line[5], line[36])
    if not prec.hasKey(bef):
        prec[bef] = newSeq[char]()
    if not prec.hasKey(aft):
        prec[aft] = newSeq[char]()
    prec[aft].add(bef)

var (prec1, res1) = (prec, "")
while len(prec1) > 0:
    var all_tasks = collect(newSeq):
        for k, v in prec1.mpairs():
            (len(v), k)
    let task = min(all_tasks)[1]
    prec1.remove_task(task)
    res1.add(task)
echo "Part 1: ", res1
assert(res1 == "BCADPVTJFZNRWXHEKSQLUYGMIO")

var (workers, res2) = (initWorkers(5), 0)
while len(prec) > 0:
    let next_tasks = collect(newSeq):
        for (k, v) in prec.pairs():
            if len(v) == 0:
                k
    for task in next_tasks:
        workers.add(task, 60 + ord(task) - ord('A') + 1)
    for task in workers.ended_tasks():
        prec.remove_task(task)
        res2 = workers.time
echo "Part 2: ", res2
assert(res2 == 973)
