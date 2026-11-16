Scheduling notes
================

There is some department-wide data (lists of courses and programs,
curriculum requirements, buildings and rooms, etc.) that I gather
with input from the chair.


Core faculty/section data
-------------------------

The main task of input preparation centers around three kinds of
data.


### List of faculty and their section assignments

This comes primarily from the chair (with input from faculty), who
must balance department needs with faculty qualifications, workload
requirements, and preferences.

Some sections may not be assigned to a specific faculty, especially
when the plan is to assign it to an adjunct later. Such sections can
be created individually with their own room and time constraints, or
you can make a faculty placeholder, e.g., "IT Adjunct #1". This is
helpful if you anticipate an adjunct teaching multiple sections and
want to make sure they are not scheduled at the same time.


### Section room and time constraints

For each section the faculty/chair must specify the rooms where the
section can be assigned. This can be an individual list, or we can
create “tags” that alias a group of rooms. For example, in the Smith
we label 107, 108, and 109 as “flex” rooms. Specifying “flex” as the
room is shorthand for listing the three rooms.

This requirement is typically a mix of course requirements and
faculty preferences. Some courses may have size requirements or
equipment needs, or faculty may prefer some classrooms over others.

In addition, some sections may have special time requirements.
Normally a section can be assigned to any time when the faculty is
available, but some need to be pushed into specific slots (normally
early morning or evening slots) even when the faculty is more widely
available.


### Faculty time availability and preferences

In Computing we start with the assumption that every faculty is
available during the core hours of 9:00 AM–4:30 PM MTWR and
9:00 AM–noon on F. This may be narrowed under special circumstances
and broadened for early morning and evening classes, but that is the
starting point.

Faculty are then invited to specify their schedule preferences by creating a list of requests in descending order of priority. Marmot attempts to accommodate as many as possible.

Marmot balances preferences across faculty according to how much they limit scheduling choices. Each request is considered together with your more important requests: how much flexibility remains if they are all accommodated? A list of several small requests can be comparable to a shorter list of broader requests.

Put your most important requests first. Marmot uses that order while balancing your preferences against those of other faculty, giving greater priority to combinations of requests that leave more scheduling flexibility. The aim is to accommodate preferences across the department without giving one person's highly restrictive requests precedence over everyone else's.

Here are the specific requests a faculty can put in their list,
given as examples with descriptions. For all preferences that
concern the distribution of sections across days, we pick two
representative days. I normally use Monday and Tuesday to represent
MWF and TR classes, respectivelly, but you can pick a different pair
of representative days if circumstances call for it. The cluster, gap, and back-to-back room-switching preferences below also use these representative days.


#### Preferences related to how classes are spread across the week

*   WantADayOff(): prefer to have sections on Monday or Tuesday but
    not both. Typically, this implies having all classes on MWF or
    all classes on TR.
*   DoNotWantADayOff(): prefer to have sections on both days (and by
    implication on all days of the week).
*   WantClassesEvenlySpreadAcrossDays(): prefer to have similar numbers of classes on the representative days that have classes, allowing a difference of one. Empty days are ignored, so combine this with DoNotWantADayOff() if you also want classes on both days. This option is available for faculty with at least four scheduled sections.
*   WantSameDayOffAs('Other Faculty'): prefer that you and another faculty member have the same day off, with exactly one of the representative days free for each of you. Both faculty must include this request and use the same representative days, but it can have different importance to each person. Each faculty member can pair with only one other person. Add WantADayOff() separately if having a day off is also a preference even when the days cannot match.


#### Preferences related to minimizing room switching

*   WantBackToBackClassesInTheSameRoom(): prefer to have two
    sections in the same room when they are scheduled back-to-back
    and they have a room in common that neither section has a preference to avoid.
*   WantClassesPackedIntoAsFewRoomsAsPossible(): prefer to use as
    few rooms as possible across all the faculty's sections (as
    constrained by section requirements).


### Clusters and gaps

A “cluster” is a run of one or more sections with no break longer than 50 minutes between consecutive classes. Its length runs from the start of the first class to the end of the last, including those short breaks. A “gap” is the longer break between two clusters. The same 50-minute rule defines back-to-back classes for the room-switching preference above.

*   AvoidGapBetweenClassClustersShorterThan('1h30m'): prefer to have
    at least some minimum amount of time between clusters of classes
    on a given day, i.e., spread the clusters out.
*   AvoidGapBetweenClassClustersLongerThan('1h45m'): prefer to have
    at most some maximum amount of time between clusters of classes
    on a given day, i.e., prevent spreading the clusters out.
