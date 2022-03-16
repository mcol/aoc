class Workers:
    MAX = 5

    def __init__(self):
        self.tasks = []
        self.time = 0

    def add(self, task, length):
        if len(self.tasks) == self.MAX or any(
            [tt == task for (tt, _) in self.tasks]
        ):
            return
        self.tasks.append((task, self.time + length))

    def ended_tasks(self):
        _, self.time = min(self.tasks, key=lambda t: t[1])
        rem = [task for (task, time) in self.tasks if time == self.time]
        self.tasks = [
            (task, time) for (task, time) in self.tasks if task not in rem
        ]
        return rem


def remove_task(task_list, task):
    del task_list[task]
    for (k, v) in task_list.items():
        task_list[k] = [i for i in v if not i == task]


file = open("data/input-07.txt")
prec = {}
for line in file.readlines():
    bef, aft = line[5:6], line[36:37]
    if not bef in prec:
        prec[bef] = []
    if not aft in prec:
        prec[aft] = []

    prec[aft].append(bef)

prec1, res = prec.copy(), ""
while prec1:
    task, _ = min(prec1.items(), key=lambda t: len(t[1]) * 1000 + ord(t[0]))
    remove_task(prec1, task)
    res += task

print(f"Part 1: {res}")
assert res == "BCADPVTJFZNRWXHEKSQLUYGMIO"

workers = Workers()
while prec:
    next_tasks = [k for (k, v) in prec.items() if len(v) == 0]
    for task in next_tasks:
        workers.add(task, 60 + ord(task) - ord("A") + 1)
    for task in workers.ended_tasks():
        remove_task(prec, task)

res = workers.time
print(f"Part 2: {res}")
assert res == 973
