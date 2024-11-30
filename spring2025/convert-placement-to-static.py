#!/usr/bin/env python3

import json
import sys

def main():
    if len(sys.argv) == 1:
        infile = 'static.js'
    elif len(sys.argv) == 2:
        infile = sys.argv[1]
    else:
        print(f'Usage: {sys.argv[0]} [inputfile]', file=sys.stderr)
        sys.exit(1)

    with open(infile) as fp:
        raw = fp.read()
    prefix = 'window.placement = '
    postfix = ';\n'
    if raw.startswith(prefix) and raw.endswith(postfix):
        raw = raw[len(prefix):-len(postfix)] + '\n'
    else:
        print(f'{infile} does not start with {repr(prefix)} and end with {repr(postfix)} as expected')
        sys.exit(1)

    lst = json.loads(raw)
    for elt in lst:
        (section, room, ts) = (elt['names'][0], elt['room'], elt['time_slot'])
        print(f'    place(solver, "{section}", "{room}", "{ts}")?;')

main()
