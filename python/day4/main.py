def part1():
    f = open("input.txt")
    count = 0
    for line in f:
        ranges = line.split(',')
        range1 = range(*map(lambda x: int(x), ranges[0].split('-')))
        range2 = range(*map(lambda x: int(x), ranges[1].split('-')))
        if set(range2).issubset(set(range1)) or set(range1).issubset(set(range2)):
            count += 1
    print("Part 1: {}".format(count))

part1()