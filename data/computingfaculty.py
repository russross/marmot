import queries
from queries import *

def build_faculty(db: DB) -> None:
    print('building computing faculty and sections')
    default_availability = [TimeInterval('MTWR', '0900', '1630'), TimeInterval('F', '0900', '1200')]
    default_prefs_twoday = [
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    ]
    default_prefs_oneday = [
        WantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    ]

    db.make_faculty('Syed Ali', 'Computing', default_availability)
    db.make_faculty_section('Syed Ali', 'IT 1100-01', '3 credit bell schedule', 'pcs')
    db.make_faculty_section('Syed Ali', 'IT 1100-02', '3 credit bell schedule', 'pcs')
    db.make_faculty_section('Syed Ali', 'IT 4600-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Syed Ali', 'IT 4700-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Syed Ali', 'MT',
        *default_prefs_twoday,
    )

    #db.make_faculty('Brayden Connole', 'Computing', default_availability)
    #db.make_faculty_section('Brayden Connole', 'IT 4200-01', '3 credit bell schedule', 'flex')
    #db.make_faculty_section('Brayden Connole', 'SE 1400-01', '3 credit bell schedule', 'flex')
    #db.make_faculty_section('Brayden Connole', 'SE 1400-02', '3 credit bell schedule', 'flex')
    #db.make_faculty_section('Brayden Connole', 'SE 3020-01', '3 credit bell schedule', 'macs')
    #db.faculty_preferences('Brayden Connole', 'MT',
    #    AvoidTimeSlot('MWF0900+50'),
    #    AvoidTimeSlot('TR0900+75'),
    #    AvoidTimeSlot('MWF1000+50'),
    #    AvoidTimeSlot('MWF1100+50'),
    #    AvoidTimeSlot('TR1030+75'),
    #    WantADayOff(),
    #)

    db.make_faculty('Jeff Compas', 'Computing', default_availability)
    db.make_faculty_section('Jeff Compas', 'CS 2450-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Jeff Compas', 'CS 4600-01', 'TR1330+75', 'SET 105a')
    db.make_section_with_no_faculty('SE 4600-01')
    db.add_cross_listing('CS 4600-01', ['SE 4600-01'])
    db.make_faculty_section('Jeff Compas', 'SD 6220-01', 'W1630+150', 'Smith 117')
    db.make_faculty_section('Jeff Compas', 'SE 3100-01', '3 credit bell schedule', 'flex')
    db.faculty_preferences('Jeff Compas', 'MT',
        AvoidSectionInTimeSlots('CS 2450-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('CS 2450-01', ['MW 2×75 bell schedule']),
        AvoidSectionInTimeSlots('SE 3100-01', ['MW 2×75 bell schedule']),
        AvoidClassClusterShorterThan('1h30m'),
    )

    db.make_faculty('Phil Daley', 'Computing', default_availability)
    db.make_faculty_section('Phil Daley', 'IT 1100-03S', 'TR0800+80', 'SUCCESS 100')
    db.make_faculty_section('Phil Daley', 'IT 2400-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Phil Daley', 'IT 3100-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 3400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Phil Daley', 'IT 4100-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Phil Daley', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        WantBackToBackClassesInTheSameRoom(),
    )

    db.make_faculty('Joe Francom', 'Computing', default_availability)
    db.make_faculty_section('Joe Francom', 'IT 1500-40A')
    db.make_faculty_section('Joe Francom', 'IT 1500-41B')
    db.make_faculty_section('Joe Francom', 'IT 3110-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Joe Francom', 'SE 3200-01', '3 credit bell schedule', 'flex')
    db.faculty_preferences('Joe Francom', 'MT',
        WantADayOff(),
        AvoidSectionInRooms('IT 3110-01', ['stadium']),
        AvoidSectionInRooms('SE 3200-01', ['stadium']),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidSectionInRooms('IT 3110-01', ['Smith 112', 'Smith 113']),
        AvoidSectionInRooms('SE 3200-01', ['Smith 112', 'Smith 113']),
        UnavailableTimeSlot('MW1200+75'),
        UnavailableTimeSlot('MW1500+75'),
        UnavailableTimeSlot('TR1500+75'),
    )

    # note: all CS/SE 4600 in one room
    db.make_faculty('DJ Holt', 'Computing', default_availability)
    db.make_faculty_section('DJ Holt', 'CS 4800R-03')
    db.make_faculty_section('DJ Holt', 'SD 6200-01', 'T1630+150', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'SE 3250-01', '3 credit bell schedule', 'Smith 109', 'Smith 113')
    db.make_faculty_section('DJ Holt', 'SE 4200-01', '3 credit bell schedule', 'Smith 117')
    db.make_faculty_section('DJ Holt', 'CS 4600-02', 'TR1330+75', 'SET 105b')
    db.make_section_with_no_faculty('SE 4600-02')
    db.add_cross_listing('CS 4600-02', ['SE 4600-02'])
    db.faculty_preferences('DJ Holt', 'MT',
        AvoidSectionInRooms('SE 3250-01', ['Smith 107', 'Smith 108']),
        AvoidSectionInTimeSlots('SE 4200-01', ['MWF 3×50 bell schedule', 'TR 2×75 bell schedule']),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        WantBackToBackClassesInTheSameRoom(),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        AvoidClassClusterShorterThan('1h50m')
    )

    db.make_faculty('Kevin Johnson', 'Computing', default_availability)
    db.make_faculty_section('Kevin Johnson', 'CS 2320-01', '3 credit bell schedule', 'flex', 'stadium')

    db.make_faculty('Matt Kearl', 'Computing', default_availability)
    db.make_faculty_section('Matt Kearl', 'SE 1400-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 1400-40')
    db.make_faculty_section('Matt Kearl', 'SE 3400-40')
    db.make_faculty_section('Matt Kearl', 'SE 3450-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 3550-01', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.make_faculty_section('Matt Kearl', 'SE 4920-01')
    db.make_faculty_section('Matt Kearl', 'SE 4990-02', '3 credit bell schedule', 'macs', 'pcs', 'flex', 'stadium')
    db.faculty_preferences('Matt Kearl', 'MT',
        AvoidSectionInTimeSlots('SE 1400-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('SE 3450-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('SE 3550-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('SE 4990-02', ['MWF 3×50 bell schedule']),
        WantADayOff(),
        AvoidTimeSlot('MW1500+75'),
        AvoidTimeSlot('TR1500+75'),
        AvoidTimeSlot('MW1330+75'),
        AvoidTimeSlot('TR1330+75'),
        AvoidClassClusterShorterThan('2h45m'),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    # note: 100% success academy in the spring
    #db.make_faculty('Lora Klein', 'Computing', default_availability)
    #db.make_faculty_section('Lora Klein', 'CS 1400-00', '3 credit bell schedule', 'flex')
    #db.make_faculty_section('Lora Klein', 'SA 1400-01', 'TR0930+80')
    #db.make_faculty_section('Lora Klein', 'SA 1400-02', 'TR1200+80')
    #db.faculty_preferences('Lora Klein', 'MT',
    #    AvoidTimeSlot('MWF0900+50'),
    #    AvoidTimeSlot('MWF1000+50'),
    #    #WantClassesEvenlySpreadAcrossDays(),
    #    AvoidTimeSlot('MWF1100+50'),
    #    AvoidTimeSlot('TR1330+75'),
    #    AvoidTimeSlot('TR1500+75'),
    #)

    db.make_faculty('Curtis Larsen', 'Computing', default_availability)
    db.make_faculty_section('Curtis Larsen', 'CS 4320-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Curtis Larsen', 'CS 4600-03', 'TR1330+75', 'SET 105c')
    db.make_section_with_no_faculty('SE 4600-03')
    db.add_cross_listing('CS 4600-03', ['SE 4600-03'])
    db.make_faculty_section('Curtis Larsen', 'CS 4920R-01')
    db.make_faculty_section('Curtis Larsen', 'CS 6330-40') # online
    db.make_faculty_section('Curtis Larsen', 'CS 6350-01') # scheduled with groups individually
    db.faculty_preferences('Curtis Larsen', 'MT',
        WantADayOff(),
        AvoidClassClusterLongerThan('2h45m'),
        AvoidGapBetweenClassClustersLongerThan('3h15m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
    )

    # note: manually add UXD cross listing
    # note: sandbox time ends at 4:30 despite being a 3 credit class
    db.make_faculty('Eric Pedersen', 'Computing', default_availability)
    db.make_faculty_section('Eric Pedersen', 'SD 6210-01', 'R1630+150', 'Smith 117')
    #db.make_section_with_no_faculty('UXD 6240-01')
    #db.add_cross_listing('SD 6210-01', ['UXD 6240-01'])
    db.make_faculty_section('Eric Pedersen', 'SE 3500-01', 'TR1200+75', 'flex')
    db.make_faculty_section('Eric Pedersen', 'SE 4990-01', 'TR1530+60', 'Smith 112') # sandbox
    # Lora has sandbox at same time

    db.make_faculty('Ren Quinn', 'Computing', default_availability)
    db.make_faculty_section('Ren Quinn', 'CS 1400-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 1400-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 1410-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 3150-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 4400-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Ren Quinn', 'CS 4800R-01')
    db.make_faculty_section('Ren Quinn', 'CS 4991R-50', 'F1400+50', 'Holland 469')
    db.make_faculty_section('Ren Quinn', 'CS 4992R-01', 'F1300+50', 'Holland 469')
    db.faculty_preferences('Ren Quinn', 'MT',
        AvoidSectionInRooms('CS 3150-01', ['Smith 116']),
        DoNotWantADayOff(),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        WantBackToBackClassesInTheSameRoom(),
        AvoidSectionInRooms('CS 3150-01', ['stadium']),
        AvoidSectionInRooms('CS 4400-01', ['stadium']),
        AvoidTimeSlot('MWF0900+50'),
    )

    db.make_faculty('Russ Ross', 'Computing', [TimeInterval('MTWR', '1200', '1630')])
    db.make_faculty_section('Russ Ross', 'CS 2810-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Russ Ross', 'CS 2810-02', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Russ Ross', 'CS 3410-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Russ Ross', 'CS 4307-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Russ Ross', 'CS 4800R-02')
    db.faculty_preferences('Russ Ross', 'MT',
        WantClassesEvenlySpreadAcrossDays(),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
    )

    db.make_faculty('Jay Sneddon', 'Computing', default_availability)
    db.make_faculty_section('Jay Sneddon', 'IT 1200-01', 'MW1200+75', 'MW1500+75', 'TR1200+75', 'TR1500+75', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 3150-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 3710-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 3750-01', '3 credit bell schedule', 'Smith 107')
    db.make_faculty_section('Jay Sneddon', 'IT 4920R-01B')
    db.make_faculty_section('Jay Sneddon', 'IT 4991R-01', 'T1630+100', 'Smith 107')
    db.faculty_preferences('Jay Sneddon', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        DoNotWantADayOff(),
        WantClassesEvenlySpreadAcrossDays(),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Bart Stander', 'Computing', default_availability)
    db.make_faculty_section('Bart Stander', 'CS 2100-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Bart Stander', 'CS 2420-01', '3 credit bell schedule', 'Smith 116')
    db.make_faculty_section('Bart Stander', 'CS 3600-01', '3 credit bell schedule', 'pcs')
    db.make_faculty_section('Bart Stander', 'CS 4550-01', '3 credit bell schedule', 'pcs')
    db.faculty_preferences('Bart Stander', 'MT',
        WantADayOff(),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MW1200+75'),
        WantBackToBackClassesInTheSameRoom(),
        WantClassesPackedIntoAsFewRoomsAsPossible(),
        AvoidSectionInTimeSlots('CS 3600-01', ['TR1200+75'])
    )

    db.make_faculty('Carol Stander', 'Computing', default_availability)
    db.make_faculty_section('Carol Stander', 'CS 1030-01', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'CS 1410-02', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'CS 1410-40')
    db.make_faculty_section('Carol Stander', 'IT 1100-40')
    db.make_faculty_section('Carol Stander', 'IT 2300-01', '3 credit bell schedule', 'flex', 'pcs')
    db.make_faculty_section('Carol Stander', 'IT 2300-40')
    db.faculty_preferences('Carol Stander', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        WantADayOff(),
        AvoidClassClusterLongerThan('1h50m'),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidSectionInTimeSlots('CS 1030-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInRooms('CS 1410-02', ['pcs']),
        AvoidTimeSlot('MWF0900+50'),
    )

    db.make_faculty('Yuanfei Sun', 'Computing', default_availability)
    db.make_faculty_section('Yuanfei Sun', 'CS 3510-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Yuanfei Sun', 'CS 3510-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Yuanfei Sun', 'CS 6310-01', 'M1800+150', 'Smith 116')
    db.make_faculty_section('Yuanfei Sun', 'CS 6350-02')
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

    db.make_faculty('Kraig Wastlund', 'Computing', default_availability)
    db.make_faculty_section('Kraig Wastlund', 'CS 3005-01', 'MW1200+75', 'stadium')

    db.make_faculty('Visiting Faculty', 'Computing', default_availability)
    db.make_faculty_section('Visiting Faculty', 'CS 1400-03', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Visiting Faculty', 'IT 2700-01', '3 credit bell schedule', 'flex', 'stadium')
    db.make_faculty_section('Visiting Faculty', 'IT 4510-01', '3 credit bell schedule', 'flex', 'stadium')
    db.faculty_preferences('Visiting Faculty', 'MT',
        AvoidClassClusterLongerThan('2h45m'),
        AvoidClassClusterShorterThan('1h50m'),
        AvoidGapBetweenClassClustersLongerThan('1h45m'),
    )

    db.make_faculty('Tim Thayne', 'Computing', [TimeInterval('TR', '0900', '1145')])
    db.make_faculty_section('Tim Thayne', 'SE 3010-01', '3 credit bell schedule', 'Smith 112')
