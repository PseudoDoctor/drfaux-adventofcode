#!/usr/bin/env python3
#
def process_grid_sliding_window(filename):
    t = 0
    with open(filename, "r") as f:
        # Pre-load the first two rows (stripped and converted to lists)
        # Use next() to get lines one by one
        try:
            prev_row = None
            curr_row = list(next(f).strip())
            next_row = list(next(f).strip())
        except StopIteration:
            # Handle files with fewer than 2 lines
            if "curr_row" in locals():
                process_row(None, curr_row, None)
            return

        # Process the middle rows
        for line in f:
            t += process_row(prev_row, curr_row, next_row)
            # Slide the window forward
            prev_row = curr_row
            curr_row = next_row
            next_row = list(line.strip())

        # Process the final row (where next_row is now the last line)
        t += process_row(prev_row, curr_row, next_row)
        t += process_row(curr_row, next_row, None)
        print(t)


def process_row(prev, curr, nxt):
    """Processes 'curr' using 'prev' and 'nxt' for neighbor checks."""
    row_output = []
    cols = len(curr)

    for c in range(cols):
        if curr[c] == "@":
            count = 0
            # Check neighbors in the 3 rows provided
            for r_data in [prev, curr, nxt]:
                if r_data is not None:
                    # Check column to left, center, and right
                    for dc in [-1, 0, 1]:
                        nc = c + dc
                        if 0 <= nc < len(r_data):
                            if r_data[nc] == "@":
                                count += 1

            # The logic above counts the center "@" itself, so subtract 1
            row_output.append(str(count - 1))
        else:
            row_output.append("0")

    print("".join(row_output))
    t = 0
    for c in row_output:
        if int(c) > 0 and int(c) < 4:
            t += 1
    return t


# Execute
process_grid_sliding_window("day4/biginput.txt")
