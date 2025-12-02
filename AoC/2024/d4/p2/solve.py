import re

inp = open("./input", "r").read()

# ================

regex = r"mul\((\d{1,3}),(\d{1,3})\)"
all_mul = re.finditer(regex, inp)
smap = []
for mul in all_mul:
    (sa, sb) = mul.groups()
    a, b = int(sa), int(sb)
    smap.append((mul.span()[0], a * b))

# ================

do = [0]
dont = []

all_do = re.finditer(r"do\(\)", inp)
for d in all_do:
    do.append(d.span()[0])

all_dont = re.finditer(r"don't\(\)", inp)
for dnt in all_dont:
    dont.append(dnt.span()[0])

do_int = []
d = do.pop(0)
dt = dont.pop(0)
while do and dont:
    while dont and d >= dt:
        dt = dont.pop(0)

    do_int.append((d, dt))

    while do and d <= dt:
        d = do.pop(0)


while dont and d >= dt:
    dt = dont.pop(0)


if d < dt: do_int.append((d, dt))
else: do_int.append((d, float("inf")))

# ================

s = 0
for p, m in smap:
    for st, ed in do_int:
        if st <= p <= ed:
            s += m
        if st > p:
            break

# ================

print(s)
