#!/usr/bin/env python3
import csv
from os import path
import sys

def main():
    if len(sys.argv) >= 2:
        filename = sys.argv[1]
    else:
        filename = path.join(path.dirname(__file__),
                path.join("veekun", "moves.csv"))
    reverse = {}
    unique = {}
    with open(filename, newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        for row in reader:
            identifier = row["identifier"]
            effect_id = int(row["effect_id"])
            if effect_id not in reverse:
                reverse[effect_id] = {identifier}
                unique[effect_id] = identifier
            else:
                reverse[effect_id].add(identifier)
                if effect_id in unique:
                    del unique[effect_id]
    prev_effect_id = 0
    for effect_id, identifier in sorted(unique.items()):
        if effect_id != prev_effect_id + 1:
            print()
        print("{: 3d}: {}".format(effect_id, identifier))
        prev_effect_id = effect_id

if __name__ == "__main__":
    main()
