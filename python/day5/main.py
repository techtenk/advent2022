
def parseInput():
    f = open("test_input.txt")
    for line in f:
        # split each line by character
        chars = [*line]
        if chars[0] == '\n':
            break

parseInput()