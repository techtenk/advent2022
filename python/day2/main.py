
def part1():
    f = open("input.txt")

    score = 0
    for line in f:
        match line.split():
            case ["A", "X"]: # Rock v Rock
                score += (1 + 3)
            case ["A", "Y"]: # Rock v Paper
                score += (2 + 6)
            case ["A", "Z"]: # Rock v Scissors
                score += (3 + 0)
            case ["B", "X"]: # Paper v Rock
                score += (1 + 0)
            case ["B", "Y"]: # Paper v Paper
                score += (2 + 3)
            case ["B", "Z"]: # Paper v Scissors
                score += (3 + 6)
            case ["C", "X"]: # Scissors v Rock
                score += (1 + 6)
            case ["C", "Y"]: # Scissors v Paper
                score += (2 + 0)
            case ["C", "Z"]: # Scissors v Scissors
                score += (3 + 3)
    print("Part 1: {}".format(score))

def part2():
    f = open("input.txt")

    score = 0
    for line in f:
        match line.split():
            case ["A", "X"]: # Lose to Rock
                score += (3 + 0)
            case ["A", "Y"]: # Draw with Rock
                score += (1 + 3)
            case ["A", "Z"]: # Win to Rock
                score += (2 + 6)
            case ["B", "X"]: # Lose to Paper
                score += (1 + 0)
            case ["B", "Y"]: # Draw with Paper
                score += (2 + 3)
            case ["B", "Z"]: # Win to Paper
                score += (3 + 6)
            case ["C", "X"]: # Lose to Scissors
                score += (2 + 0)
            case ["C", "Y"]: # Draw with Scissors
                score += (3 + 3)
            case ["C", "Z"]: # Win to Scissors
                score += (1 + 6)
    print("Part 2: {}".format(score))

part1()
part2()
        