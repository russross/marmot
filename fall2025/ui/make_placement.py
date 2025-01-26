#!/usr/bin/env python3

# TODO: cross listings

import json
import os
import sys
import sqlite3

DB_FILE = '../timetable.db'

if len(sys.argv) != 2:
    print(f'Usage: {sys.argv[0]} <placement_id>', file=sys.stderr)
    sys.exit(1)

id = int(sys.argv[1])

db = sqlite3.connect(DB_FILE)
db.execute('PRAGMA busy_timeout = 10000')
db.execute('PRAGMA temp_store = MEMORY')
db.execute('PRAGMA mmap_size = 100000000')

rows = db.execute('SELECT score, comment FROM placements WHERE placement_id = ?', (id,)).fetchall()
for row in rows:
    print(f'{row[0]} {row[1]}', file=sys.stderr)

rows = db.execute('SELECT section, time_slot, room FROM placement_sections WHERE placement_id = ?', (id,)).fetchall()

sections = {}
for (section, time_slot, room) in rows:
    if room is None:
        print(f'skipping placement with no room: {section}', file=sys.stderr)
        continue
    elt = {
        'names': [section],
        'prefixes': [section[:section.index(' ')]],
        'instructors': [],
        'is_placed': True,
        'room': room,
        'time_slot': time_slot,
        'problems': [],
    }
    sections[section] = elt

rows = db.execute('SELECT section, priority, message FROM placement_penalties NATURAL JOIN placement_penalty_sections WHERE placement_id = ?', (id,)).fetchall()

for (section, priority, message) in rows:
    if section not in sections:
        continue
    elt = sections[section]
    elt['problems'].append({ 'score': priority, 'message': message })

rows = db.execute('SELECT section, faculty FROM placement_sections NATURAL JOIN faculty_sections WHERE placement_id = ?', (id,)).fetchall()

for (section, faculty) in rows:
    if section not in sections:
        continue
    elt = sections[section]
    elt['instructors'].append(faculty)

as_list = [sections[key] for key in sorted(sections.keys())]

print('window.placement = ', end='')
json.dump(as_list, sys.stdout, indent=4)
print(';')
