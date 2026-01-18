#!/usr/bin/env python3
#
import os
from pathlib import Path


def process_line_part2(str, bat_count):
    lenn = len(str)
    max_jolts = 0
    # magix
    return max_jolts


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
                    jolts = process_line_part2(buff, 3)
                    total_jolts += jolts
        print(f"Jolts: {total_jolts}")


open_myself("day3/smallinput.txt")
