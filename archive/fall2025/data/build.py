#!/usr/bin/env python3

import queries
import os
import subprocess

import courses
import computing

DB_FILE = '../timetable.db'

print('deleting old database')
try:
    os.remove(DB_FILE)
except FileNotFoundError:
    pass

print('building schema')
subprocess.run(['sqlite3', DB_FILE], stdin=open('schema.sql'), check=True)

print('building term and holidays')
db = queries.DB(DB_FILE)
db.make_term('Fall 2025', '2025-08-18', '2025-12-05')
db.make_holiday('2025-09-01')
db.make_holiday('2025-10-10')
db.make_holiday('2025-10-13')
db.make_holiday('2025-11-26')
db.make_holiday('2025-11-27')
db.make_holiday('2025-11-28')

courses.build(db)
computing.build(db)

db.db.commit()


print('materializing views')
db.db.execute('ANALYZE')
db.db.execute('INSERT INTO conflict_pairs_materialized SELECT * FROM conflict_pairs')
db.db.execute('INSERT INTO anti_conflict_pairs_materialized SELECT * FROM anti_conflict_pairs')
db.db.execute('INSERT INTO time_slots_used_by_departments_materialized SELECT * FROM time_slots_used_by_departments')
db.db.execute('INSERT INTO time_slots_available_to_sections_materialized SELECT * FROM time_slots_available_to_sections')
db.db.commit()

db.db.execute('VACUUM')
db.db.execute('ANALYZE')

subprocess.run(['rm', '-r', '__pycache__'], check=True)
