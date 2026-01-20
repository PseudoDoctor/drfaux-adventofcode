#!/usr/bin/env python3
#
import os
from pathlib import Path


def list_to_int(list):
    return "".join(map(str, list))


def shift_inject(new_digit, index, original_list):
    t = original_list
    print(index)
    # print(t)
    s = original_list[: index - 1] + original_list[index:] + [new_digit]
    # print(s)
    return s


def get_largest_subsequence(digit_string, k=12):
    stack = []
    # How many digits we are allowed to discard
    to_remove = len(digit_string) - k

    for digit in digit_string:
        # While we can still remove digits and the current digit is bigger
        # than the last one we kept, discard the smaller one.
        while to_remove > 0 and stack and stack[-1] < digit:
            stack.pop()
            to_remove -= 1
        stack.append(digit)

    # Ensure we only return exactly k digits
    # (handles cases where the string was already decreasing)
    result = "".join(stack[:k])
    # print(f"{len(result)} {result}")
    thing = int(result)
    # print(len(f"{thing}"), thing)
    return int(result)


def process_line_part2(str, bat_count=12):
    axual = str.strip()
    lenn = len(axual)
    max_jolts = 0
    print(f"{lenn} {axual}")
    # magix
    return get_largest_subsequence(str)


def process_line_part1(str):
    total_jolts = 0
    max_jolts_values = [0, 0]
    max_jolts_indecies = [0, 0]
    tmp_max_values = [0, 0]
    tmp_max_indicies = [0, 0]
    for i, char in enumerate(str.strip()):
        if char not in max_jolts_values:
            if max_jolts_values[0] == 0:
                max_jolts_values[0] = int(char)
                max_jolts_indecies[0] = i
                # print("First pass")
            else:
                # print(f"processing {char} compared to {max_jolts_values}")
                tmp_max_existing = int(f"{max_jolts_values[0]}{max_jolts_values[1]}")
                tmp_max_replace = int(f"{max_jolts_values[0]}{char}")
                tmp_max_shift = int(f"{max_jolts_values[1]}{char}")
                print(f"e: {tmp_max_existing} r: {tmp_max_replace} s: {tmp_max_shift}")
                if (
                    tmp_max_existing < tmp_max_replace
                    or tmp_max_existing < tmp_max_shift
                ):
                    if tmp_max_replace < tmp_max_shift:
                        print("shift")
                        max_jolts_values = [max_jolts_values[1], int(char)]
                    else:
                        print("replace")
                        max_jolts_values = [max_jolts_values[0], int(char)]
                else:
                    # print("noop")
                    pass

    total_jolts = int(f"{max_jolts_values[0]}{max_jolts_values[1]}")
    print(f"Found max values in order:{total_jolts}")
    return total_jolts


def open_myself(filename):
    total_jolts = 0
    file_path = Path(filename)
    if file_path.exists():
        buff = ""
        print(file_path, os.path.getsize(file_path))
        with open(file_path, "r") as file:
            while True:
                buff = file.readline()
                if buff == "":
                    break
                else:
                    # jolts = process_line_part1(buff)
                    jolts = process_line_part2(buff.strip())
                    print(jolts)
                    total_jolts += jolts
        print(f"Jolts: {total_jolts}")


open_myself("day3/biginput.txt")
