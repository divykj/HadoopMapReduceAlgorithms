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
    for current_item, group in groupby(data, itemgetter(0)):
        try:
            ratings = list(int(rating) for _, rating in group)
            num_ratings = len(ratings)
            avg_rating = sum(ratings) / num_ratings
            sys.stdout.write(
                f"{current_item}{separator}{num_ratings}{separator}{avg_rating}\n"
            )
        except ValueError:
            pass


if __name__ == "__main__":
    main()
