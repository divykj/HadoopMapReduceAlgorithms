#!/usr/bin/env python
"""mapper.py"""

import sys
import re
import string


def preprocess(text):
    text = text.translate(str.maketrans("", "", string.punctuation)).lower().strip()
    tokens = re.split(r"\W+", text)
    return tokens


def read_input(input_file):
    for line in input_file:
        yield preprocess(line)


def main(seperator="\t"):
    for words in read_input(sys.stdin):
        for word in words:
            sys.stdout.write(f"{word}{seperator}1\n")


if __name__ == "__main__":
    main()
