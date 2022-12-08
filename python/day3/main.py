
def get_priority(letter):
    ascii = ord(letter)
    if ascii > 96:
        return ascii - 96
    else:
        return ascii - 64 + 26

def part1():
    f = open("python/day3/input.txt")
    priority = 0
    for line in f:
        length = len(line)
        pack1 = line[0:int(length/2)]
        pack2 = line[int(length/2):length]
        # print("pack1: {}".format(pack1))
        # print("pack2: {}".format(pack2))
        for letter in pack1:
            if letter in pack2:
                # print("found duplicate in pack! {}".format(letter))
                p = get_priority(letter)
                # print("priority is {}".format(p))
                priority += p
                break

    print("Part 1: {}".format(priority))

def part2() :
    f = open("python/day3/input.txt")
    priority = 0
    lines = f.readlines()

    while (len(lines) > 0):
        elf1 = lines.pop(0).strip()
        elf2 = lines.pop(0).strip()
        elf3 = lines.pop(0).strip()
        badge = set(elf1).intersection(elf2).intersection(elf3)
        # print("{}".format(badge))
        letter = badge.pop()
        # print("badge item is: {}".format(letter))
        priority += get_priority(letter)
            
    print("Part 2: {}".format(priority))
part1()
part2()