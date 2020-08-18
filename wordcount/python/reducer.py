#!/usr/bin/env python
"""reducer.py"""

from itertools import groupby
from operator import itemgetter
import sys


def read_mapper_output(input_file, separator="\t"):
    for line in input_file:
        yield line.rstrip().split(separator, 1)


def main(separator="\t"):
    data = read_mapper_output(sys.stdin, separator=separator)
    for current_word, group in groupby(data, itemgetter(0)):
        try:
            total_count = sum(int(count) for _, count in group)
            sys.stdout.write(f"{current_word}{separator}{total_count}\n")
        except ValueError:
            pass


if __name__ == "__main__":
    main()