*   AvoidClassClusterShorterThan('1h50m'): prefer to group classes into clusters at least this long. One shorter cluster per day is allowed, so a day with only one class or one short cluster is fine.
*   AvoidClassClusterLongerThan('2h45m'): prefer to avoid clusters
    of classes that are too long.


### Time preferences for a faculty's overall schedule

*   AvoidTimeSlot('MWF0900+50'): prefer not to teach in this specific time slot. Each request names one existing time slot; list multiple requests in priority order to avoid several slots. This does not cover other slots that happen to overlap it.
*   UnavailableTimeSlot('TR1500+75'): make a concrete time slot
    unavailable. This is a hard exclusion, not a soft preference.
    If a section is explicitly assigned to that slot, the unavailable
    request still wins and the audit should catch the conflict. Like AvoidTimeSlot, this names a specific slot rather than blocking every overlapping slot. It is a requirement rather than a preference to balance against other faculty's requests.


### Section-specific requests

*   AvoidSectionInTimeSlots('SE 1400-03', ['MWF 3×50 bell schedule']):
    prefer that a specific section avoid one or more time slots
    (including groups of time slots as in the example). Typically
    used to favor a section having, for example, a 2×75 pattern vs a
    3×50 pattern (as in this example for SE 1400-03). This only
    affects time slots already allowed for that section.
*   UseSameTimePattern(['CS 2450-01', 'CS 2450-02']): prefer that
    the listed sections have the same time pattern,
    e.g., both are on 2×75 patterns or 3×50 patterns. This does not
    let you specify the preferred pattern, just that both sections
    should have the *same* pattern. The days and start times can differ; the number of meetings per week and the length of each meeting should match.

This last one requires a little extra explanation:

*   AvoidSectionInRooms('CS 3150-01', ['Smith 116']): prefer to
    avoid having a section in a specific room without making it
    impossible. This is balanced against other faculty's requests like any other preference.

This does not expand the room options for a section. The section's
room tags define the allowed rooms, and AvoidSectionInRooms only
adds a soft penalty to the intersection between its tags and the
section's allowed rooms.

The same rule applies to section-specific time requests: list every
room or time pattern that should remain possible on the section
itself, then use AvoidSectionInRooms or AvoidSectionInTimeSlots to
mark the less desirable subset.


Examples
--------

I defined `default_availability` as MTWR 09:00–16:30 and
F 09:00–12:00, i.e., the baseline availability of almost all
faculty.


### Braydon Connole

Braydon's input was pretty simple:

    db.make_faculty('Brayden Connole', 'Computing', default_availability)
    db.make_faculty_section('Brayden Connole', 'IT 4200-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 1400-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 1400-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 3020-01', '3 credit bell schedule', 'macs')
    db.faculty_preferences('Brayden Connole', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1030+75'),
        WantADayOff(),
    )

The input format is a Python script that calls into some helper
functions to build the input. Let's break this down:

    db.make_faculty('Brayden Connole', 'Computing', default_availability)

This creates Brayden as a faculty in the Computing department and
sets his availability hours to the standard set of times.

    db.make_faculty_section('Brayden Connole', 'IT 4200-01', '3 credit bell schedule', 'flex')

