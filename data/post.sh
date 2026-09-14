#!/bin/sh

set -e

cp computingfaculty.py timetable.db ~/courses/ross/schedule/
../ui/make_placement.py 1 > ~/courses/ross/schedule/placement.js
exec push
