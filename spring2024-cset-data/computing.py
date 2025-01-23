import queries

def build(db: queries.DB) -> None:
    print('building smith building and classrooms')
    db.make_building('Smith')
    db.make_room('Smith 107', 32, ['flex'])
    db.make_room('Smith 108', 32, ['flex'])
    db.make_room('Smith 109', 32, ['flex'])
    db.make_room('Smith 112', 24, ['macs'])
    db.make_room('Smith 113', 24, ['pcs'])
    db.make_room('Smith 116', 38, ['stadium'])
    db.make_room('Smith 117', 38, ['stadium'])

    print('building computing time slots')
    db.make_time_slot('MWF0800+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF0900+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1000+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MWF1100+50', ['3 credit bell schedule', 'MWF 3×50 bell schedule'])
    db.make_time_slot('MW1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('MW1630+75', ['3 credit bell schedule', '2×75 bell schedule', 'MW 2×75 bell schedule'])
    db.make_time_slot('TR0900+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1030+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1200+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1330+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1500+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('TR1630+75', ['3 credit bell schedule', '2×75 bell schedule', 'TR 2×75 bell schedule'])
    db.make_time_slot('R1800+150', ['1×150 evening'])
    db.make_time_slot('T1800+150', ['1×150 evening'])
    db.make_time_slot('W1800+150', ['1×150 evening'])
    db.make_time_slot('R1900+50', [])
    db.make_time_slot('F1300+50', [])

    print('building computing conflicts')
    db.make_program('Computer Science', 'Computing')
    db.make_conflict('Computer Science', '3rd/4th semester classes', 1, 'boost',
        ['CS 2420', 'CS 2450', 'CS 2810', 'CS 3005'])
    db.make_conflict('Computer Science', 'grad plan: 2nd year fall', 1, 'boost',
        ['CS 2420', 'CS 2450', 'CS 2810'])
    db.make_conflict('Computer Science', 'grad plan: 2nd year spring', 1, 'boost',
        ['CS 3005', 'CS 3520', 'SE 3200'])
    db.make_conflict('Computer Science', 'grad plan: 3rd year fall', 1, 'boost',
        ['CS 3310', 'CS 3400', 'CS 3530'])
    db.make_conflict('Computer Science', 'grad plan: 3rd year spring', 1, 'boost',
        ['CS 3510', 'CS 4307', 'CS 4550'])
    db.make_conflict('Computer Science', 'grad plan: 4th year fall', 1, 'boost',
        ['CS 4300'])
    db.make_conflict('Computer Science', 'grad plan: 4th year spring', 1, 'boost',
        ['CS 3600', 'CS 4600'])
    db.make_conflict('Computer Science', 'core requirements', 3, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3310', 'CS 3400', 'CS 3410', 'CS 3510', 'CS 3520', 'CS 3530', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'CS 4600',
        'SE 3200'])
    db.make_conflict('Computer Science', 'electives', 9, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3310', 'CS 3400', 'CS 3410', 'CS 3510', 'CS 3520', 'CS 3530', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'CS 4600',
        'SE 3200',
        'SE 3010', 'SE 3020', 'SE 3100', 'SE 3400', 'SE 4200',
        'IT 2700', 'IT 3100', 'IT 3110', 'IT 4200'])
    db.make_conflict('Computer Science', 'math and science', 5, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810', 'CS 3005',
        'CS 3150', 'CS 3310', 'CS 3400', 'CS 3410', 'CS 3510', 'CS 3520', 'CS 3530', 'CS 3600',
        'CS 4300', 'CS 4307', 'CS 4320', 'CS 4550', 'CS 4600',
        'SE 3200',
        'MATH 1210', 'MATH 1220', 'BIOL 1610', 'BIOL 1615', 'PHYS 2210', 'PHYS 2215'])

    db.make_program('Data Science', 'Computing')
    db.make_conflict('Data Science', 'third semester', 7, 'boost',
        ['CS 2500', 'CS 2810', 'CS 3005'])

    db.make_program('Software Engineering', 'Computing')
    db.make_conflict('Software Engineering', 'core requirements', 3, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3150', 'CS 3310', 'CS 3510', 'CS 4307',
        'IT 1100', 'IT 2300',
        'SE 1400', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600'])
    db.make_conflict('Software Engineering', 'Entrepreneurial and marketing track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3150', 'CS 3310', 'CS 3510', 'CS 4307',
        'IT 1100', 'IT 2300',
        'SE 1400', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400', 'SE 3500', 'SE 3550',
        'SE 4200', 'SE 4600'])
    db.make_conflict('Software Engineering', 'DevOps track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3150', 'CS 3310', 'CS 3510', 'CS 4307',
        'IT 1100', 'IT 2300', 'IT 3110', 'IT 3300', 'IT 4200',
        'SE 1400', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600'])
    db.make_conflict('Software Engineering', 'Application track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3150', 'CS 3310', 'CS 3500', 'CS 3510', 'CS 4307',
        'IT 1100', 'IT 2300',
        'SE 1400', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400', 'SE 3450',
        'SE 4200', 'SE 4600'])
    db.make_conflict('Software Engineering', 'Data science track', 7, 'boost',
        ['CS 1030', 'CS 1400', 'CS 1410', 'CS 2420', 'CS 2450', 'CS 2810',
        'CS 3150', 'CS 3310', 'CS 3510', 'CS 4300', 'CS 4307', 'CS 4320',
        'IT 1100', 'IT 2300',
        'SE 1400', 'SE 3010', 'SE 3020', 'SE 3100', 'SE 3150', 'SE 3200', 'SE 3400',
        'SE 4200', 'SE 4600'])
    db.make_conflict('Software Engineering', 'only need one database class', 0, 'reduce',
        ['CS 4307', 'IT 2300'])

    db.make_program('Information Technology', 'Computing')
    db.make_conflict('Information Technology', 'core requirements', 3, 'boost',
        ['IT 1100', 'IT 1200', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3110', 'IT 3150', 'IT 3300', 'IT 3400',
        'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4600'])
    db.make_conflict('Information Technology', 'choose two section', 6, 'boost',
        ['CS 3005',
        'IT 1100', 'IT 1200', 'IT 2300', 'IT 2400', 'IT 2500', 'IT 2700',
        'IT 3100', 'IT 3110', 'IT 3150', 'IT 3300', 'IT 3400',
        'IT 4100', 'IT 4200', 'IT 4310', 'IT 4400', 'IT 4510', 'IT 4600',
        'SE 3200', 'SE 3400'])


    print('building computing faculty and sections')
    db.make_faculty('Bart Stander', 'Computing',
        '''MWF 0900-1200,
        MW  1200-1330 with priority 11,
        MW  1330-1630,
        TR  1030-1200,
        TR  1330-1500,
        TR  1500-1630 with priority 11''')
    db.faculty_default_clustering('Bart Stander', 'MT', 1)
    db.make_section('CS 2420-01', ['stadium', 'flex:11', 'MWF 3×50 bell schedule'])
    db.make_section('CS 3310-01', ['stadium', 'pcs', '3 credit bell schedule'])
    db.make_section('CS 3600-01', ['pcs', 'stadium:11', '3 credit bell schedule'])
    db.make_section('CS 4550-01', ['pcs', '3 credit bell schedule'])
    db.assign_faculty_sections('Bart Stander', ['CS 2420-01', 'CS 3310-01', 'CS 3600-01', 'CS 4550-01'])

    db.make_faculty('Carol Stander', 'Computing',
        '''MWF 1000-1200,
        MW  1200-1330 with priority 11,
        MW  1330-1500,
        TR  1330-1500 with priority 16''')
    db.faculty_default_clustering('Carol Stander', 'MT', -1)
    db.make_section('CS 1030-01', ['flex', '3 credit bell schedule'])
    db.make_section('CS 1410-01', ['flex', '3 credit bell schedule'])
    db.make_section('CS 1410-40', [])
    db.make_section('IT 1100-40', [])
    db.make_section('IT 2300-02', ['Smith 113', '3 credit bell schedule'])
    db.assign_faculty_sections('Carol Stander', ['CS 1030-01', 'CS 1410-01', 'CS 1410-40', 'IT 1100-40', 'IT 2300-02'])

    db.make_faculty('Curtis Larsen', 'Computing',
        '''MWF 0900-1100,
        MWF 1100-1200 with priority 11,
        MW  1200-1330 with priority 11,
        MW  1330-1630,
        TR  0900-1030,
        TR  1030-1330 with priority 11,
        TR  1330-1630''')
    db.faculty_default_clustering('Curtis Larsen', 'MT', 0)
    db.make_section('CS 3005-01', ['Smith 116', 'MWF 3×50 bell schedule'])
    db.make_section('CS 3510-01', ['Smith 116', 'flex:19', '3 credit bell schedule', 'TR 2×75 bell schedule:11'])
    db.make_section('CS 4320-01', ['Smith 116', 'flex:19', 'MWF 3×50 bell schedule:11', '2×75 bell schedule'])
    db.make_section('CS 4600-01', ['Smith 116', 'flex:19', '3 credit bell schedule', 'TR 2×75 bell schedule:11'])
    db.make_section('CS 4920R-01', [])
    db.assign_faculty_sections('Curtis Larsen', ['CS 3005-01', 'CS 3510-01', 'CS 4320-01', 'CS 4600-01', 'CS 4920R-01'])

    db.make_faculty('DJ Holt', 'Computing',
        '''MW 1200-1500,
        MW 1500-1630 with priority 11,
        TR 0900-1500,
        TR 1500-1630 with priority 11''')
    db.faculty_default_clustering('DJ Holt', 'MT', 0)
    # SE 3010-01 same day as SE 4200-01
    db.make_section('SE 3010-01', ['flex', 'macs', 'MW1500+75'])
    db.make_section('SE 4200-01', ['flex', 'macs', 'MW1330+75'])
    db.make_section('SE 4600-01', [])
    db.make_section('CS 4600-02', ['flex', '3 credit bell schedule'])
    db.assign_faculty_sections('DJ Holt', ['SE 3010-01', 'SE 4200-01', 'CS 4600-02'])
    # crosslist!(t, "SE 4600-01" cross-list with "CS 4600-02");
    # anticonflict!(t, set priority to 16, single: "CS 4600-01", group: "CS 4600-02");

    db.make_faculty('Eric Pedersen', 'Computing',
        '''TR  1200-1330''')
    db.make_section('SE 3500-01', ['flex', 'TR1200+75'])
    db.assign_faculty_sections('Eric Pedersen', ['SE 3500-01'])

    db.make_faculty('Jay Sneddon', 'Computing',
        '''MWF 0800-0900 with priority 10,
        MWF 0900-1200 with priority 11,
        MW  1200-1630,
        TR  0900-1500,
        TR  1500-1630 with priority 16''')
    db.faculty_default_clustering('Jay Sneddon', 'MT', 0)
    db.make_section('IT 1200-01', ['Smith 107', 'TR 2×75 bell schedule'])
    db.make_section('IT 2300-01', ['Smith 107', 'Smith 113', '3 credit bell schedule'])
    db.make_section('IT 2700-01', ['Smith 107', 'TR 2×75 bell schedule'])
    db.make_section('IT 3150-01', ['Smith 107', 'MW 2×75 bell schedule', 'MWF 3×50 bell schedule:16'])
    db.make_section('IT 3400-01', ['Smith 107', '3 credit bell schedule'])
    db.assign_faculty_sections('Jay Sneddon', ['IT 1200-01', 'IT 2300-01', 'IT 2700-01', 'IT 3150-01', 'IT 3400-01'])

    db.make_faculty('Jeff Compas', 'Computing',
        '''MWF 0800-0900,
        MW  1630-1800,
        TR  1630-1800,
        T   1800-2030''')
    db.make_section('CS 1400-01', ['stadium', '3 credit bell schedule', '1×150 evening'])
    db.make_section('CS 1400-50', ['stadium', '3 credit bell schedule', '1×150 evening'])
    db.make_section('CS 2450-02', ['flex', '3 credit bell schedule', '1×150 evening'])
    db.make_section('SE 3100-01', ['flex', '3 credit bell schedule', '1×150 evening'])
    db.assign_faculty_sections('Jeff Compas', ['CS 1400-01', 'CS 1400-50', 'CS 2450-02', 'SE 3100-01'])

    db.make_faculty('Joe Francom', 'Computing',
        '''MWF 0800-1200,
        MW  1330-1500''')
    db.faculty_default_clustering('Joe Francom', 'MT', 1)
    db.make_section('IT 1500-40A', [])
    db.make_section('IT 1500-41A', [])
    db.make_section('IT 1500-42A', [])
    db.make_section('IT 3110-01', ['flex', '3 credit bell schedule'])
    db.make_section('IT 4600-01', ['flex', '3 credit bell schedule'])
    db.make_section('SE 4900R-02', [])
    db.assign_faculty_sections('Joe Francom', ['IT 1500-40A', 'IT 1500-41A', 'IT 1500-42A', 'IT 3110-01', 'IT 4600-01', 'SE 4900R-02'])

    db.make_faculty('Lora Klein', 'Computing',
        '''TR 0900-1500,
        MW 1500-1630 with priority 10''')
    db.faculty_default_clustering('Lora Klein', 'MT', -1)
    db.make_section('SE 3200-01', ['Smith 107:16', 'flex', '3 credit bell schedule'])
    db.assign_faculty_sections('Lora Klein', ['SE 3200-01'])

    db.make_faculty('Matt Kearl', 'Computing',
        '''MW 1200-1330,
        TR 0900-1330''')
    db.faculty_default_clustering('Matt Kearl', 'MT', 1)
    db.make_section('SE 1400-02', ['macs', '3 credit bell schedule'])
    db.make_section('SE 1400-40', [])
    db.make_section('SE 1400-42', [])
    db.make_section('SE 3450-01', ['flex', 'macs', '3 credit bell schedule'])
    db.make_section('SE 3550-01', ['flex', 'macs', '3 credit bell schedule'])
    db.make_section('SE 4900R-01', [])
    db.make_section('SE 4920-01', [])
    db.assign_faculty_sections('Matt Kearl', ['SE 1400-40', 'SE 1400-42', 'SE 1400-02', 'SE 3450-01', 'SE 3550-01', 'SE 4900R-01', 'SE 4920-01'])

    db.make_faculty('Phil Daley', 'Computing',
        '''MWF 0900-1200,
        MW  1200-1500,
        MW  1500-1630 with priority 11,
        TR  0900-1500,
        TR  1500-1630 with priority 11''')
    db.faculty_default_clustering('Phil Daley', 'MT', 0)
    db.make_section('IT 1100-01', ['pcs', '3 credit bell schedule'])
    db.make_section('IT 1100-02', ['pcs', '3 credit bell schedule'])
    db.make_section('IT 2400-01', ['Smith 107', '3 credit bell schedule'])
    db.make_section('IT 3100-01', ['Smith 107', '3 credit bell schedule'])
    db.assign_faculty_sections('Phil Daley', ['IT 1100-01', 'IT 1100-02', 'IT 2400-01', 'IT 3100-01'])

    db.make_faculty('Derek Sneddon', 'Computing',
        '''R 1800-2230''')
    db.make_section('IT 4510-01', ['flex', 'R1800+150'])
    db.assign_faculty_sections('Derek Sneddon', ['IT 4510-01'])

    db.make_faculty('Ren Quinn', 'Computing',
        '''MWF 0900-1200,
        TR  1200-1330 with priority 16,
        TR  1330-1630,
        R   1900-2000,
        F   1300-1400''')
    db.faculty_default_clustering('Ren Quinn', 'MT', 0)
    db.make_section('CS 1400-02', ['flex', '3 credit bell schedule'])
    db.make_section('CS 1400-03', ['flex', '3 credit bell schedule'])
    db.make_section('CS 1410-02', ['flex', '3 credit bell schedule'])
    db.make_section('CS 2450-01', ['flex', '3 credit bell schedule'])
    db.make_section('CS 3150-01', ['flex', '3 credit bell schedule'])
    db.make_section('CS 4800R-01', [])
    db.make_section('CS 4991R-50', ['Smith 116', 'R1900+50'])
    db.make_section('CS 4992R-01', ['Smith 109', 'F1300+50'])
    db.assign_faculty_sections('Ren Quinn', ['CS 1400-02', 'CS 1400-03', 'CS 1410-02', 'CS 2450-01', 'CS 3150-01', 'CS 4800R-01', 'CS 4991R-50', 'CS 4992R-01'])

    db.make_faculty('Russ Ross', 'Computing',
        '''MTWR 1200-1500''')
    db.faculty_default_clustering('Russ Ross', 'MT', 0)
    db.make_section('CS 2810-01', ['Smith 109', '3 credit bell schedule'])
    db.make_section('CS 2810-02', ['Smith 109', '3 credit bell schedule'])
    db.make_section('CS 3410-01', ['Smith 109', '3 credit bell schedule'])
    db.make_section('CS 4307-01', ['Smith 109', '3 credit bell schedule'])
    db.make_section('CS 4800R-02', [])
    db.assign_faculty_sections('Russ Ross', ['CS 2810-01', 'CS 2810-02', 'CS 3410-01', 'CS 4307-01', 'CS 4800R-02'])

    db.make_faculty('Rex Frisbey', 'Computing',
        '''MWF 1100-1200''')
    db.make_section('SE 1400-01', ['macs', '3 credit bell schedule'])
    db.assign_faculty_sections('Rex Frisbey', ['SE 1400-01'])

    db.make_faculty('Jamie Bennion', 'Computing',
        '''W 1800-2030''')
    db.make_section('IT 4990-01', ['flex', '1×150 evening'])
    db.assign_faculty_sections('Jamie Bennion', ['IT 4990-01'])

    db.add_cross_listing('CS 4600-02', ['SE 4600-01'])
    db.add_anti_conflict(5, 'CS 4600-01', ['CS 4600-02'])
    db.add_anti_conflict(5, 'CS 1030-01', ['CS 1400'])
    #./edit add-anti-conflict 50 'SE 1400', 'IT 1100' # temporarily removed because of new hire planning

    db.make_conflict('Computer Science', 'spread out CS 1400', 1, 'boost',
        ['CS 1400-01', 'CS 1400-02', 'CS 1400-03', 'CS 1400-50'])
    db.make_conflict('Computer Science', 'spread out CS 1410', 1, 'boost',
        ['CS 1410-01', 'CS 1410-02'])
    db.make_conflict('Computer Science', 'spread out CS 2450', 1, 'boost',
        ['CS 2450-01', 'CS 2450-02'])
    db.make_conflict('Computer Science', 'spread out CS 2810', 1, 'boost',
        ['CS 2810-01', 'CS 2810-02'])
    db.make_conflict('Information Technology', 'spread out IT 1100', 1, 'boost',
        ['IT 1100-01', 'IT 1100-02'])
    db.make_conflict('Information Technology', 'spread out IT 2300', 1, 'boost',
        ['IT 2300-01', 'IT 2300-02'])
    db.make_conflict('Software Engineering', 'spread out SE 1400', 1, 'boost',
        ['SE 1400-01', 'SE 1400-02'])

    db.add_multiple_section_override('CS 4600', 1)
