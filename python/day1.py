#!/usr/bin/env python3

# brute force variables
import numpy as np

# Creates a list of 100 zeros
arr = [0] * 100


def brute_force_part2(start, ticks):
    dir = np.sign(ticks)
    h = 0
    k = 0
    stop = start + dir + ticks
    ra = range(start, stop, dir)
    print(
        f"Start:{start} Ticks:{ticks} Stop:{start + dir + ticks}(sign:{dir}) ((range:{ra}))"
    )

    for i in ra:
        print(i)
        if h != 0:
            if i > 99:
                k = i % 100
            elif i < -100:
                k = i % 100
            else:
                k = i
            # print(f"{k}:{arr[k]}, add 1")
            arr[k] += 1
            # print(f"{k}:{arr[k]}, complete")

        h += 1

    # for idx,num in list(enumerate(arr)):
    #     print(f"N:{idx}:{num}")


# normal


def parseline(line):
    left = line[0]
    right = line[1:]
    sign = 0
    if left == "R":
        sign = 1
    elif left == "L":
        sign = -1
    else:
        print("ERROR", line)
    right_int = int(right) * sign
    # print(left)
    # print(right)
    # print(right_int)
    return right_int


# enddef


def move_ticks_part1(start, ticks):
    # Dial has 100 positions, starting at 0 and ending at 99.
    # modulus *should* work fine
    j = start + ticks
    mod = j % 100

    # quo = j // 100
    # print(start,ticks,j,quo,mod)
    return mod


# enddef


def move_ticks_part2(start, ticks):
    land = 0
    passed = 0
    j = start + ticks
    quo = j // 100
    mod = j % 100

    if mod == 0:
        land += 1
        print("landed on 0")
        if quo > 1:
            passed += quo - 1
            print("Also passed 0 going right")
        if quo < -1:
            passed += abs(quo) - 1
            print("Also passed 0 going left")
    elif quo >= 1 and start != 0:
        passed += abs(quo)
        print("Passed 0 going right")
    elif quo <= -1 and start != 0:
        passed += abs(quo)
        print("Passed 0 going left")

    print("start,ticks,j,quo,land,passed,mod", start, ticks, j, quo, land, passed, mod)
    return (land + passed, mod)


# enddef

with open("biginput.txt", "r") as file:
    pos = 50
    land = 0
    passed = 0
    old_pos = 0
    new_pos = 50
    print(pos)
    for line in file:
        # print(line.strip())
        i = parseline(line.strip())
        # bruteforce
        brute_force_part2(pos, i)
        # part 1
        print("Moving", i, "ticks")
        pos = move_ticks_part1(pos, i)
        if pos == 0:
            land += 1
        print("Landing on", pos, "Total 0's ", land)
        # part 2
        # old_pos = new_pos
        # movement = move_ticks_part2(old_pos,i)
        # passed += movement[0]
        # new_pos = movement[1]
        # print("Movement,passed0,newpos",movement,passed,new_pos)

    print(f"Landed:{land} Passed:{arr[0]}")
