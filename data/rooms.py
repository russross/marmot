import queries


# The earlier college dry run identified these as general science lecture
# halls. Capacities are provisional; floated rooms are offered only to the
# sections originally posted there, alongside suitable college-owned rooms.
SCIENCE_LECTURE_CAPACITIES = {
    'SET 105': 60, 'SET 106': 125, 'SET 201': 70, 'SET 301': 65,
    'SET 418': 48, 'SET 420': 48, 'SET 524': 45,
    'Snow 113': 40, 'Snow 128': 40, 'Snow 208': 30, 'Snow 216': 45,
    'Brown 201': 65, 'COE 121': 60, 'COE 204': 30, 'COE 208': 30,
    'GCB 131': 30, 'GCB 220': 40, 'GCB 318': 40, 'GCB 319': 40,
    'UHB 120': 30, 'UHB 230': 30,
}


def science_lecture_tag(posted_room: str) -> str | None:
    if posted_room not in SCIENCE_LECTURE_CAPACITIES:
        return None
    return f'science lecture from {posted_room}'


def science_room_tags(room: str) -> list[str]:
    if room not in SCIENCE_LECTURE_CAPACITIES:
        return []
    capacity = SCIENCE_LECTURE_CAPACITIES[room]
    owned = room.startswith(('SET ', 'Snow '))
    return [
        f'science lecture from {source}'
        for source, minimum in SCIENCE_LECTURE_CAPACITIES.items()
        if room == source or (owned and capacity >= minimum)
    ]


def build_rooms(db: queries.DB) -> None:
    print('building smith building and classrooms')
    db.make_building('Smith')
    db.make_room('Smith 107', 32, ['flex'])
    db.make_room('Smith 108', 32, ['flex'])
    db.make_room('Smith 109', 32, ['flex'])
    db.make_room('Smith 112', 24, ['macs'])
    db.make_room('Smith 114', 24, [])
    db.make_room('Smith 113', 24, ['pcs'])
    db.make_room('Smith 116', 38, ['stadium'])
    db.make_room('Smith 117', 38, ['stadium'])
    db.make_room('Smith 127', 24, [])

    # The 2026 mathematics dry run treated these Snow classrooms as
    # interchangeable. Room 003 is listed as room 3 in the 2024 inventory.
    db.make_building('Snow')
    for number in ('003', '112', '124', '125', '144', '145', '147', '150', '151'):
        db.make_room(f'Snow {number}', 40, ['snow math rooms'])
    db.make_room('Snow 103', 16, [])
    db.make_room('Snow 113', 40, science_room_tags('Snow 113'))
    db.make_room('Snow 128', 40, science_room_tags('Snow 128'))
    db.make_room('Snow 216', 45, science_room_tags('Snow 216'))
    db.make_room('Snow 204', 10, [])

    # The posted math sections have a cap of 40 in these rooms. This is an
    # inferred lower bound, not a verified room capacity or interchangeability.
    db.make_building('GCB')
    db.make_room('GCB 318', 40, science_room_tags('GCB 318'))
    db.make_room('GCB 319', 40, science_room_tags('GCB 319'))
    db.make_room('GCB 220', 40, science_room_tags('GCB 220'))

    # Physics lecture and lab rooms retain their posted assignments. The
    # capacities are from the earlier college room inventory and need review.
    db.make_building('SET')
    # Provisional capacities below are at least the largest posted section cap
    # using each room; equipment and true capacities still need confirmation.
    db.make_room('SET 101', 18, [])
    db.make_room('SET 102', 18, [])
    db.make_room('SET 104', 40, [])
    db.make_room('SET 105', 60, science_room_tags('SET 105'))
    db.make_room('SET 106', 125, science_room_tags('SET 106'))
    db.make_room('SET 201', 70, science_room_tags('SET 201'))
    db.make_room('SET 213', 26, [])
    db.make_room('SET 214', 24, [])
    db.make_room('SET 215', 24, [])
    db.make_room('SET 216', 30, [])
    db.make_room('SET 219', 24, [])
    db.make_room('SET 225', 20, [])
    db.make_room('SET 226', 40, [])
    db.make_room('SET 301', 65, science_room_tags('SET 301'))
    db.make_room('SET 304', 22, [])
    db.make_room('SET 308', 24, [])
    db.make_room('SET 309', 24, [])
    db.make_room('SET 310', 24, [])
    db.make_room('SET 312', 14, [])
    db.make_room('SET 318', 24, [])
    db.make_room('SET 319', 24, [])
    db.make_room('SET 404', 16, [])
    db.make_room('SET 405', 24, [])
    db.make_room('SET 407', 24, [])
    db.make_room('SET 408', 15, [])
    db.make_room('SET 409', 24, [])
    db.make_room('SET 410', 24, [])
    db.make_room('SET 412', 24, [])
    db.make_room('SET 418', 48, science_room_tags('SET 418'))
    db.make_room('SET 420', 48, science_room_tags('SET 420'))
    db.make_room('SET 524', 45, science_room_tags('SET 524'))
    db.make_room('SET 523', 24, [])
    db.make_room('SET 522', 24, [])
    db.make_room('SET 501', 20, [])
    db.make_room('SET 526', 24, [])
    db.make_room('SET 527', 24, [])
    db.make_room('SET 303', 12, [])
    db.make_room('Snow 208', 30, science_room_tags('Snow 208'))
    db.make_room('GCB 131', 30, science_room_tags('GCB 131'))
    db.make_building('Brown')
    db.make_room('Brown 201', 65, science_room_tags('Brown 201'))
    db.make_building('COE')
    db.make_room('COE 121', 60, science_room_tags('COE 121'))
    db.make_room('COE 204', 30, science_room_tags('COE 204'))
    db.make_room('COE 208', 30, science_room_tags('COE 208'))
    db.make_building('UHB')
    db.make_room('UHB 120', 30, science_room_tags('UHB 120'))
    db.make_room('UHB 230', 30, science_room_tags('UHB 230'))
    db.make_building('HURCTR')
    db.make_room('HURCTR 117', 19, [])
    db.make_building('AIP')
    db.make_room('AIP 139', 24, [])

