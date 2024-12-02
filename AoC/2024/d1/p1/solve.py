from heapq import *

inp = open("./input", "r").readlines()
left = []
right = []

for line in inp:
    l, r = list(map(lambda x: int(x), line.split()))
    heappush(left, l)
    heappush(right, r)

dist = 0

while left:
    ml = heappop(left)
    mr = heappop(right)
    d = abs(ml - mr)
    dist += d

print(dist)