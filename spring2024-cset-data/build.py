#!/usr/bin/env python3

import queries
import os
import subprocess

import courses
import computing
import cset

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
db.make_term('Spring 2024', '2024-01-08', '2024-04-25')
db.make_holiday('2024-01-15')
db.make_holiday('2024-02-19')
db.make_holiday('2024-03-11')
db.make_holiday('2024-03-12')
db.make_holiday('2024-03-13')
db.make_holiday('2024-03-14')
db.make_holiday('2024-03-15')

courses.build(db)
computing.build(db)
cset.build(db)

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
