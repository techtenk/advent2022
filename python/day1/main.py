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

print(highest)

