inp = open("./input", "r").readlines()

def is_safe(report):
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

        if 1 <= diff <= 3:
            continue
        
        return False
    
    return True

safe = 0

for line in inp:
    report = list(map(int, line.split(' ')))
    if is_safe(report):
        safe += 1

print(safe)