#!/usr/bin/env python3
import sys

import numpy as np

fresh_ranges = []
ingredient_ids = []
fresh_ids = []


def process_ranges_part2(ranges=fresh_ranges):
    if not ranges:
        return []

    # 1. Sort by the start value
    sorted_ranges = sorted(ranges, key=lambda r: r.start)

    merged = [sorted_ranges[0]]

    for current in sorted_ranges[1:]:
        previous = merged[-1]

        # If the current range starts before or exactly where the previous ends
        if current.start <= previous.stop:
            # Merge them by taking the maximum stop value
            new_stop = max(previous.stop, current.stop)
            merged[-1] = range(previous.start, new_stop)
        else:
            merged.append(current)

    return merged


def process_ranges_part1(ids=ingredient_ids, ranges=fresh_ranges):
    for r in ranges:
        print(f"{r}")
        for i in ids:
            if int(i) in r:
                print(f"{i} yay")
                fresh_ids.append(int(i))
    pass


def process_ingredient(id: str, ranges=fresh_ranges):
    print(f"{id}")
    for r in ranges:
        for s in r:
            if int(id) == s:
                print(f"{id} not spoiled {s}")
                return True

        pass

    pass
    print(f"{id} is spoiled!")


is_part2 = False
with open(sys.argv[1], "r") as input:
    if len(sys.argv) > 2 and sys.argv[2] == "part2":
        is_part2 = True
    is_range = True
    linenum = 0
    for line in input:
        linenum += 1
        if line == "\n":
            is_range = False
        else:
            if is_range:
                if "-" in line:
                    print(f"{linenum}: {line.strip()} is a range")
                    pass
                a = line.strip().split("-")
                fresh_ranges.append(range(int(a[0]), int(a[1]) + 1))
            else:
                if is_part2:
                    break
                if "-" not in line:
                    print(f"{linenum}: {line.strip()} is an ingredient")
                    pass
                ingredient_ids.append(line.strip())


if not is_part2:
    fresh_count = 0
    print(f"{fresh_count}")
    for i in ingredient_ids:
        print(f"{i}")
        is_fresh = False
        is_fresh = process_ingredient(i)
        if is_fresh:
            fresh_count += 1
    print(f"{fresh_count=}")
    process_ranges_part1()
    u = np.unique(fresh_ids)
    print(f"{len(u)}")
else:
    v = process_ranges_part2()
    print(v)
    w = sum(len(r) for r in v)
    print(w)
