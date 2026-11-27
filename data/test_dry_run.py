import csv
import sqlite3
import tempfile
import unittest
from pathlib import Path

from posted_schedule import PostedMeeting, supplemental_offerings
from queries import AvoidSectionInRooms, AvoidTimeSlot, DB, TimeInterval, UnavailableTimeSlot
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

    def test_advisory_fields_do_not_control_placement(self) -> None:
        self.db.make_course_format('INV', workload_multiplier=0.75, meeting_time_multiplier=0, has_scheduled_meetings=False)
        self.db.make_course('Test', 'BIO 3000', 'Directed study', 1, 3, course_format='INV')
        self.db.make_faculty('A', 'Test', [], faculty_id='00001234', phone='ext. 24', maximum_workload_credits=0)
        self.db.make_faculty_section('A', 'BIO 3000-01', 'M0900+50', credit_hours=3, workload_factor=2.25,
                                     seats_capacity=0, waitlist_capacity=7, campus='A01')
        self.db.make_section_with_no_faculty('BIO 3000-02', 'M0900+50')
        self.db.make_section_with_no_faculty('BIO 1000-01', 'M0900+50', credit_hours=None)

        # Editing the course default leaves the saved workload and placement choices intact.
        sql = self.db.db
        sql.execute("UPDATE course_formats SET workload_multiplier = 1 WHERE course_format = 'INV'")
        self.assertEqual(sql.execute('SELECT faculty_id, phone, minimum_workload_credits, maximum_workload_credits FROM faculty').fetchone(),
                         ('00001234', 'ext. 24', None, 0))
        self.assertEqual(sql.execute("SELECT workload_factor, seats_capacity, waitlist_capacity, campus FROM sections WHERE section = 'BIO 3000-01'").fetchone(),
                         (2.25, 0, 7, 'A01'))
        self.assertEqual(sql.execute('SELECT section, credit_hours FROM time_slots_available_to_sections ORDER BY section').fetchall(),
                         [('BIO 1000-01', None), ('BIO 3000-01', 3), ('BIO 3000-02', None)])
        for invalid_id in ('1234567', '123456789', '1234567A'):
            with self.assertRaises(sqlite3.IntegrityError):
                sql.execute('UPDATE faculty SET faculty_id = ?', (invalid_id,))
        with self.assertRaises(sqlite3.IntegrityError):
            sql.execute("UPDATE courses SET course_format = 'unknown'")

    def test_last_five_minutes_of_the_day(self) -> None:
        self.db.make_faculty('Night', 'Test', [TimeInterval('M', '2355', '2400')])
        for slot in ('M2350+5', 'M2355+5', 'M2350+10', 'T0000+5'):
            self.db.make_time_slot(slot, [])
        available = self.db.db.execute('SELECT time_slot FROM time_slots_available_to_faculty').fetchall()
        self.assertEqual(available, [('M2355+5',)])
        conflicts = self.db.db.execute(
            "SELECT time_slot_b FROM conflicting_time_slots WHERE time_slot_a = 'M2355+5' ORDER BY time_slot_b"
        ).fetchall()
        self.assertEqual(conflicts, [('M2350+10',), ('M2355+5',)])

        # Adjacent ranges cover the whole day, including the final cell.
        self.db.make_faculty('All day', 'Test', [TimeInterval('M', 0, 1435), TimeInterval('M', 1435, 1440)])
        self.assertEqual(self.db.db.execute(
            "SELECT readable, duration FROM faculty_availability WHERE faculty = 'All day'"
        ).fetchone(), ('M0000-2400', 1440))
        for start, end in [('2400', '2405'), ('2355', '2355'), ('2360', '2400'), ('2356', '2400')]:
            with self.assertRaises(ValueError):
                TimeInterval('M', start, end)
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.db.execute("INSERT INTO time_slots VALUES ('M2400+5')")

    def test_edit_ordered_requests(self) -> None:
        self.db.make_building('Hall')
        self.db.make_room('Hall 1', 20, ['lab'])
        self.db.make_time_slot('M0900+50', [])
        self.db.make_time_slot('M1000+50', [])
        self.db.make_faculty('Owner', 'Test', [TimeInterval('M', '0900', '1100')])
        self.db.make_faculty_section('Owner', 'BIO 1000-01', 'M0900+50', 'lab')
        self.db.faculty_preferences('Owner', 'MT',
            AvoidSectionInRooms('BIO 1000-01', ['lab', 'Hall 1']),
            UnavailableTimeSlot('M1000+50'),
            AvoidTimeSlot('M0900+50'), AvoidTimeSlot('M0900+50'),
        )
        sql = self.db.db
        request_id = sql.execute("SELECT request_id FROM faculty_preference_requests WHERE position = 1").fetchone()[0]
        self.assertEqual(sql.execute(
            'SELECT room_tag FROM preference_room_tags WHERE request_id = ? ORDER BY room_tag', (request_id,)
        ).fetchall(), [('Hall 1',), ('lab',)])

        # Move a multi-target request past both soft time requests. Gaps in
        # positions do not affect the consecutive ranks.
        sql.execute('UPDATE faculty_preference_requests SET position = 200 WHERE request_id = ?', (request_id,))
        self.assertEqual(sql.execute(
            'SELECT position, priority FROM ranked_faculty_requests ORDER BY position'
        ).fetchall(), [(2, 10), (3, 11), (200, 12)])
        self.assertEqual(sql.execute(
            'SELECT preference_priority FROM faculty_time_preferences_to_be_scheduled ORDER BY preference_priority'
        ).fetchall(), [(10,), (11,)])
        self.assertEqual(sql.execute(
            'SELECT preference_priority FROM faculty_room_preferences_to_be_scheduled'
        ).fetchall(), [(12,)])

        # Reject a tied position. Removing the first avoidance leaves the
        # second one effective, and its rank is recomputed on the next read.
        with self.assertRaises(sqlite3.IntegrityError):
            sql.execute('UPDATE faculty_preference_requests SET position = 2 WHERE position = 3')
        sql.execute('DELETE FROM faculty_preference_requests WHERE position = 2')
        self.assertEqual(sql.execute('SELECT time_slot_priority FROM faculty_time_slot_preferences').fetchall(), [(10,)])

    def test_import_subtracts_overlapping_minutes(self) -> None:
        self.db.make_faculty('Owner', 'Test', [TimeInterval('MTW', '2300', '2400')])
        for slot in ('MW2330+30', 'M2325+10', 'M2300+30', 'T2330+30'):
            self.db.make_time_slot(slot, ['late'])
        self.db.make_faculty_section('Owner', 'BIO 1000-01', 'late')
        self.db.make_faculty_section('Owner', 'BIO 1000-02', 'M2325+10')
        self.db.faculty_preferences('Owner', 'MT', UnavailableTimeSlot('MW2330+30'))

        # Monday and Wednesday lose their final half hour; Tuesday is untouched.
        sql = self.db.db
        self.assertEqual(sql.execute('SELECT readable FROM faculty_availability ORDER BY day_of_week').fetchall(),
                         [('M2300-2330',), ('T2300-2400',), ('W2300-2330',)])
        self.assertEqual(sql.execute('SELECT COUNT(*) FROM faculty_preference_requests').fetchone(), (0,))
        self.assertEqual(sql.execute(
            'SELECT section, time_slot FROM time_slots_available_to_sections ORDER BY section, time_slot'
        ).fetchall(), [('BIO 1000-01', 'M2300+30'), ('BIO 1000-01', 'T2330+30'), ('BIO 1000-02', 'M2325+10')])

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

    def check_cross_listing_prerequisites(self, left: str, right: str) -> None:
        # The alias belongs to a different curricular sequence from the
        # representative. Every rule shape must filter using that alias.
        self.db.make_course('Test', 'BIO 3000', 'Earlier course', 3, 3)
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('BIO 2000-01')
        self.db.make_section_with_no_faculty('BIO 3000-01', 'TR0900+75')
        self.db.add_cross_listing('BIO 1000-01', ['BIO 2000-01'])
        self.db.add_prereqs('BIO 2000', ['BIO 3000'])
        self.db.make_program('Alias curriculum', 'Test')
        self.db.make_conflict('Alias curriculum', 'requirements', 1, 'boost', [left, right])
        self.assertEqual(self.db.db.execute('SELECT * FROM conflict_pairs').fetchall(), [])

        # A separate curriculum can still require the representative and the
        # earlier course together. The alias's prerequisite must not hide it.
        self.db.make_program('Other curriculum', 'Test')
        self.db.make_conflict('Other curriculum', 'requirements', 3, 'boost', ['BIO 1000', 'BIO 3000'])
        self.assertEqual(self.db.db.execute(
            'SELECT section_a, section_b, priority FROM conflict_pairs WHERE section_a < section_b'
        ).fetchall(), [('BIO 1000-01', 'BIO 3000-01', 3)])

    def test_cross_listing_course_course_prerequisites(self) -> None:
        self.check_cross_listing_prerequisites('BIO 2000', 'BIO 3000')

    def test_cross_listing_section_section_prerequisites(self) -> None:
        self.check_cross_listing_prerequisites('BIO 2000-01', 'BIO 3000-01')

    def test_cross_listing_section_course_prerequisites(self) -> None:
        self.check_cross_listing_prerequisites('BIO 2000-01', 'BIO 3000')

    def test_cross_listing_course_section_prerequisites(self) -> None:
        self.check_cross_listing_prerequisites('BIO 2000', 'BIO 3000-01')

    def test_cross_listing_reduction_does_not_cancel_another_listing(self) -> None:
        self.db.make_course('Test', 'BIO 3000', 'Shared requirement', 3, 3)
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('BIO 2000-01')
        self.db.make_section_with_no_faculty('BIO 3000-01', 'TR0900+75')
        self.db.add_cross_listing('BIO 1000-01', ['BIO 2000-01'])
        self.db.make_program('Test program', 'Test')
        self.db.make_conflict('Test program', 'requirements', 1, 'boost', ['BIO 1000', 'BIO 2000', 'BIO 3000'])
        self.db.make_conflict('Test program', 'alternatives', None, 'reduce', ['BIO 2000', 'BIO 3000'])

        # Canceling the alias relationship leaves the representative's rule.
        # The two listings of the shared meeting never create a self-conflict.
        self.assertEqual(self.db.db.execute(
            'SELECT section_a, section_b, priority FROM conflict_pairs WHERE section_a <= section_b'
        ).fetchall(), [('BIO 1000-01', 'BIO 3000-01', 1)])

    def test_cross_listing_discount_uses_original_course_choices(self) -> None:
        self.db.make_department('Other')
        self.db.make_course('Other', 'ALT 3000', 'Alias', 3, 3)
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('ALT 3000-01')
        self.db.make_section_with_no_faculty('ALT 3000-02')
        self.db.make_section_with_no_faculty('BIO 2000-01', 'TR0900+75')
        self.db.add_cross_listing('BIO 1000-01', ['ALT 3000-01'])
        self.db.make_program('Test program', 'Test')
        self.db.make_conflict('Test program', 'alias choices', 1, 'boost', ['ALT 3000', 'BIO 2000'])
        self.db.make_conflict('Test program', 'primary requirements', 8, 'boost', ['BIO 1000', 'BIO 2000'])

        # The alias has an untimed alternative, so its priority becomes six.
        # Both listing rules merge into one pair in the primary department.
        self.assertEqual(self.db.db.execute(
            'SELECT * FROM conflict_pairs WHERE section_a < section_b'
        ).fetchall(), [('Test', 'BIO 1000-01', 'Test', 'BIO 2000-01', 6)])

    def test_cross_listing_conflicts_do_not_depend_on_representative(self) -> None:
        self.db.make_course('Test', 'BIO 3000', 'Earlier course', 3, 3)
        self.db.make_section_with_no_faculty('BIO 1000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('BIO 2000-01', 'MWF0900+50')
        self.db.make_section_with_no_faculty('BIO 3000-01', 'TR0900+75')
        self.db.add_prereqs('BIO 2000', ['BIO 3000'])
        self.db.make_program('Test program', 'Test')
        self.db.make_conflict('Test program', 'requirements', 1, 'boost', ['BIO 2000', 'BIO 3000'])

        # Either listing can own the physical meeting. Delete only the alias's
        # time tags before attaching it, as required by the cross-listing API.
        for primary, secondary in [('BIO 1000-01', 'BIO 2000-01'), ('BIO 2000-01', 'BIO 1000-01')]:
            with self.subTest(primary=primary):
                self.db.db.execute('SAVEPOINT representative')
                self.db.db.execute('DELETE FROM section_time_slot_tags WHERE section = ?', (secondary,))
                self.db.add_cross_listing(primary, [secondary])
                self.assertEqual(self.db.db.execute('SELECT * FROM conflict_pairs').fetchall(), [])
                self.db.db.execute('ROLLBACK TO representative')
                self.db.db.execute('RELEASE representative')

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
