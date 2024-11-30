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
    by_faculty = {}
    for elt in lst:
        section = elt['names'][0].replace(' ', '')
        index = section.index('-')
        section = section[:index]
        room = elt['room'].replace('Smith ', '')
        ts = elt['time_slot']
        index = ts.index('+')
        ts = ts[:index]
        for f in elt['instructors']:
            faculty = f.replace(' ', '.')
            if faculty not in by_faculty:
                by_faculty[faculty] = []
            by_faculty[faculty].append([section, room, ts])
    print('{')
    for (j, faculty) in enumerate(sorted(by_faculty.keys())):
        print(f'    "{faculty}": [')
        lst = by_faculty[faculty]
        for (i, elt) in enumerate(lst):
            print(f'        ["{elt[0]}", "{elt[1]}", "{elt[2]}"]{"," if i < len(lst)-1 else ""}')
        print(f'    ]{"," if j < len(by_faculty)-1 else ""}')
    print('}')

main()
