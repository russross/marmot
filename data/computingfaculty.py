from queries import (
    DB,
    AvoidClassClusterLongerThan,
    AvoidClassClusterShorterThan,
    AvoidGapBetweenClassClustersLongerThan,
    AvoidSectionInRooms,
    AvoidSectionInTimeSlots,
    AvoidTimeSlot,
    DoNotWantADayOff,
    TimeInterval,
    UnavailableTimeSlot,
    WantADayOff,
    WantBackToBackClassesInTheSameRoom,
    WantClassesEvenlySpreadAcrossDays,
    WantClassesPackedIntoAsFewRoomsAsPossible,
)


def build_faculty(db: DB) -> None:
    """Build Spring 2027 assignments and ordered faculty preferences."""
    print('building computing faculty and sections')

    # Agreed Spring 2027 placements in the September 2 coordination email.
    # The external department's fixed booking reserves the room without
    # assigning Andrew Wilson or choosing a placement for his department.
    # db.make_section_with_no_faculty('UXD 6280-01', 'M1630+150', 'Smith 117')

    # Senior Project has four independent rooms at the same TR meeting time.
    # Each instructor's CS/SE pair is a single cross-listed meeting.

    # Deferred coordination request: Bart wants his and Carol's classes to use
    # the same all-MWF or all-TR option. Their individual preferences below
    # do not enforce a shared choice; no shared day restriction is active.
    db.make_faculty('Bart Stander', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Bart Stander', 'CS 2100-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Bart Stander', 'CS 2420-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Bart Stander', 'CS 3600-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Bart Stander', 'CS 4550-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')

    db.faculty_preferences('Bart Stander', 'MT',
        WantADayOff(),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        AvoidSectionInRooms('CS 3600-01', ['Smith 116']),
        AvoidSectionInRooms('CS 3600-01', ['Smith 117']),
        AvoidSectionInRooms('CS 3600-01', ['Smith 107']),
        AvoidSectionInRooms('CS 3600-01', ['Smith 108']),
        AvoidSectionInRooms('CS 3600-01', ['Smith 109']),
        AvoidSectionInRooms('CS 4550-01', ['Smith 116']),
        AvoidSectionInRooms('CS 4550-01', ['Smith 117']),
        AvoidSectionInRooms('CS 4550-01', ['Smith 107']),
        AvoidSectionInRooms('CS 4550-01', ['Smith 108']),
        AvoidSectionInRooms('CS 4550-01', ['Smith 109']),
    )

    db.make_faculty('Carol Stander', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Carol Stander', 'CS 1410-02', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'CS 1410-40')
    db.make_faculty_section('Carol Stander', 'IT 1100-40')
    db.make_faculty_section('Carol Stander', 'IT 2300-01', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'CS 1030-01', '3 credit bell schedule', 'flex', 'pcs')
    db.faculty_preferences('Carol Stander', 'MT',
        WantADayOff(),
        UnavailableTimeSlot('TR1030+75'),
        AvoidClassClusterLongerThan('1h50m'),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidSectionInTimeSlots('CS 1030-01', ['MWF0900+50']),
        AvoidSectionInTimeSlots('CS 1030-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('CS 1030-01', ['TR1500+75']),
        AvoidSectionInTimeSlots('CS 1030-01', ['MWF1000+50']),
        AvoidSectionInTimeSlots('CS 1030-01', ['MWF1100+50']),
        AvoidTimeSlot('MWF0900+50'),
        AvoidSectionInRooms('CS 1410-02', ['Smith 113']),
    )

    db.make_faculty('Curtis Larsen', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Curtis Larsen', 'CS 4320-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Curtis Larsen', 'CS 4600-03', '3 credit bell schedule', 'flex', 'stadium')
    db.make_section_with_no_faculty('SE 4600-03')
    db.add_cross_listing('CS 4600-03', ['SE 4600-03'])
    db.make_faculty_section('Curtis Larsen', 'CS 4920R-01', credit_hours=1)

    db.faculty_preferences('Curtis Larsen', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
    )

    db.make_faculty('DJ Holt', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('DJ Holt', 'CS 4600-02', '3 credit bell schedule', 'flex', 'stadium')
    db.make_section_with_no_faculty('SE 4600-02')
    db.add_cross_listing('CS 4600-02', ['SE 4600-02'])
    db.make_faculty_section('DJ Holt', 'CS 4800R-03', credit_hours=1)
    db.make_faculty_section('DJ Holt', 'SD 6200-01', 'T1630+150', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'SE 3250-01', '3 credit bell schedule', 'Smith 109')
    db.make_faculty_section('DJ Holt', 'SE 4200-01', '3 credit bell schedule', 'Smith 117')
    db.faculty_preferences('DJ Holt', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        WantADayOff(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        WantBackToBackClassesInTheSameRoom(),
    )

    db.make_faculty('Eric Pedersen', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Eric Pedersen', 'SD 6210-01', 'R1630+150', 'Smith 117')
    # db.make_section_with_no_faculty('UXD 6240-01')
    # db.add_cross_listing('SD 6210-01', ['UXD 6240-01'])
    db.make_faculty_section('Eric Pedersen', 'SE 4600-04', '3 credit bell schedule', 'flex', 'stadium')
    # ParksPass also includes 30 minutes outside Smith, arranged separately.
    db.make_faculty_section('Eric Pedersen', 'SE 4990-01', 'TR1500+75', 'Smith 112', credit_hours=3)
    db.make_faculty_section('Eric Pedersen', 'SE 3500-01', '3 credit bell schedule', 'flex', 'stadium')

    db.faculty_preferences('Eric Pedersen', 'MT',
        AvoidSectionInTimeSlots('SE 3500-01', ['MWF0900+50']),
        AvoidSectionInTimeSlots('SE 3500-01', ['MWF1000+50']),
        AvoidSectionInTimeSlots('SE 3500-01', ['MWF1100+50']),
        AvoidSectionInTimeSlots('SE 3500-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('SE 3500-01', ['MW1330+75']),
        AvoidSectionInTimeSlots('SE 3500-01', ['MW1200+75']),
        AvoidSectionInTimeSlots('SE 3500-01', ['TR0900+75']),
        AvoidSectionInTimeSlots('SE 3500-01', ['TR1200+75']),
        AvoidSectionInTimeSlots('SE 3500-01', ['TR1330+75']),
        AvoidSectionInTimeSlots('SE 3500-01', ['TR1500+75']),
        AvoidSectionInRooms('SE 3500-01', ['Smith 117']),
        AvoidSectionInRooms('SE 3500-01', ['Smith 107']),
        AvoidSectionInRooms('SE 3500-01', ['Smith 108']),
        AvoidSectionInRooms('SE 3500-01', ['Smith 109']),
    )

    db.make_faculty('Jay Sneddon', 'Computing', [TimeInterval('MTWR', '0900', '1630')])
    db.make_faculty_section('Jay Sneddon', 'IT 2700-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 2150-01', '3 credit bell schedule', 'flex', 'macs', 'pcs', 'stadium')
    db.make_faculty_section('Jay Sneddon', 'IT 3710-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 3750-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 4920R-01', credit_hours=1)
    db.make_faculty_section('Jay Sneddon', 'IT 4920R-01B', credit_hours=1)
    # The approved Tuesday practice is distinct from the excluded TR1630 slot.
    db.make_faculty_section('Jay Sneddon', 'IT 4991R-01', 'T1630+100', 'Smith 107')

    db.faculty_preferences('Jay Sneddon', 'MT',
        UnavailableTimeSlot('TR1500+75'),
        UnavailableTimeSlot('TR1630+75'),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        DoNotWantADayOff(),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Jeff Compas', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Jeff Compas', 'CS 2450-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Jeff Compas', 'CS 4600-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_section_with_no_faculty('SE 4600-01')
    db.add_cross_listing('CS 4600-01', ['SE 4600-01'])
    db.make_faculty_section('Jeff Compas', 'SD 6220-01', 'W1630+150', 'Smith 117')
    db.make_faculty_section('Jeff Compas', 'SE 3100-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Jeff Compas', 'CS 3005-01', '3 credit bell schedule', 'flex', 'stadium')

    db.faculty_preferences('Jeff Compas', 'MT',
        AvoidSectionInTimeSlots('CS 2450-01', ['MWF0900+50']),
        AvoidSectionInTimeSlots('CS 2450-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('CS 2450-01', ['TR1500+75']),
        AvoidSectionInTimeSlots('CS 2450-01', ['MWF1000+50']),
        AvoidSectionInTimeSlots('CS 2450-01', ['MW1330+75']),
        AvoidSectionInTimeSlots('CS 2450-01', ['TR1330+75']),
        AvoidSectionInTimeSlots('CS 2450-01', ['MWF1100+50']),
        AvoidSectionInTimeSlots('CS 2450-01', ['MW1200+75']),
        AvoidSectionInTimeSlots('CS 2450-01', ['TR1200+75']),
        AvoidSectionInTimeSlots('CS 2450-01', ['TR1030+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['MWF0900+50']),
        AvoidSectionInTimeSlots('CS 3005-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['TR1500+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['MWF1000+50']),
        AvoidSectionInTimeSlots('CS 3005-01', ['MW1330+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['TR1330+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['MWF1100+50']),
        AvoidSectionInTimeSlots('CS 3005-01', ['MW1200+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['TR1200+75']),
        AvoidSectionInTimeSlots('CS 3005-01', ['TR0900+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MWF0900+50']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['TR1500+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MWF1000+50']),
        AvoidSectionInTimeSlots('SE 3100-01', ['TR1330+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MWF1100+50']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MW1200+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['TR1200+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['TR1030+75']),
        AvoidSectionInTimeSlots('SE 3100-01', ['TR0900+75']),
    )

    db.make_faculty('Joe Francom', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Joe Francom', 'IT 1500-40A')
    db.make_faculty_section('Joe Francom', 'IT 1500-41B')
    db.make_faculty_section('Joe Francom', 'IT 3110-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Joe Francom', 'MT',
        UnavailableTimeSlot('MW1200+75'),
        UnavailableTimeSlot('MW1500+75'),
        UnavailableTimeSlot('TR1500+75'),
        AvoidSectionInRooms('IT 3110-01', ['Smith 116']),
        AvoidSectionInRooms('IT 3110-01', ['Smith 117']),
    )

    db.make_faculty('Kalyan Venugopal', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Kalyan Venugopal', 'CS 1400-02', '3 credit bell schedule', 'flex', 'stadium')
    # First-block one-credit project with no scheduled meetings.
    db.make_faculty_section('Kalyan Venugopal', 'CS 6352-01', credit_hours=1)
    db.faculty_preferences('Kalyan Venugopal', 'MT',
    )

    db.make_faculty('Lora Klein', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    # External CS meetings are MW and IT meetings are TR: section 03SJ at
    # 09:30–10:50, section 04SJ at 13:30–14:50. They reserve no Smith room.
    # SE 3200's allowed slots exclude all four commitments.
    db.make_faculty_section('Lora Klein', 'CS 1410-03SJ')
    db.make_faculty_section('Lora Klein', 'CS 1410-04SJ')
    db.make_faculty_section('Lora Klein', 'IT 1100-03SJ')
    db.make_faculty_section('Lora Klein', 'IT 1100-04SJ')
    db.make_faculty_section('Lora Klein', 'SE 3200-01', 'MW1200+75', 'TR1200+75', 'MW1500+75', 'TR1500+75', 'flex', 'stadium')

    db.faculty_preferences('Lora Klein', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1330+75'),
        AvoidSectionInRooms('SE 3200-01', ['Smith 116']),
        AvoidSectionInRooms('SE 3200-01', ['Smith 117']),
        AvoidTimeSlot('TR1500+75'),
    )

    db.make_faculty('Matt Kearl', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Matt Kearl', 'SE 1400-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 1400-40')
    db.make_faculty_section('Matt Kearl', 'SE 3450-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 3550-40')
    db.make_faculty_section('Matt Kearl', 'SE 4920-01')
    db.faculty_preferences('Matt Kearl', 'MT',
        AvoidSectionInTimeSlots('SE 1400-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('SE 3450-01', ['MW1500+75']),
        AvoidSectionInTimeSlots('SE 1400-01', ['MW1200+75']),
        AvoidSectionInTimeSlots('SE 3450-01', ['MW1200+75']),
        AvoidSectionInTimeSlots('SE 1400-01', ['MW1330+75']),
        AvoidSectionInTimeSlots('SE 3450-01', ['MW1330+75']),
        AvoidSectionInTimeSlots('SE 1400-01', ['TR1500+75']),
        AvoidSectionInTimeSlots('SE 3450-01', ['TR1500+75']),
    )

    db.make_faculty('Phil Daley', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Phil Daley', 'IT 1100-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 1100-02', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 2400-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Phil Daley', 'IT 3100-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 3400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Phil Daley', 'MT',
        WantADayOff(),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidSectionInRooms('IT 1100-01', ['Smith 116']),
        AvoidSectionInRooms('IT 1100-01', ['Smith 117']),
        AvoidSectionInRooms('IT 1100-01', ['Smith 107']),
        AvoidSectionInRooms('IT 1100-01', ['Smith 108']),
        AvoidSectionInRooms('IT 1100-01', ['Smith 109']),
        AvoidSectionInRooms('IT 1100-02', ['Smith 116']),
        AvoidSectionInRooms('IT 1100-02', ['Smith 117']),
        AvoidSectionInRooms('IT 1100-02', ['Smith 107']),
        AvoidSectionInRooms('IT 1100-02', ['Smith 108']),
        AvoidSectionInRooms('IT 1100-02', ['Smith 109']),
        WantBackToBackClassesInTheSameRoom(),
    )

    db.make_faculty('Ren Quinn', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Ren Quinn', 'CS 1400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 1410-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 3150-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 4400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 4800R-01', credit_hours=1)
    db.make_faculty_section('Ren Quinn', 'CS 4991R-01', 'F1400+50', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 4992R-01', 'F1300+50', 'flex')
    db.faculty_preferences('Ren Quinn', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        WantBackToBackClassesInTheSameRoom(),
        AvoidSectionInRooms('CS 1400-01', ['Smith 116']),
        AvoidSectionInRooms('CS 1400-01', ['Smith 117']),
        AvoidSectionInRooms('CS 1410-01', ['Smith 116']),
        AvoidSectionInRooms('CS 1410-01', ['Smith 117']),
        AvoidSectionInRooms('CS 3150-01', ['Smith 116']),
        AvoidSectionInRooms('CS 3150-01', ['Smith 117']),
        AvoidSectionInRooms('CS 4400-01', ['Smith 116']),
        AvoidSectionInRooms('CS 4400-01', ['Smith 117']),
        AvoidTimeSlot('MWF0900+50'),
        DoNotWantADayOff(),
    )

    db.make_faculty('Russ Ross', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Russ Ross', 'CS 2810-01', '3 credit bell schedule', 'Smith 109')
    db.make_faculty_section('Russ Ross', 'CS 3410-01', '3 credit bell schedule', 'Smith 109')
    db.make_faculty_section('Russ Ross', 'CS 4307-01', '3 credit bell schedule', 'Smith 109')
    db.make_faculty_section('Russ Ross', 'CS 4990-01', '3 credit bell schedule', 'Smith 109', credit_hours=3)
    db.make_faculty_section('Russ Ross', 'CS 4800R-02', credit_hours=1)
    db.faculty_preferences('Russ Ross', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidClassClusterLongerThan('2h45m'),
        WantClassesEvenlySpreadAcrossDays(),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
    )

    db.make_faculty('Syed Ali', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Syed Ali', 'IT 4510-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 4600-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 4700-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 2600-01', '3 credit bell schedule', 'flex', 'stadium')
    # The four requested meeting times are interchangeable among these sections.
    # The standard schedule's closest non-Friday pool is MW 12:00 and 13:30
    # plus TR 09:00 and 10:30.
    db.faculty_preferences('Syed Ali', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1200+75'),
        AvoidSectionInRooms('IT 4510-01', ['Smith 116']),
        AvoidSectionInRooms('IT 4510-01', ['Smith 117']),
        AvoidSectionInRooms('IT 2600-01', ['Smith 116']),
        AvoidSectionInRooms('IT 2600-01', ['Smith 117']),
        AvoidSectionInRooms('IT 4700-01', ['Smith 116']),
        AvoidSectionInRooms('IT 4700-01', ['Smith 117']),
        AvoidSectionInRooms('IT 4600-01', ['Smith 116']),
        AvoidSectionInRooms('IT 4600-01', ['Smith 117']),
    )

    db.make_faculty('Yuanfei Sun', 'Computing', [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')])
    db.make_faculty_section('Yuanfei Sun', 'CS 3510-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Yuanfei Sun', 'CS 3510-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Yuanfei Sun', 'CS 6310-50', 'M1800+150', 'Smith 116')
    db.make_faculty_section('Yuanfei Sun', 'CS 6322-50', 'W1800+150', 'Smith 116')
    # First-block project: individual Zoom appointments, no regular meeting.
    db.make_faculty_section('Yuanfei Sun', 'CS 6350-01', credit_hours=1)
    db.faculty_preferences('Yuanfei Sun', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1500+75'),
        WantADayOff(),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidClassClusterLongerThan('2h45m'),
    )

    db.add_anti_conflict(5, 'CS 4600-01', ['CS 4600-02'])
    db.add_anti_conflict(5, 'CS 4600-02', ['CS 4600-03'])
    db.add_anti_conflict(5, 'CS 4600-03', ['SE 4600-04'])
    db.add_anti_conflict(5, 'SE 4600-04', ['CS 4600-01'])
