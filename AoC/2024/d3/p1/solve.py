import re

regex = r"mul\((\d{1,3}),(\d{1,3})\)"

inp = open("./input", "r").read()

all_mul = re.findall(regex, inp)

s = 0
for (sa, sb) in all_mul:
    a, b = int(sa), int(sb)
    s += a * b

print(s)