This creates his first section, IT 4200-01, and lets it be scheduled
on any standard 3-credit time slot that is within his available
hours (`'3 credit bell schedule'` is a tag I defined as part of
the system setup). `'flex'` is a tag that means this section is
allowed to be in Smith 107, Smith 108, or Smith 109.

    db.make_faculty_section('Brayden Connole', 'SE 1400-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 1400-02', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Brayden Connole', 'SE 3020-01', '3 credit bell schedule', 'macs')

His remaining three sections are similar.

    db.faculty_preferences('Brayden Connole', 'MT',
        AvoidTimeSlot('MWF0900+50'),
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('MWF1000+50'),
        AvoidTimeSlot('MWF1100+50'),
        AvoidTimeSlot('TR1030+75'),
        WantADayOff(),
    )

Here we specify Brayden's preferences. `'MT'` means that requests
related to course distribution across the week (the WantADayOff
request in this case) will look at Monday and Tuesday as the the
representative days.

So Braydon's top priorities are to avoid mornings, and he would also
like to have all of his classes either be MW/MWF or TR, but that is
less important than avoiding morning.


### Carol Stander

Carol's input is slightly more complicated:

    db.make_faculty('Carol Stander', 'Computing', default_availability)
    db.make_faculty_section('Carol Stander', 'CS 1030-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Carol Stander', 'CS 1400-40')
    db.make_faculty_section('Carol Stander', 'IT 1100-01', '3 credit bell schedule', 'pcs', 'flex', 'Smith 116')
    db.make_faculty_section('Carol Stander', 'IT 2300-01', '3 credit bell schedule', 'flex', 'pcs', 'Smith 116')
    db.make_faculty_section('Carol Stander', 'IT 2300-40')
    db.faculty_preferences('Carol Stander', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MWF0900+50'),
        AvoidSectionInRooms('IT 1100-01', ['flex', 'Smith 116']),
        AvoidSectionInRooms('IT 2300-01', ['pcs', 'Smith 116']),
        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidClassClusterLongerThan('1h50m'),
        AvoidSectionInTimeSlots('IT 1100-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('IT 2300-01', ['MWF 3×50 bell schedule']),
    )

Again let's take each part in turn:

    db.make_faculty('Carol Stander', 'Computing', default_availability)

This create's Carol's entry in the input and marks her as part of
Computing.

    db.make_faculty_section('Carol Stander', 'CS 1030-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Carol Stander', 'CS 1400-40')

CS 1400-40 is an online class so it does not use a room or occupy a
time slot. We still include it here to make it easier to track
workload, and to help when entering the schedule into the
spreadsheet format required by central scheduling.

    db.make_faculty_section('Carol Stander', 'IT 1100-01', '3 credit bell schedule', 'pcs', 'flex', 'Smith 116')
    db.make_faculty_section('Carol Stander', 'IT 2300-01', '3 credit bell schedule', 'flex', 'pcs', 'Smith 116')
    db.make_faculty_section('Carol Stander', 'IT 2300-40')

IT 1100-1 and IT 2300-1 can be in one of the flex rooms or the pcs
room (Smith 113) or Smith 116. You can name rooms using tags or
explicit room names, and the permitted set of rooms is the union of
everything listed.

    db.faculty_preferences('Carol Stander', 'MT',
        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('MWF0900+50'),

Carol also wants to avoid the early time slots. This ordering
implies that avoiding any TR morning slot is more important than
avoiding the MWF 09:00–09:50 slot. The order matters.

        AvoidSectionInRooms('IT 1100-01', ['flex', 'Smith 116']),
        AvoidSectionInRooms('IT 2300-01', ['pcs', 'Smith 116']),

She would really prefer to have IT 1100-01 in the pcs room and
IT 2300-01 in a flex room, but the other listed rooms remain
available as lower-priority choices.

        AvoidTimeSlot('MW1200+75'),
        AvoidTimeSlot('TR1200+75'),

Having lunch hour off is nice, but less important that the ones
already listed.

        AvoidClassClusterLongerThan('1h50m'),

Carol doesn't like long stretches of classes and would prefer clusters no longer than 1 hour 50 minutes, separated by breaks longer than 50 minutes.

        AvoidSectionInTimeSlots('IT 1100-01', ['MWF 3×50 bell schedule']),
        AvoidSectionInTimeSlots('IT 2300-01', ['MWF 3×50 bell schedule']),
    )

She would rather teach these two classes on TR or MW, just not MWF,
i.e., she wants those classes to be 75 minutes each.


### Ren Quinn

Ren has a few things worth mentioning:

    db.make_faculty('Ren Quinn', 'Computing', default_availability)
    db.make_faculty_section('Ren Quinn', 'CS 1400-03', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 1410-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 2500-01', '3 credit bell schedule', 'flex')
    db.make_faculty_section('Ren Quinn', 'CS 3150-01', '3 credit bell schedule', 'flex', 'Smith 116')
    db.make_faculty_section('Ren Quinn', 'CS 4800R-01')

The individual research class is not scheduled, but is included here
just like online classes and internship classes.

    db.make_faculty_section('Ren Quinn', 'CS 4991R-50', 'R1900+50', 'Smith 116')

This is an evening class with a non-standard time slot. An explicit
section time like this is allowed for that section even though
`default_availability` would not normally let him be scheduled after
4:30 PM. Note that there is no `'3 credit bell schedule'` listed, so
this class is fully constrained into this time slot. This is a
typical way to handle evening classes.

    db.make_faculty_section('Ren Quinn', 'CS 4992R-01', 'F1300+50', 'Smith 109')

This Friday afternoon class is similar.

    db.faculty_preferences('Ren Quinn', 'MT',
        AvoidSectionInRooms('CS 3150-01', ['Smith 116']),

Ren does not really want CS 3150-01 in Smith 116, but is willing to
allow it. This is balanced against other faculty's requests like any other preference.

        DoNotWantADayOff(),

Ren has lots of classes and wants to use every day of the week.

        AvoidTimeSlot('TR0900+75'),
        AvoidTimeSlot('TR1030+75'),
        AvoidTimeSlot('TR1200+75'),
        AvoidTimeSlot('MW1200+75'),

He has a few time slot preferences.

        WantBackToBackClassesInTheSameRoom(),
    )

He does not like having to switch rooms in the short time between
two back-to-back classes.
