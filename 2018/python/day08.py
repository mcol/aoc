from typing import NamedTuple


class NodeSummary(NamedTuple):
    vec: list
    value: int
    sum_meta: int


def eval_node(vec):
    (num_children, len_metadata), vec = vec[:2], vec[2:]
    children, sum_meta = [], 0
    for _ in range(num_children):
        child = eval_node(vec)
        vec = child.vec
        sum_meta += child.sum_meta
        children.append(child)

    metadata, vec = vec[:len_metadata], vec[len_metadata:]
    sum_meta += sum(metadata)
    if num_children > 0:
        value = 0
        for idx in metadata:
            if 0 < idx <= len(children):
                value += children[idx - 1].value
    else:
        value = sum(metadata)
    return NodeSummary(vec, value, sum_meta)


file = open("data/input-08.txt")
vals = [
    int(v)
    for line in file.readlines()
    for v in line.split(" ")
    if len(line) > 1
]
root = eval_node(vals)

res = root.sum_meta
print(f"Part 1: {res}")
assert res == 41849

res = root.value
print(f"Part 2: {res}")
assert res == 32487
