#!/usr/bin/env python3
import math
import sys

import numpy as np

raw_lines = np.array([])
columns = np.array([])
lines = np.array([])
is_part2 = False
with open(sys.argv[1], "r") as input:
    if len(sys.argv) > 2 and sys.argv[2] == "part2":
        is_part2 = True
    for line in input:
        # print(len(lines))
        s = line.split()
        if len(lines) == 0:
            lines = np.array(s)
            raw_lines = np.array(line)
        else:
            lines = np.vstack([lines, s])
            raw_lines = np.vstack([raw_lines, line])
        # print(s)
        # print(len(s))


def part1(liness):
    print(liness.shape)
    print(liness)
    columnss = np.array([])
    if len(columnss) == 0:
        columnss = liness.T
    print(columnss)
    total = 0
    for c in columnss:
        lenn = len(c)
        nums = c[: lenn - 1].astype(int)
        op = c[lenn - 1 :]
        print(op)
        print(nums)
        if op == "*":
            p = math.prod(nums)
            print(p)
            total += p
        elif op == "+":
            s = sum(nums)
            print(s)
            total += s
    print(total)


def part2():
    print(raw_lines)
    pass


if not is_part2:
    part1(lines)
else:
    part2()
