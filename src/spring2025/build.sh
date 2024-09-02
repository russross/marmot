#!/bin/bash

set -e

echo deleting old database
rm -f timetable.db

echo building schema
sqlite3 timetable.db < schema.sql

echo building term and holidays
./edit make-term 'Spring 2025' 2025-01-06 2025-04-24
./edit make-holiday 2025-01-20
./edit make-holiday 2025-02-17
./edit make-holiday 2025-03-10
./edit make-holiday 2025-03-11
./edit make-holiday 2025-03-12
./edit make-holiday 2025-03-13
./edit make-holiday 2025-03-14

source courses.sh
source computing.sh
source math.sh

sqlite3 timetable.db vacuum
sqlite3 timetable.db analyze
