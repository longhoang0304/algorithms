inp = open("./input", "r").readlines()


def is_safe(report):
    if len(report) <= 1: return True

    diff = report[0] - report[1]
    sign = 1 if diff > 0 else 0
    diff = abs(diff)

    if len(report) == 2:
        return 1 <= diff <= 3
    
    if not (1 <= diff <= 3): return False

    for i in range(1, len(report) - 1):
        diff = report[i] - report[i + 1]
        nsign = 1 if diff > 0 else 0
        diff = abs(diff)

        if nsign != sign: return False
        sign = nsign

        if not (1 <= diff <= 3): return False
    
    return True

def try_safe(report):
    if len(report) <= 2: return True

    diff = report[0] - report[1]
    sign = 1 if diff > 0 else 0
    diff = abs(diff)

    for i in range(0, len(report)):
        if is_safe(report[:i] + report[i + 1:]): return True
    
    return False

safe = 0

for line in inp:
    report = list(map(int, line.split(' ')))
    if is_safe(report):
        safe += 1
        continue

    if try_safe(report):
        safe += 1


print(safe)