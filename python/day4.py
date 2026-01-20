import os
from pathlib import Path


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
