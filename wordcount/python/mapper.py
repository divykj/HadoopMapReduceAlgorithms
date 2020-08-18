#!/usr/bin/env python
"""mapper.py"""

import sys
import string

punctuation_to_space = str.maketrans(string.punctuation, " " * len(string.punctuation))


def preprocess(text):
    return text.translate(punctuation_to_space).lower().strip().split()


def read_input(input_file):
    for line in input_file:
        yield preprocess(line)


def main(seperator="\t"):
    for words in read_input(sys.stdin):
        for word in words:
            sys.stdout.write(f"{word}{seperator}1\n")


if __name__ == "__main__":
    main()
