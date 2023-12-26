#!/usr/bin/python3

#neato -Tsvg example.gv > example.svg

import sys
import os

fname = sys.argv[1]
no_overlap = False
if len(sys.argv) >= 3:
	no_overlap = sys.argv[2].lower() == "true"


statements = [] # "overlap=scale"]
if no_overlap:
	statements.append("overlap=scale")

with open(fname) as f:
    lines = f.read().strip().split("\n")
print(lines)
for line in lines:
    frm, to_ = line.split(": ")
    to = to_.split(" ")
    for t in to:
        statements.append(f"{frm} -- {t}")

with open("/tmp/aoc25.gv", "w") as f:
    f.write("strict graph aoc25 {\n")
    f.write(";\n".join(statements))
    f.write("\n}")

os.system(f"neato -Tsvg /tmp/aoc25.gv > \"{fname+'.svg'}\"")
