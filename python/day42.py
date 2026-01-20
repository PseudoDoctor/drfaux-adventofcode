#!/usr/bin/env python3

import copy
import sys

import numpy as np


def find_rolls(input_array: np.array) -> tuple([np.array, int]):
    """
    Takes an input array representing paper roll locations and finds the
    number of eligible rolls, returning the number and the original modified
    array where eligible rolls are replaced with an 'X'.

    Parameters:
    input_array (numpy.array): Array with empty slots marked with '.' and paper rolls marked with '@'.

    Returns:
    tuple([numpy.array, int]): Returns a tuple, the first value shows an array with an 'X' where the eligible paper rolls were removed.
                               The second tuple value is the total number of eligible rolls that were removed/replaced.

    """
    eligible_rolls = 0
    input_copy = copy.deepcopy(input_array)
    row_len, col_len = input_copy.shape

    for i in range(row_len):
        for j in range(col_len):
            if input_copy[i][j] == "@":
                start_row = 0
                end_row = 0
                start_col = 0
                end_col = 0

                # Assign row indices for sub-matrices
                if i == 0:
                    start_row = i
                else:
                    start_row = i - 1

                if i < row_len - 2:
                    end_row = i + 2
                else:
                    end_row = None

                # Assign column indices for sub-matrices
                if j == 0:
                    start_col = j
                else:
                    start_col = j - 1

                if j < col_len - 2:
                    end_col = j + 2
                else:
                    end_col = None

                sub_matrix = input_copy[start_row:end_row, start_col:end_col]

                # Count '@' and 'X' representing rolls
                surrounding_rolls = np.count_nonzero(sub_matrix == "@") - 1
                surrounding_rolls += np.count_nonzero(sub_matrix == "X")

                if surrounding_rolls < 4:
                    eligible_rolls += 1
                    input_copy[i][j] = np.str_("X")

    return (input_copy, eligible_rolls)


def clear_rolls(input_array: np.array) -> np.array:
    """
    Modifies an input array by removing the 'X' representing removed paper rolls (as in the output of the 'find_rolls' function).

    Parameters:
    input_array (numpy.array): An array showing paper roll locations with 'X' representing previously removed rolls.

    Returns:
    numpy.array: The input array with any 'X' symbols removed.

    """
    input_copy = copy.deepcopy(input_array)
    row_len, col_len = input_copy.shape

    for i in range(row_len):
        for j in range(col_len):
            if input_copy[i][j] == "X":
                input_copy[i][j] = "."

    return input_copy


answer = 0
with open(sys.argv[1], "r") as input:
    line_array = np.array(list(list(line.strip()) for line in input))
    next_array, eligible = find_rolls(line_array)

    if len(sys.argv) > 2 and sys.argv[2] == "part2":
        while eligible:
            answer += eligible
            next_array = clear_rolls(next_array)
            next_array, eligible = find_rolls(next_array)

    answer += eligible

for line in next_array:
    print("".join(line))
print(f"\n{answer=}")
