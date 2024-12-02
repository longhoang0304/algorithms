from collections import Counter

inp = open("./input", "r").readlines()
left = set()
right = Counter()

for line in inp:
    l, r = list(map(lambda x: int(x), line.split()))
    left.add(l)
    right[r] += 1

sim = 0

for v in left:
    sim += v * right[v]

print(sim)