#!/usr/bin/env python3
#
import os
from pathlib import Path


def count_neighbors(grid):
    rows = len(grid)
    cols = len(grid[0])
    # Relative positions of the 8 neighbors (row_offset, col_offset)
    neighbors = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]

    # Create a results grid (same size)
    result = [[0 for _ in range(cols)] for _ in range(rows)]
    result2 = [[0 for _ in range(cols)] for _ in range(rows)]

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == "@":
                count = 0
                for dr, dc in neighbors:
                    nr, nc = r + dr, c + dc
                    # Check bounds and if neighbor is an "@"
                    if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == "@":
                        count += 1
                result[r][c] = count
                if count > 0 and count < 4:
                    result2[r][c] = 1
            else:
                result[r][c] = 0  # Or leave as "." if you prefer
    t = 0
    for r in result2:
        for c in r:
            t += c
    print(t)
    return result


def open_myself(filename):
    file_path = Path(filename)
    if file_path.exists():
        buff = []
        total = 0
        print(file_path, os.path.getsize(file_path))
        with open(file_path, "r") as file:
            for line in file:
                cleaned_line = line.strip()
                if cleaned_line:
                    buff.append(list(cleaned_line))
                    # print(",".join(list(cleaned_line)))
                    pass
        counts = count_neighbors(buff)
        for row in counts:
            # print(",".join(map(str, row)))
            for c in row:
                if c > 0 and c < 4:
                    total += 1
        print(total)


open_myself("day4/biginput.txt")
