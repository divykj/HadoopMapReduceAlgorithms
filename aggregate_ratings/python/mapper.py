#!/usr/bin/env python
"""mapper.py"""

import sys


def main(seperator="\t"):
    key = sys.stdin.readline().strip()[:-1]
    for line in sys.stdin:
        line = line.strip()
        if line[-1] == ":":
            key = line[:-1]
            continue
        _, rating, _ = line.split(",")
        sys.stdout.write(f"{key}{seperator}{rating}\n")


if __name__ == "__main__":
    main()
