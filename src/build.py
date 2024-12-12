#!/usr/bin/env python3

import queries
import os
import subprocess

DB_FILE = 'timetable.db'

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
db.db.commit()

subprocess.run(['./courses.py'], check=True)
subprocess.run(['./computing.py'], check=True)
subprocess.run(['./cset.py'], check=True)

print('running vacuum and analyze')
subprocess.run(['sqlite3', DB_FILE, 'vacuum'], check=True)
subprocess.run(['sqlite3', DB_FILE, 'analyze'], check=True)
