import queries
from queries import *


def build_faculty(db: DB) -> None:
    print('building math faculty and sections')
    default_availability = [TimeInterval('MTWR', '0900', '1640'), TimeInterval('F', '0900', '1200')]
    default_prefs = [
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    ]
    reduced_default_prefs = [
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    ]

    db.make_section_with_no_faculty('MATH 0900-02', '4 credit bell schedule', 'snow math rooms')  # no instructor assigned; created with room/time constraints only
    db.make_section_with_no_faculty('MATH 1080-01', 'MTWRF0800+50', 'snow math rooms')  # no instructor assigned; created with room/time constraints only

    db.make_faculty('Jameson C Hardy', 'Mathematics', default_availability)
    db.make_faculty_section('Jameson C Hardy', 'MATH 0900-01', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jameson C Hardy', 'MATH 1040-06', '3 credit bell schedule', 'snow math rooms')  # SI omitted: T 1200+50 SNOW 125
    db.make_faculty_section('Jameson C Hardy', 'MATH 1040-07', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jameson C Hardy', 'MATH 1040-14', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jameson C Hardy', 'MATH 1060-05', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Jameson C Hardy', 'MT',
        *default_prefs,
    )

    db.make_faculty('Neil James Duncan', 'Mathematics', default_availability)
    db.make_faculty_section('Neil James Duncan', 'MATH 0900-03', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Neil James Duncan', 'MATH 1010-01', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Neil James Duncan', 'MATH 1010-40')
    db.make_faculty_section('Neil James Duncan', 'MATH 1040-05', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Neil James Duncan', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Scott Patrick Hicks', 'Mathematics', default_availability)
    db.make_faculty_section('Scott Patrick Hicks', 'MATH 0900-04', 'MW1630+100', 'snow math rooms')
    db.make_faculty_section('Scott Patrick Hicks', 'MATH 0900-41')

    db.make_faculty('Paula Manuele Temple', 'Mathematics', default_availability)
    db.make_faculty_section('Paula Manuele Temple', 'MATH 0900-05', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Paula Manuele Temple', 'MATH 0900-40')
    db.make_faculty_section('Paula Manuele Temple', 'MATH 0980-04', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Paula Manuele Temple', 'MATH 0980-05', '4 credit bell schedule', 'snow math rooms')  # SI omitted: W 1330+50 SNOW 147
    db.faculty_preferences('Paula Manuele Temple', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Craig D Seegmiller', 'Mathematics', default_availability)
    db.make_faculty_section('Craig D Seegmiller', 'MATH 0900-06', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Craig D Seegmiller', 'MATH 1030-01', 'TR0730+75', 'snow math rooms')
    db.make_faculty_section('Craig D Seegmiller', 'MATH 1030-04', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Craig D Seegmiller', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Matthew S Smith', 'Mathematics', default_availability)
    db.make_faculty_section('Matthew S Smith', 'MATH 0900-50', 'MW1800+100', 'snow math rooms')
    db.make_faculty_section('Matthew S Smith', 'MATH 1060-04', 'MW1630+75', 'snow math rooms')

    db.make_faculty('Buna Sambandham', 'Mathematics', default_availability)
    db.make_faculty_section('Buna Sambandham', 'MATH 0980-02', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Buna Sambandham', 'MATH 0980-40')
    db.make_faculty_section('Buna Sambandham', 'MATH 1210-02', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Buna Sambandham', 'MATH 3500-01', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Buna Sambandham', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Kathie E Ott', 'Mathematics', default_availability)
    db.make_faculty_section('Kathie E Ott', 'MATH 0980-06', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Kathie E Ott', 'MATH 0980-08', '4 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Kathie E Ott', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Odean Bowler', 'Mathematics', default_availability)
    db.make_faculty_section('Odean Bowler', 'MATH 0980-07', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Odean Bowler', 'MATH 1010-04', '4 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Odean Bowler', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Michael N Paxman', 'Mathematics', default_availability)
    db.make_faculty_section('Michael N Paxman', 'MATH 0980-09', 'TR1630+100', 'snow math rooms')

    db.make_faculty('John R Weber', 'Mathematics', default_availability)
    db.make_faculty_section('John R Weber', 'MATH 0980-10', 'MTWR0800+50', 'snow math rooms')
    db.make_faculty_section('John R Weber', 'MATH 1040-17', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('John R Weber', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Elizabeth Karen Ludlow', 'Mathematics', default_availability)
    db.make_faculty_section('Elizabeth Karen Ludlow', 'MATH 0980-41')
    db.make_faculty_section('Elizabeth Karen Ludlow', 'MATH 1010-06', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Elizabeth Karen Ludlow', 'MATH 1010-41')
    db.make_faculty_section('Elizabeth Karen Ludlow', 'MATH 1050-03', '4 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Elizabeth Karen Ludlow', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Adina Ionita', 'Mathematics', default_availability)
    db.make_faculty_section('Adina Ionita', 'MATH 1010-02', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Adina Ionita', 'MATH 1010-03', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Adina Ionita', 'MATH 1050-01', 'MTWR0800+50', 'snow math rooms')
    db.make_faculty_section('Adina Ionita', 'MATH 1050-02', '4 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Adina Ionita', 'MT',
        *default_prefs,
    )

    db.make_faculty('Shelly Lashell Kidd-Thomas', 'Mathematics', default_availability)
    db.make_faculty_section('Shelly Lashell Kidd-Thomas', 'MATH 1010-07', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Shelly Lashell Kidd-Thomas', 'MATH 1010-08', '4 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Shelly Lashell Kidd-Thomas', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty("Amanda Fa'onelua", 'Mathematics', default_availability)
    db.make_faculty_section("Amanda Fa'onelua", 'MATH 1030-02', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section("Amanda Fa'onelua", 'MATH 1030-03', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences("Amanda Fa'onelua", 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Jeff P Harrah', 'Mathematics', default_availability)
    db.make_faculty_section('Jeff P Harrah', 'MATH 1030-05', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jeff P Harrah', 'MATH 1030-06', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jeff P Harrah', 'MATH 2010-01', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jeff P Harrah', 'MATH 2010-02', 'T1630+150', 'snow math rooms')
    db.make_faculty_section('Jeff P Harrah', 'MATH 2020-01', 'W1630+150', 'snow math rooms')
    db.make_faculty_section('Jeff P Harrah', 'MATH 4500-01', 'R1630+150', 'snow math rooms')
    db.faculty_preferences('Jeff P Harrah', 'MT',
        *default_prefs,
    )

    db.make_faculty('Md Sazib Hasan', 'Mathematics', default_availability)
    db.make_faculty_section('Md Sazib Hasan', 'MATH 1030-40')
    db.make_faculty_section('Md Sazib Hasan', 'MATH 1050-05', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Md Sazib Hasan', 'MATH 2050-01', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Md Sazib Hasan', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('James P Fitzgerald', 'Mathematics', default_availability)
    db.make_faculty_section('James P Fitzgerald', 'MATH 1040-01', 'MWF0800+50', 'snow math rooms')
    db.make_faculty_section('James P Fitzgerald', 'MATH 1040-02', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('James P Fitzgerald', 'MATH 1040-03', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('James P Fitzgerald', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Trevor K Johnson', 'Mathematics', default_availability)
    db.make_faculty_section('Trevor K Johnson', 'MATH 1040-04', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Trevor K Johnson', 'MATH 1040-08', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Trevor K Johnson', 'MATH 1100-40')
    db.faculty_preferences('Trevor K Johnson', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Robert T Reimer', 'Mathematics', default_availability)
    db.make_faculty_section('Robert T Reimer', 'MATH 1040-15', 'TR1630+75', 'snow math rooms')

    db.make_faculty('Ryan C McConnell', 'Mathematics', default_availability)
    db.make_faculty_section('Ryan C McConnell', 'MATH 1040-16', 'MW1630+75', 'snow math rooms')

    db.make_faculty('Jie Liu', 'Mathematics', default_availability)
    db.make_faculty_section('Jie Liu', 'MATH 1040-40')
    db.make_faculty_section('Jie Liu', 'MATH 1040-41')
    db.make_faculty_section('Jie Liu', 'MATH 1210-03', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jie Liu', 'MATH 3400-01', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Jie Liu', 'MATH 3410-01', 'W1200+50', 'snow math rooms')
    db.faculty_preferences('Jie Liu', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Clare C Banks', 'Mathematics', default_availability)
    db.make_faculty_section('Clare C Banks', 'MATH 1050-04', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Clare C Banks', 'MATH 1050-40')
    db.make_faculty_section('Clare C Banks', 'MATH 1220-01', 'MTWR0800+50', 'snow math rooms')
    db.make_faculty_section('Clare C Banks', 'MATH 3010-40')
    db.faculty_preferences('Clare C Banks', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Kevin Gregory Johnston', 'Mathematics', default_availability)
    db.make_faculty_section('Kevin Gregory Johnston', 'MATH 1050-06', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Kevin Gregory Johnston', 'MATH 1050-07', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Kevin Gregory Johnston', 'MATH 3120-01', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Kevin Gregory Johnston', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('McKay Sullivan', 'Mathematics', default_availability)
    db.make_faculty_section('McKay Sullivan', 'MATH 1060-03', '3 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('McKay Sullivan', 'MATH 1210-04', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('McKay Sullivan', 'MATH 2210-01', 'MTWR0800+50', 'snow math rooms')
    db.make_faculty_section('McKay Sullivan', 'MATH 4000-01', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('McKay Sullivan', 'MT',
        *default_prefs,
    )

    db.make_faculty('Costel Ionita', 'Mathematics', default_availability)
    db.make_faculty_section('Costel Ionita', 'MATH 1060-40')
    db.make_faculty_section('Costel Ionita', 'MATH 1210-01', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Costel Ionita', 'MATH 1220-02', '4 credit bell schedule', 'snow math rooms')
    db.make_faculty_section('Costel Ionita', 'MATH 2270-01', '3 credit bell schedule', 'snow math rooms')
    db.faculty_preferences('Costel Ionita', 'MT',
        *reduced_default_prefs,
    )

    db.make_faculty('Vinodh Kumar Chellamuthu', 'Mathematics', default_availability)
    db.make_faculty_section('Vinodh Kumar Chellamuthu', 'MATH 3700-01', 'MW1630+75', 'snow math rooms')
