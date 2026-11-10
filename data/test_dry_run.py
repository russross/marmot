import csv
import sqlite3
import tempfile
import unittest
from pathlib import Path

from posted_schedule import PostedMeeting, supplemental_offerings
from queries import DB
from rooms import build_rooms, science_lecture_tag
from timeslots import build_time_slots


class SchedulingInputTests(unittest.TestCase):
    def setUp(self) -> None:
        self.directory = tempfile.TemporaryDirectory()
        path = Path(self.directory.name) / 'test.db'
        connection = sqlite3.connect(path)
        try:
            connection.executescript((Path(__file__).parent / 'schema.sql').read_text())
        finally:
            connection.close()
        self.db = DB(str(path))
        self.db.make_department('Test')
        self.db.make_course('Test', 'BIO 1000', 'Example', 3, 3)
        self.db.make_course('Test', 'BIO 2000', 'Other credits', 1, 1)

    def tearDown(self) -> None:
        self.db.db.close()
        self.directory.cleanup()

    def test_supplemental_meetings_follow_shared_lecture(self) -> None:
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('BIO 1000-02')
        self.db.add_cross_listing('BIO 1000-01', ['BIO 1000-02'])
        self.db.make_section_with_no_faculty('BIO 1000-02si', 'T0900+50')
        self.db.add_supplemental_instruction('BIO 1000-02', 'BIO 1000-02si')

        pair = self.db.db.execute(
            'SELECT priority FROM conflict_pairs WHERE section_a = ? AND section_b = ?',
            ('BIO 1000-01', 'BIO 1000-02si'),
        ).fetchone()
        self.assertEqual(pair, (0,))
        self.assertIsNone(self.db.db.execute("SELECT section_count FROM section_counts WHERE course = 'BIO 1000'").fetchone())

    def test_shared_meeting_uses_primary_credits_for_solver(self) -> None:
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('BIO 2000-01')
        self.db.add_cross_listing('BIO 1000-01', ['BIO 2000-01'])
        source = self.db.db.execute("SELECT credit_hours FROM sections WHERE section = 'BIO 2000-01'").fetchone()
        exposed = self.db.db.execute(
            "SELECT DISTINCT credit_hours FROM time_slots_available_to_sections WHERE section = 'BIO 1000-01'"
        ).fetchall()
        self.assertEqual(source, (1,))
        self.assertEqual(exposed, [(3,)])

    def test_supplemental_classification_uses_weekly_credit_minutes(self) -> None:
        path = Path(self.directory.name) / 'input.csv'
        fields = ['CRN', 'Subject', 'Course', 'Section', 'Credits', 'Days', 'Time', 'Location', 'Instructor', 'Attributes', 'Cap']
        with path.open('w', newline='') as file:
            writer = csv.DictWriter(file, fieldnames=fields)
            writer.writeheader()
            common = {'CRN': '1', 'Subject': 'BIO', 'Course': '1000', 'Section': '01', 'Credits': '3',
                      'Location': 'SET 1', 'Instructor': 'Example', 'Attributes': 'SI', 'Cap': '30'}
            writer.writerow(common | {'Days': 'W', 'Time': '9:00 am-9:50 am'})
            writer.writerow(common | {'Days': 'TR', 'Time': '9:00 am-10:15 am'})
        offerings = supplemental_offerings(Path(self.directory.name))
        self.assertEqual(len(offerings), 1)
        self.assertEqual(offerings[0].primary.days, 'TR')
        self.assertEqual(offerings[0].supplemental.days, 'W')

    def test_science_lecture_rooms_preserve_capacity_and_floated_space(self) -> None:
        build_rooms(self.db)
        medium = science_lecture_tag('SET 105')
        floated = science_lecture_tag('Brown 201')
        assert medium is not None and floated is not None
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50', medium)
        self.db.make_section_with_no_faculty('BIO 1000-02', 'MWF0900+50', floated)
        rooms = {
            section: {room for (room,) in self.db.db.execute(
                'SELECT DISTINCT room FROM rooms_available_to_sections WHERE section = ?', (section,),
            )}
            for section in ('BIO 1000-01', 'BIO 1000-02')
        }
        self.assertEqual(rooms['BIO 1000-01'], {'SET 105', 'SET 106', 'SET 201', 'SET 301'})
        self.assertEqual(rooms['BIO 1000-02'], {'Brown 201', 'SET 106', 'SET 201', 'SET 301'})

    def test_labs_move_within_staggered_same_day_grid(self) -> None:
        build_time_slots(self.db)
        meeting = PostedMeeting('TR', '10:00 am-11:50 am', 'SET 213', 'Example')
        self.db.make_section_with_no_faculty('BIO 1000-01', *meeting.lab_time_tags())
        slots = {row[0] for row in self.db.db.execute(
            "SELECT time_slot FROM time_slots_available_to_sections WHERE section = 'BIO 1000-01'"
        )}
        self.assertEqual(slots, {'TR1000+110', 'TR1200+110', 'TR1400+110'})
        evening = PostedMeeting('T', '6:00 pm-7:50 pm', 'SET 213', 'Example')
        self.assertEqual(evening.lab_time_tags(), ['T1800+110'])


if __name__ == '__main__':
    unittest.main()
