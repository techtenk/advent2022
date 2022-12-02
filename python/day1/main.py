

def part1():
    f = open("input.txt", "r")

    highest = 0
    current = 0
    for num in f:
        if num.strip().isnumeric():
            current += int(num)
        else:
            if current > highest:
                highest = current
            current = 0

    print("Part 1: {}".format(highest))

def part2():

    f = open("input.txt", "r")

    highest = [0,0,0]
    iter = 0
    current = 0
    lines = f.readlines() + ["\n"] # add another newline so that my logic works on the last line of the file
    for num in lines:
        if num.strip().isnumeric():
            current += int(num)
        else:
            if current > min(highest):
                highest.append(current)
                highest.sort(reverse=True) #sort in descending order
                highest.pop() #remove the last element (smallest)
            current = 0

    print("Part 2: {}".format(sum(highest)))

part1()
part2()