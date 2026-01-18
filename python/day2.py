#!/usr/bin/env python3
#
import os
from pathlib import Path


def process_range_part1(range_as_string):
    k = []
    a = range_as_string.split("-")
    b = range(int(a[0]), int(a[1]) + 1)
    for c in b:
        d = f"{c}"
        if len(d) % 2 == 0:
            print(f"len{len(d)} {int(len(d) / 2)}")
            e = d[0 : int(len(d) / 2)]
            f = d[int(len(d) / 2) :]
            print(f"{e} {f} {e == f}")
            if e == f:
                k.append(d)
        else:
            # print(f"len{len(d)}- {d}")
            pass
    # range()
    return k


def split_equal_from_gemini(text, num_parts):
    step = len(text) // num_parts
    return [text[i : i + step] for i in range(0, len(text), step)]


def process_range_part2(range_as_string):
    invalids = []
    start_end = range_as_string.split("-")
    range_to_check = range(int(start_end[0]), int(start_end[1]) + 1)
    for c in range_to_check:
        num = f"{c}"
        lenn = len(num)
        for div in range(1, (lenn // 2) + 2):
            modd = lenn % div
            parts = split_equal_from_gemini(num, div)
            if modd == 0 and lenn > 1:
                # print(f"len {lenn} div {div} mod {modd} num {num} parts {parts}")
                if div == 1:
                    prev = -1
                    # print(f"Single digit checks on {num}")
                    for char in num:
                        if prev != -1:
                            # print(f"Further Pass digit {char} vs {prev}")
                            if int(prev) == int(char):
                                # print("Chars Match")
                                prev = int(char)
                            else:
                                # print("Chars don't match, break")
                                prev = -2
                                break
                        else:
                            # print(f"First pass digit {char}")
                            prev = int(char)
                    # endfor
                    # print(f"prev {prev}")
                    if prev >= 0:
                        if num not in invalids:
                            invalids.append(num)
                            # print(f"New invalids list {invalids}")
                else:
                    prev = ""
                    for n in parts:
                        if prev != "":
                            # print(f"Further multi-digit check {prev} vs {n}")
                            if prev == n:
                                # print("Match!")
                                prev = n
                            else:
                                # print("NoMatch break")
                                prev = "x"
                                break
                        else:
                            # print(f"Multi digit checks on {num}")
                            prev = n
                    # endfor
                    if prev != "" and prev != "x":
                        if num not in invalids:
                            invalids.append(num)
                            # print(f"New invalids list {invalids}")
    return invalids


def open_myself(filename):
    invalid = []
    sum = 0
    file_path = Path(filename)
    if file_path.exists():
        buff = []
        print(file_path, os.path.getsize(file_path))
        with open(file_path, "r") as file:
            while True:
                char = file.read(1)
                if char == "," or char == "":
                    the_range = "".join(buff)
                    print(the_range)
                    # j = process_range_part1(the_range)
                    j = process_range_part2(the_range)
                    for l in j:
                        invalid.append(l)
                    # print(f"invalid {invalid}")
                    buff = []
                    if char == "":
                        break
                elif char != "\n":
                    buff.append(char)
                # endif
            # endwhile
        # endwhile
        print(f"invalid{invalid}")
        for m in invalid:
            sum += int(m)
        print(f"{sum}")
    # endif


# enddef

open_myself("day2/biginput.txt")
