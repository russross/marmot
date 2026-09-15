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
    UseSameTimePattern,
    WantADayOff,
    WantSameDayOffAs,
    WantBackToBackClassesInTheSameRoom,
    WantClassesEvenlySpreadAcrossDays,
    WantClassesPackedIntoAsFewRoomsAsPossible,
)


def build_faculty(db: DB) -> None:
    print('building computing faculty and sections')
    default_availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]

    db.make_faculty('Bart Stander', 'Computing', default_availability)
    db.make_faculty_section('Bart Stander', 'CS 2100-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Bart Stander', 'CS 2420-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Bart Stander', 'CS 3600-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Bart Stander', 'CS 4550-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')

    db.faculty_preferences('Bart Stander', 'MT',
        WantADayOff(),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1500+75'),
        WantSameDayOffAs('Carol Stander'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        AvoidSectionInRooms('CS 3600-01', ['stadium']),
        AvoidSectionInRooms('CS 3600-01', ['flex']),
        AvoidSectionInRooms('CS 4550-01', ['stadium']),
        AvoidSectionInRooms('CS 4550-01', ['flex']),
    )

    db.make_faculty('Brayden Connole', 'Computing', default_availability)
    db.make_faculty_section('Brayden Connole', 'CS 4600-03', '3 credit bell schedule', 'flex', 'stadium')
    db.make_section_with_no_faculty('SE 4600-03')
    db.add_cross_listing('CS 4600-03', ['SE 4600-03'])
    db.faculty_preferences('Brayden Connole', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
    )

    db.make_faculty('Carol Stander', 'Computing', default_availability)
    db.make_faculty_section('Carol Stander', 'CS 1410-02', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'CS 1410-40')
    db.make_faculty_section('Carol Stander', 'IT 1100-40')
    db.make_faculty_section('Carol Stander', 'IT 2300-01', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'CS 1030-01', '3 credit bell schedule', 'flex', 'pcs')
    db.faculty_preferences('Carol Stander', 'MT',
        WantADayOff(),
        AvoidClassClusterLongerThan('1h50m'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR1200+75'),
        WantSameDayOffAs('Bart Stander'),
    )

    db.make_faculty('Curtis Larsen', 'Computing', default_availability)
    db.make_faculty_section('Curtis Larsen', 'CS 4320-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Curtis Larsen', 'CS 4920R-01', credit_hours=1)
    db.faculty_preferences('Curtis Larsen', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MWF1100+50'),
    )

    db.make_faculty('DJ Holt', 'Computing', default_availability)
    db.make_faculty_section('DJ Holt', 'CS 4600-02', '3 credit bell schedule', 'flex', 'stadium')
    db.make_section_with_no_faculty('SE 4600-02')
    db.add_cross_listing('CS 4600-02', ['SE 4600-02'])
    db.make_faculty_section('DJ Holt', 'CS 4800R-03', credit_hours=1)
    db.make_faculty_section('DJ Holt', 'SD 6200-01', 'T1630+150', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'SE 3250-01', '3 credit bell schedule', 'Smith 109') # might drop
    db.make_faculty_section('DJ Holt', 'SE 4200-01', '3 credit bell schedule', 'stadium', 'flex')
    db.faculty_preferences('DJ Holt', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidClassClusterShorterThan('1h45m'),
        AvoidClassClusterShorterThan('4h'),
        AvoidClassClusterShorterThan('5h30m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        #WantADayOff(),
        AvoidSectionInRooms('SE 4200-01', ['flex', 'Smith 116']),
        AvoidSectionInRooms('CS 4600-02', ['flex', 'Smith 116']),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        WantBackToBackClassesInTheSameRoom(),
    )

    db.make_faculty('Eric Pedersen', 'Computing', default_availability)
    db.make_faculty_section('Eric Pedersen', 'SD 6210-01', 'R1630+150', 'Smith 117')
    db.make_section_with_no_faculty('UXD 6240-01')
    db.add_cross_listing('SD 6210-01', ['UXD 6240-01'])
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
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidSectionInRooms('SE 3500-01', ['Smith 117']),
        AvoidSectionInRooms('SE 3500-01', ['flex']),
    )

    db.make_faculty('Jay Sneddon', 'Computing', default_availability)
    db.make_faculty_section('Jay Sneddon', 'IT 1200-01', 'MW1200+75', 'MW1500+75', 'TR1200+75', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 2700-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Jay Sneddon', 'IT 2150-01', '3 credit bell schedule', 'flex', 'macs', 'pcs', 'stadium')
    db.make_faculty_section('Jay Sneddon', 'IT 3710-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Jay Sneddon', 'IT 3750-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 4920R-01', credit_hours=1)
    db.make_faculty_section('Jay Sneddon', 'IT 4920R-01B', credit_hours=1)
    db.make_faculty_section('Jay Sneddon', 'IT 4991R-01', 'T1630+50', 'Smith 107')

    db.faculty_preferences('Jay Sneddon', 'MT',
        UnavailableTimeSlot('TR1500+75'),
        UnavailableTimeSlot('TR1630+75'),
        AvoidSectionInRooms('IT 1200-01', ['Smith 108', 'Smith 109', 'stadium']),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Jeff Compas', 'Computing', default_availability)
    db.make_faculty_section('Jeff Compas', 'CS 2450-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Jeff Compas', 'CS 4600-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_section_with_no_faculty('SE 4600-01')
    db.add_cross_listing('CS 4600-01', ['SE 4600-01'])
    db.make_faculty_section('Jeff Compas', 'SD 6220-01', 'W1630+150', 'Smith 117')
    db.make_faculty_section('Jeff Compas', 'SE 3100-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Jeff Compas', 'CS 3005-01', '3 credit bell schedule', 'flex', 'stadium')

    db.faculty_preferences('Jeff Compas', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('MW1330+75'),
        AvoidTimeSlot('MW1200+75'),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Joe Francom', 'Computing', default_availability)
    db.make_faculty_section('Joe Francom', 'IT 1500-40A')
    db.make_faculty_section('Joe Francom', 'IT 1500-41B')
    db.make_faculty_section('Joe Francom', 'IT 3110-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Joe Francom', 'MT',
        UnavailableTimeSlot('MW1200+75'),
        UnavailableTimeSlot('MW1500+75'),
        UnavailableTimeSlot('TR1500+75'),
        AvoidSectionInRooms('IT 3110-01', ['stadium']),
    )

    db.make_faculty('Kalyan Venugopal', 'Computing', default_availability)
    db.make_faculty_section('Kalyan Venugopal', 'CS 1400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Kalyan Venugopal', 'CS 1400-02', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Kalyan Venugopal', 'CS 2320-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Kalyan Venugopal', 'CS 6351-01B', credit_hours=1)
    # First-block one-credit project with no scheduled meetings.
    db.make_faculty_section('Kalyan Venugopal', 'CS 6352-01A', credit_hours=1)

    db.faculty_preferences('Kalyan Venugopal', 'MT',
        WantADayOff(),
        UseSameTimePattern(['CS 1400-01', 'CS 1400-02']),
        AvoidSectionInTimeSlots('CS 1400-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('CS 1400-02', ['MWF 3×50 bell schedule']),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Lora Klein', 'Computing', default_availability)
    # Success: CS 1410 sections are MW0930+80 and MW1330+80
    #          IT 1100 sections are TR0930+80 and TR1330+80
    db.make_faculty_section('Lora Klein', 'SA 1410-01SJ', 'MW0930+80')
    db.make_faculty_section('Lora Klein', 'SA 1410-02SJ', 'MW1330+80')
    db.make_faculty_section('Lora Klein', 'SA 1100-01SJ', 'TR0930+80')
    db.make_faculty_section('Lora Klein', 'SA 1100-02SJ', 'TR1330+80')
    db.make_faculty_section('Lora Klein', 'SE 3200-01', '3 credit bell schedule', 'flex', 'stadium')

    db.faculty_preferences('Lora Klein', 'MT',
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1500+75'),
        AvoidSectionInRooms('SE 3200-01', ['stadium']),
    )

    db.make_faculty('Matt Kearl', 'Computing', default_availability)
    db.make_faculty_section('Matt Kearl', 'SE 1400-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 1400-40')
    db.make_faculty_section('Matt Kearl', 'SE 3450-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 3550-40')
    db.make_faculty_section('Matt Kearl', 'SE 4920-01')
    db.faculty_preferences('Matt Kearl', 'MT',
        AvoidSectionInTimeSlots('SE 1400-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('SE 3450-01', ['MWF 3×50 bell schedule']),
        WantADayOff(),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('MW1330+75'),
        AvoidTimeSlot('MW1200+75'),
    )

    db.make_faculty('Phil Daley', 'Computing', default_availability)
    db.make_faculty_section('Phil Daley', 'IT 1100-01', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 1100-02', '3 credit bell schedule', 'flex', 'pcs', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 2400-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Phil Daley', 'IT 3100-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 3400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Phil Daley', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1200+75'),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidSectionInRooms('IT 1100-01', ['stadium', 'flex']),
        AvoidSectionInRooms('IT 1100-02', ['stadium', 'flex']),
        WantBackToBackClassesInTheSameRoom(),
    )

    db.make_faculty('Ren Quinn', 'Computing', default_availability)
    db.make_faculty_section('Ren Quinn', 'CS 1410-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 3150-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 3510-02', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 4400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 4800R-01', credit_hours=1)
    db.make_faculty_section('Ren Quinn', 'CS 4991R-01', 'F1400+50', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 4992R-01', 'F1300+50', 'flex')
    db.faculty_preferences('Ren Quinn', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        WantBackToBackClassesInTheSameRoom(),
        AvoidSectionInRooms('CS 1410-01', ['stadium']),
        AvoidSectionInRooms('CS 3150-01', ['stadium']),
        AvoidSectionInRooms('CS 4400-01', ['stadium']),
        AvoidTimeSlot('MWF0900+50'),
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Russ Ross', 'Computing', default_availability)
    db.make_faculty_section('Russ Ross', 'CS 2810-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Russ Ross', 'CS 3410-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Russ Ross', 'CS 4307-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Russ Ross', 'CS 4990-01', '3 credit bell schedule', 'flex', credit_hours=3)
    db.make_faculty_section('Russ Ross', 'CS 4800R-02', credit_hours=1)
    db.faculty_preferences('Russ Ross', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        #DoNotWantADayOff(),
        AvoidClassClusterLongerThan('2h45m'),
        #AvoidClassClusterShorterThan('1h50m'),
        #AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidTimeSlot('TR1030+75'),
        #WantClassesEvenlySpreadAcrossDays(),
        AvoidSectionInRooms('CS 2810-01', ['Smith 107']),
        AvoidSectionInRooms('CS 3410-01', ['Smith 107']),
        AvoidSectionInRooms('CS 4307-01', ['Smith 107']),
        AvoidSectionInRooms('CS 4990-01', ['Smith 107']),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
    )

    db.make_faculty('Syed Ali', 'Computing', default_availability)
    db.make_faculty_section('Syed Ali', 'IT 4510-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 4600-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 4700-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 2600-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Syed Ali', 'MT',
        AvoidSectionInTimeSlots('IT 4600-01', ['MWF 3×50 bell schedule']),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1200+75'),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidSectionInRooms('IT 4510-01', ['stadium']),
        AvoidSectionInRooms('IT 2600-01', ['stadium']),
        AvoidSectionInRooms('IT 4700-01', ['stadium']),
        AvoidSectionInRooms('IT 4600-01', ['stadium']),
    )

    db.make_faculty('Yuanfei Sun', 'Computing', default_availability)
    db.make_faculty_section('Yuanfei Sun', 'CS 3510-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Yuanfei Sun', 'CS 6310-50', 'M1800+150', 'Smith 116')
    db.make_faculty_section('Yuanfei Sun', 'CS 6322-50', 'W1800+150', 'Smith 116')
    # First-block project: individual Zoom appointments, no regular meeting.
    db.make_faculty_section('Yuanfei Sun', 'CS 6350-01A', credit_hours=1)
    db.make_faculty_section('Yuanfei Sun', 'CS 6353-01B', credit_hours=1)
    db.faculty_preferences('Yuanfei Sun', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidTimeSlot('TR1500+75'),
        WantADayOff(),
        AvoidGapBetweenClassClustersLongerThan('6h5m'),
        AvoidGapBetweenClassClustersLongerThan('4h40m'),
        AvoidGapBetweenClassClustersLongerThan('3h10m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidClassClusterLongerThan('2h45m'),
    )

    #db.make_section_with_no_faculty('MATH 2250-01', 'MTWR1000+50')
    #db.make_section_with_no_faculty('MATH 2280-01', 'MW1500+75')
    #db.make_section_with_no_faculty('MATH 3050-01', 'MW1200+75')
    #db.make_section_with_no_faculty('MATH 3450-01', 'TR1030+75')
