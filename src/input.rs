use super::bits::*;
use super::solver::Solver;

pub fn setup() -> Result<Solver, String> {
    let mut solver = super::data::input()?;
    solver.missing.sort();
    solver.missing.dedup();

    // compute the transitive closure of all prereqs
    // we will do it the easy/dumb way and just iterate until it converges
    let mut changed = true;
    while changed {
        changed = false;

        for sec_i in 0..solver.input_sections.len() {
            let mut new_list = Vec::new();
            for &pre in &solver.input_sections[sec_i].prereqs {
                // keep the prereq
                new_list.push(pre);

                // add the prereq's prereqs
                for &elt in &solver.input_sections[pre].prereqs {
                    new_list.push(elt);
                }

                // and the prereq's coreqs
                for &elt in &solver.input_sections[pre].coreqs {
                    new_list.push(elt);
                }
            }
            for &co in &solver.input_sections[sec_i].coreqs {
                // and the coreq's prereqs
                for &elt in &solver.input_sections[co].prereqs {
                    new_list.push(elt);
                }
            }

            // but filter out the coreqs themselves
            new_list.retain(|elt| !solver.input_sections[sec_i].coreqs.contains(elt));

            new_list.sort();
            new_list.dedup();
            if new_list.len() != solver.input_sections[sec_i].prereqs.len() {
                changed = true;
                solver.input_sections[sec_i].prereqs = new_list;
            } else {
                for i in 0..new_list.len() {
                    if new_list[i] != solver.input_sections[sec_i].prereqs[i] {
                        changed = true;
                        solver.input_sections[sec_i].prereqs = new_list;
                        break;
                    }
                }
            }
        }
    }

    // remove all conflicts between courses and their prereqs
    for sec_i in 0..solver.input_sections.len() {
        for pre_i in 0..solver.input_sections[sec_i].prereqs.len() {
            let prereq = solver.input_sections[sec_i].prereqs[pre_i];

            // delete the conflict unless it is marked as a hard conflict
            if (1..=99).contains(&solver.input_sections[sec_i].get_conflict(prereq)) {
                solver.input_sections[sec_i].set_conflict(prereq, 0);
                solver.input_sections[prereq].set_conflict(sec_i, 0);
            }
        }
    }

    // add hard conflicts between all the sections an instructor teaches
    for instructor in &solver.instructors {
        for &left in &instructor.sections {
            // we only care about primary cross listings, so ignore others
            if !solver.input_sections[left].cross_listings.is_empty()
                && solver.input_sections[left].cross_listings[0] != left
            {
                continue;
            }
            for &right in &instructor.sections {
                // we only care about primary cross listings, so ignore others
                if !solver.input_sections[right].cross_listings.is_empty()
                    && solver.input_sections[right].cross_listings[0] != right
                {
                    continue;
                }
                if left == right {
                    continue;
                };
                solver.input_sections[left].set_conflict(right, 100);
            }
        }
    }

    // handle courses with multiple sections:
    // 1. remove/relax all conflicts involving those sections (except hard conflicts)
    // 2. make sections of the same course hard conflicts with each other
    // 3. scoring criteria to spread sections across morning/afternoon, mw/tr?
    let mut counts = std::collections::HashMap::<String, usize>::new();
    for sec in &solver.input_sections {
        let key = format!("{} {}", sec.prefix, sec.course);
        *counts.entry(key).or_default() += 1;
    }
    counts.retain(|_, v| *v > 1);
    //for course_raw in counts.keys() {
    //println!("{} has multiple sections", course_raw);
    //}

    // compute time slot conflict lookup table
    // used by time_slots_conflict
    for i in 0..solver.time_slots.len() {
        for j in 0..solver.time_slots.len() {
            solver.time_slot_conflicts.push(solver.time_slots[i].conflicts.contains(&j));
        }
    }

    Ok(solver)
}

// cross a holiday off the 5-minute interval list for the semester
pub fn make_holiday(solver: &mut Solver, holiday: time::Date) -> Result<(), String> {
    assert!(!solver.input_locked);

    if holiday < solver.start || holiday > solver.end {
        return Err(format!(
            "block_out_holiday: {} is outside the term",
            holiday
        ));
    }
    let mut index = ((holiday - solver.start).whole_days() * 24 * 60 / 5) as usize;
    for _hour in 0..24 {
        for _min in (0..60).step_by(5) {
            solver.slots.set(index, false).unwrap();
            index += 1;
        }
    }
    Ok(())
}

// create a time slot from user input
pub fn make_time(solver: &mut Solver, name: &str, tags: Vec<&str>) -> Result<(), String> {
    assert!(!solver.input_locked);

    // example: MWF0900+50
    let re = regex::Regex::new(
        r"^([mtwrfsuMTWRFSU]+)([0-1][0-9]|2[0-3])([0-5][05])\+([1-9][0-9]?[05])$",
    )
    .unwrap();

    if solver.time_slots.iter().any(|elt| elt.name == *name) {
        return Err(format!("cannot have two time slots with name \"{}\"", name));
    }

    let Some(caps) = re.captures(name) else {
        return Err(format!(
            "unrecognized time format: '{}' should be like 'MWF0900+50'",
            name
        ));
    };
    let weekday_part = &caps[1];
    let hour_part = &caps[2];
    let minute_part = &caps[3];
    let length_part = &caps[4];

    // extract days of week
    let days = parse_days(weekday_part)?;

    // get start time
    let start_hour = hour_part.parse::<u8>().unwrap();
    let start_minute = minute_part.parse::<u8>().unwrap();
    let start_time = time::Time::from_hms(start_hour, start_minute, 0).unwrap();
    let length = length_part.parse::<i64>().unwrap();
    let duration = time::Duration::minutes(length);

    // set up the vector of 5-minute intervals used over the term
    let mut slots = Bits::new(date_range_slots(solver.start, solver.end));
    let mut i = 0;
    let mut day = solver.start;
    while day <= solver.end {
        let weekday = day.weekday();
        let active_day = days.contains(&weekday);
        let mut minutes_left = 0;
        for hour in 0..24 {
            for min in (0..60).step_by(5) {
                if active_day && start_hour == hour && start_minute == min {
                    minutes_left = length;
                }
                slots.set(i, minutes_left > 0).unwrap();
                i += 1;
                if minutes_left > 0 {
                    minutes_left -= 5;
                }
            }
        }
        day = day.next_day().unwrap();
    }
    slots.intersect_in_place(&solver.slots)?;

    // check for conflicts
    let mut conflicts = Vec::new();
    let my_index = solver.time_slots.len();
    for (other_index, other) in solver.time_slots.iter_mut().enumerate() {
        if !slots.is_disjoint(&other.slots)? {
            conflicts.push(other_index);
            other.conflicts.push(my_index);
        }
    }

    // a time slot always conflicts with itself
    conflicts.push(solver.time_slots.len());

    solver.time_slots.push(TimeSlot {
        name: name.into(),
        slots,
        days,
        start_time,
        duration,
        conflicts,
        tags: tags.iter().map(|s| s.to_string()).collect(),
    });

    Ok(())
}

pub fn make_room(solver: &mut Solver, name: &str, cap: u16, tags: Vec<&str>) -> Result<(), String> {
    assert!(!solver.input_locked);

    if cap == 0 {
        return Err(format!("room {name} must have capacity > 0"));
    }
    if cap > 10000 {
        return Err(format!("room {name} cannot have capacity > 10000"));
    }
    let name_s = name.to_string();
    if solver.rooms.iter().any(|elt| elt.name == *name) {
        return Err(format!("cannot have two rooms with name \"{}\"", name_s));
    }
    solver.rooms.push(Room {
        name: name_s,
        capacity: cap,
        tags: tags.iter().map(|s| s.to_string()).collect(),
    });
    Ok(())
}

pub fn make_instructor(solver: &mut Solver, name: &str, available_times_raw: Vec<(&str, isize)>,) -> Result<(), String> {
    assert!(!solver.input_locked);

    if find_instructor_by_name(solver, name).is_ok() {
        return Err(format!("duplicate instructor name: {}", name));
    }

    let mut available_times = Vec::new();
    for _ in 0..7 {
        available_times.push(vec![-1isize; 24 * 12]);
    }

    // example: MTWRF 0900-1700
    let re = regex::Regex::new(
        r"^([mtwrfsuMTWRFSU]+) *([0-1][0-9]|2[0-3])([0-5][05])-([0-1][0-9]|2[0-4])([0-5][05])$",
    )
    .unwrap();

    for (time_name, penalty) in available_times_raw {
        if !(0..=99).contains(&penalty) {
            return Err(format!(
                "instructor {} cannot have an available time penalty of {}",
                name, penalty
            ));
        }

        let Some(caps) = re.captures(time_name) else {
            return Err(format!(
                "unrecognized time format '{}' should be like 'MTWRF 0900-1700' for instructor {}", time_name, name));
        };

        let days = parse_days(&caps[1])?;
        let start_hour = caps[2].parse::<usize>().unwrap();
        let start_minute = caps[3].parse::<usize>().unwrap();
        let end_hour = caps[4].parse::<usize>().unwrap();
        let end_minute = caps[5].parse::<usize>().unwrap();

        if end_hour == 24 && end_minute != 0 {
            return Err(format!(
                "available time for instructor {} cannot end after midnight",
                name
            ));
        }

        let start_index = start_hour * 12 + start_minute / 5;
        let end_index = end_hour * 12 + end_minute / 5;
        if end_index <= start_index {
            return Err(format!(
                "available time for instructor {} cannot end before it begins",
                name
            ));
        }
        for &day_of_week in &days {
            let day = &mut available_times[day_of_week.number_days_from_sunday() as usize];
            for elt in day.iter_mut().take(end_index).skip(start_index) {
                *elt = std::cmp::max(*elt, penalty);
            }
        }
    }

    solver.instructors.push(Instructor {
        name: name.into(),
        available_times,
        sections: Vec::new(),
        distribution: Vec::new(),
    });
    Ok(())
}

pub fn make_section(solver: &mut Solver, section_raw: &str, instructor_names: Vec<&str>, rooms_and_times: Vec<(&str, isize)>,) -> Result<(), String> {
    assert!(!solver.input_locked);

    let (prefix, course, Some(section)) = parse_section_name(section_raw)? else {
        return Err(format!(
            "section name {section_raw} must include prefix, course, and section, like 'CS 1400-01'"
        ));
    };

    // start with instructors
    let mut instructors = Vec::new();
    for name in &instructor_names {
        instructors.push(find_instructor_by_name(solver, name)?);
    }
    instructors.sort();
    instructors.dedup();

    // handle constraints
    let mut rwp = Vec::new();
    let mut twp = Vec::new();

    for (t, badness) in rooms_and_times {
        let tag = t.to_string();
        if !(-1..=99).contains(&badness) {
            return Err(format!(
                "section {} cannot have a room/time penalty of {}",
                section_raw, badness
            ));
        }
        let mut found = false;

        // check for matching rooms
        for (room_i, _room) in solver.rooms.iter().enumerate().filter(|(_, r)| r.name == tag || r.tags.contains(&tag)) {
            found = true;
            add_room_with_penalty_keep_last(
                &mut rwp,
                RoomWithPenalty {
                    room: room_i,
                    penalty: badness,
                },
            );
        }

        // check for matching times
        for (time_i, _time) in solver.time_slots.iter().enumerate().filter(|(_, t)| t.name == tag || t.tags.contains(&tag)) {
            found = true;
            add_time_with_penalty_keep_last(
                &mut twp,
                TimeWithPenalty {
                    time_slot: time_i,
                    penalty: badness,
                },
            );
        }

        if !found {
            return Err(format!("unrecognized constraint tag for section: {}", tag));
        }
    }

    if rwp.is_empty() {
        return Err(format!("no rooms found for {}", section_raw));
    }
    if twp.is_empty() {
        return Err(format!(
            "section {} does not specify any time slots",
            section_raw
        ));
    }

    rwp.sort_by_key(|elt| elt.room);
    twp.sort_by_key(|elt| elt.time_slot);
    if solver.input_sections.iter().any(|s| s.prefix == prefix && s.course == course && s.section == section) {
        return Err(format!("section {} appears more than once", section_raw));
    }
    for &instructor in &instructors {
        solver.instructors[instructor].sections.push(solver.input_sections.len());
    }
    solver.input_sections.push(InputSection {
        prefix,
        course,
        section,
        instructors,
        rooms: rwp,
        time_slots: twp,
        hard_conflicts: Vec::new(),
        soft_conflicts: Vec::new(),
        cross_listings: Vec::new(),
        coreqs: Vec::new(),
        prereqs: Vec::new(),
    });

    Ok(())
}

// set the conflict penalty between every pair of sections in the list
//
// a subtlety: if a course is specified without a section and resolves
// to multiple sections, the penalty between those sections will be unchanged.
// e.g.: specifying CS 101 and CS 102 will set the conflict between every
// section of CS 101 vs every CS 102, but not between the individual
// sections of CS 101 nor between the individual sections of CS 102
pub fn make_conflict_clique(solver: &mut Solver, badness: isize, maximize: bool, sections_raw: Vec<&str>) -> Result<(), String> {
    assert!(!solver.input_locked);

    // parse and sanity check the inputs
    if !(0..=100).contains(&badness) {
        return Err("badness for a conflict clique must be between 0 and 100".into());
    }
    if badness == 100 && !maximize {
        return Err("make_conflict_clique does not support badness=100 and !maximize".into());
    } else if badness == 0 && maximize {
        return Err("make_conflict_clique does not support badness=0 and maximize".into());
    }
    let mut courses = Vec::new();
    for section_raw in sections_raw {
        let list = find_sections_by_name(solver, section_raw)?;
        if list.is_empty() {
            solver.missing.push(section_raw.to_string());
        } else {
            courses.push(list);
        }
    }

    // process every pairing (except element vs self)
    for (left_course_i, left_course) in courses.iter().enumerate() {
        for (right_course_i, right_course) in courses.iter().enumerate() {
            if left_course_i == right_course_i {
                continue;
            }

            for &left_section_i in left_course {
                for &right_section_i in right_course {
                    let left = &mut solver.input_sections[left_section_i];
                    let old = left.get_conflict(right_section_i);
                    left.set_conflict(
                        right_section_i,
                        if maximize {
                            std::cmp::max(old, badness)
                        } else {
                            std::cmp::min(old, badness)
                        },
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn make_anti_conflict(solver: &mut Solver, badness: isize, single_raw: &str, group_raw: Vec<&str>) -> Result<(), String> {
    assert!(!solver.input_locked);

    // parse and sanity check the inputs
    if !(1..100).contains(&badness) {
        return Err("badness for an anticonflict must be between 1 and 99".into());
    }

    // look up the group sections first
    let mut group = Vec::new();
    for raw in group_raw {
        let mut list = find_sections_by_name(solver, raw)?;
        if list.is_empty() {
            solver.missing.push(raw.to_string());
        } else {
            group.append(&mut list);
        }
    }

    // see if the single section is present in the data
    let single_list = find_sections_by_name(solver, single_raw)?;
    if single_list.is_empty() {
        solver.missing.push(single_raw.to_string());
    }

    // single must be a single section
    if single_list.len() > 1 {
        return Err(format!(
            "anticonflict: single {} must be a single section",
            single_raw
        ));
    }

    // single and group must both exist
    if group.is_empty() || single_list.len() != 1 {
        return Ok(());
    }
    let single = single_list[0];
    group.sort();
    group.dedup();

    solver.anticonflicts.push((badness, single, group));

    Ok(())
}

pub fn add_prereqs_fn(solver: &mut Solver, course_raw: &str, coreqs_raw: Vec<&str>, prereqs_raw: Vec<&str>) -> Result<(), String> {
    assert!(!solver.input_locked);

    // see if the course is present in the data
    let course_list = find_sections_by_name(solver, course_raw)?;
    if course_list.is_empty() {
        solver.missing.push(course_raw.to_string());
    }

    // look up the coreq sections
    let mut coreqs = Vec::new();
    for raw in coreqs_raw {
        let mut list = find_sections_by_name(solver, raw)?;
        if list.is_empty() {
            solver.missing.push(raw.to_string());
        } else {
            coreqs.append(&mut list);
        }
    }

    // look up the prereq sections
    let mut prereqs = Vec::new();
    for raw in prereqs_raw {
        let mut list = find_sections_by_name(solver, raw)?;
        if list.is_empty() {
            solver.missing.push(raw.to_string());
        } else {
            prereqs.append(&mut list);
        }
    }

    coreqs.sort();
    coreqs.dedup();
    for course in course_list {
        let cr = &mut solver.input_sections[course].coreqs;
        for &elt in &coreqs {
            cr.push(elt);
        }
        cr.sort();
        cr.dedup();

        let pr = &mut solver.input_sections[course].prereqs;
        for &elt in &prereqs {
            pr.push(elt);
        }
        pr.sort();
        pr.dedup();
    }

    Ok(())
}

pub fn multiple_sections_reduce_penalties_fn(solver: &mut Solver, courses_raw: Vec<(&str, isize)>) -> Result<(), String> {
    assert!(!solver.input_locked);

    let threshold = 30;
    for (course_raw, online) in courses_raw {
        // get the sections of this course
        let course_list = find_sections_by_name(solver, course_raw)?;
        if course_list.is_empty() {
            solver.missing.push(course_raw.to_string());
            continue;
        }
        let number = (course_list.len() as isize) + online;
        if number == 1 {
            // nothing to do
            continue;
        }

        // find all the soft conflicts involving these sections
        for sec_i in course_list {
            let others: Vec<usize> = solver.input_sections[sec_i]
                .soft_conflicts
                .iter()
                .map(|elt| elt.section)
                .collect();
            for other in others {
                let old_score = solver.input_sections[sec_i].get_conflict(other);
                if old_score >= 100 || old_score <= 0 {
                    continue;
                }
                let mut new_score =
                    (solver.input_sections[sec_i].get_conflict(other) - 1) / (number + 1);
                if new_score < threshold {
                    new_score = 0;
                }

                // set in both directions
                solver.input_sections[sec_i].set_conflict(other, new_score);
                solver.input_sections[other].set_conflict(sec_i, new_score);
            }
        }
    }

    Ok(())
}

pub fn make_cross_listing(solver: &mut Solver, sections_raw: Vec<&str>) -> Result<(), String> {
    assert!(!solver.input_locked);

    let mut sections = Vec::new();
    for section_raw in &sections_raw {
        let section_list = find_sections_by_name(solver, section_raw)?;
        if section_list.len() != 1 {
            return Err(format!("section {section_raw} not found in cross-listing"));
        }

        sections.push(section_list[0]);
    }
    sections.sort();
    sections.dedup();
    if sections.len() < 2 {
        return Err(format!(
            "cross-listing that includes {} must include at least two unique sections",
            solver.input_sections[sections[0]].get_name()
        ));
    }
    for &i in &sections {
        if !solver.input_sections[i].cross_listings.is_empty() {
            return Err(format!(
                "cannot cross list {} because it is already cross-listed",
                solver.input_sections[i].get_name()
            ));
        }
        solver.input_sections[i].cross_listings = sections.clone();
    }

    Ok(())
}

pub fn find_time_slot_by_name(solver: &Solver, name: &str) -> Result<usize, String> {
    let Some(i) = solver.time_slots.iter().position(|elt| elt.name == *name) else {
        return Err(format!("timeslot named \"{}\" not found", name));
    };
    Ok(i)
}

pub fn find_room_by_name(solver: &Solver, name: &str) -> Result<usize, String> {
    let Some(i) = solver.rooms.iter().position(|elt| elt.name == *name) else {
        return Err(format!("room named \"{}\" not found", name));
    };
    Ok(i)
}

pub fn find_sections_by_name(solver: &Solver, course_raw: &str) -> Result<Vec<usize>, String> {
    let (prefix, course, section) = parse_section_name(course_raw)?;
    let mut list = Vec::new();
    solver.input_sections.iter().enumerate().for_each(|(i, s)| {
        if s.prefix == *prefix && s.course == *course {
            match &section {
                None => list.push(i),
                Some(name) if *name == s.section => list.push(i),
                _ => (),
            }
        }
    });
    Ok(list)
}

pub fn find_instructor_by_name(solver: &Solver, name: &str) -> Result<usize, String> {
    let Some(i) = solver.instructors.iter().position(|elt| elt.name == *name) else {
        return Err(format!("instructor named \"{}\" not found", name));
    };
    Ok(i)
}

pub fn time_slots_conflict(solver: &Solver, a: usize, b: usize) -> bool {
    solver.time_slot_conflicts[a * solver.time_slots.len() + b]
    //solver.time_slots[a].conflicts.contains(&b)
}

pub fn is_primary_cross_listing(solver: &Solver, index: usize) -> bool {
    solver.input_sections[index].cross_listings.is_empty()
        || index == solver.input_sections[index].cross_listings[0]
}

pub fn get_primary_cross_listing(solver: &Solver, index: usize) -> usize {
    if solver.input_sections[index].cross_listings.is_empty() {
        return index;
    }
    solver.input_sections[index].cross_listings[0]
}

pub fn parse_days(weekday_raw: &str) -> Result<Vec<time::Weekday>, String> {
    let mut days = Vec::new();
    for day in weekday_raw.chars() {
        match day {
            'm' | 'M' => days.push(time::Weekday::Monday),
            't' | 'T' => days.push(time::Weekday::Tuesday),
            'w' | 'W' => days.push(time::Weekday::Wednesday),
            'r' | 'R' => days.push(time::Weekday::Thursday),
            'f' | 'F' => days.push(time::Weekday::Friday),
            's' | 'S' => days.push(time::Weekday::Saturday),
            'u' | 'U' => days.push(time::Weekday::Sunday),
            _ => {
                return Err(format!(
                    "Unknown day of week in {}: I only understand mtwrfsu",
                    weekday_raw
                ))
            }
        }
    }
    Ok(days)
}

pub fn parse_section_name(name: &str) -> Result<(String, String, Option<String>), String> {
    // example CS 1400-01
    let re = regex::Regex::new(r"^([^ ]+) ([^- ]+)(?:-([^ ]+))?$").unwrap();
    let Some(caps) = re.captures(name) else {
        return Err(format!(
            "unrecognized section name format: '{}' should be like 'CS 1400-01' or 'CS 1400'",
            name
        ));
    };
    let prefix_part = caps.get(1).unwrap().as_str().to_string().to_uppercase();
    let course_part = caps.get(2).unwrap().as_str().to_string().to_uppercase();
    let section_part = caps.get(3).map(|s| s.as_str().to_string().to_uppercase());
    Ok((prefix_part, course_part, section_part))
}

pub fn date_range_slots(start: time::Date, end: time::Date) -> usize {
    let size = ((end - start).whole_days() + 1) * 24 * 60 / 5;
    if size <= 0 {
        panic!("date_range_slots must have start < end");
    }
    size as usize
}

pub fn add_room_with_penalty_keep_worst(list: &mut Vec<RoomWithPenalty>, rwp: RoomWithPenalty) {
    match list.iter().position(|elt| elt.room == rwp.room) {
        Some(i) => list[i].penalty = std::cmp::max(list[i].penalty, rwp.penalty),
        None => list.push(rwp),
    }
}

pub fn add_time_with_penalty_keep_worst(list: &mut Vec<TimeWithPenalty>, twp: TimeWithPenalty) {
    match list.iter().position(|elt| elt.time_slot == twp.time_slot) {
        Some(i) => list[i].penalty = std::cmp::max(list[i].penalty, twp.penalty),
        None => list.push(twp),
    }
}

pub fn add_room_with_penalty_keep_last(list: &mut Vec<RoomWithPenalty>, rwp: RoomWithPenalty) {
    if rwp.penalty < 0 {
        list.retain(|elt| elt.room != rwp.room);
    } else {
        match list.iter().position(|elt| elt.room == rwp.room) {
            Some(i) => list[i].penalty = rwp.penalty,
            None => list.push(rwp),
        }
    }
}

pub fn add_time_with_penalty_keep_last(list: &mut Vec<TimeWithPenalty>, twp: TimeWithPenalty) {
    if twp.penalty < 0 {
        list.retain(|elt| elt.time_slot != twp.time_slot);
    } else {
        match list.iter().position(|elt| elt.time_slot == twp.time_slot) {
            Some(i) => list[i].penalty = twp.penalty,
            None => list.push(twp),
        }
    }
}

pub fn date(year: i32, month: u8, day: u8) -> Result<time::Date, String> {
    let Ok(m) = time::Month::try_from(month) else {
        return Err(format!("date input with invalid month {}", month));
    };
    let Ok(d) = time::Date::from_calendar_date(year, m, day) else {
        return Err(format!(
            "date {}-{}-{} is invalid, should be year-month-day",
            year, month, day
        ));
    };
    Ok(d)
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub capacity: u16,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct TimeSlot {
    pub name: String,
    pub slots: Bits,
    pub days: Vec<time::Weekday>,
    pub start_time: time::Time,
    pub duration: time::Duration,

    // a list of all overlapping time slots
    // a time slot IS always in its own conflict list
    pub conflicts: Vec<usize>,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct RoomTime {
    pub room: usize,
    pub time_slot: usize,
}

#[derive(Clone)]
pub struct TimeWithPenalty {
    pub time_slot: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct RoomWithPenalty {
    pub room: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct RoomTimeWithPenalty {
    pub room: usize,
    pub time_slot: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct SectionWithPenalty {
    pub section: usize,
    pub penalty: isize,
}

#[derive(Clone)]
pub struct Instructor {
    pub name: String,

    // one entry per day of the week (time::Weekday::number_days_from_sunday())
    // each day has a list of penalty values for each 5-minute slot,
    // with -1 meaning impossible
    pub available_times: Vec<Vec<isize>>,
    pub sections: Vec<usize>,
    pub distribution: Vec<DistributionPreference>,
}

impl Instructor {
    pub fn get_time_slot_penalty(&self, time_slot: &TimeSlot) -> Option<isize> {
        let start_hour = time_slot.start_time.hour() as usize;
        let start_minute = time_slot.start_time.minute() as usize;
        let minutes = time_slot.duration.whole_minutes() as usize;

        let mut penalty = 0;
        for &day_of_week in &time_slot.days {
            let day = &self.available_times[day_of_week.number_days_from_sunday() as usize];
            let start_index = start_hour * 12 + start_minute / 5;
            let end_index = start_index + minutes / 5;
            for &elt in day.iter().take(end_index).skip(start_index) {
                if elt < 0 {
                    return None;
                }
                penalty = std::cmp::max(penalty, elt);
            }
        }
        Some(penalty)
    }
}

#[derive(Clone)]
pub enum DistributionPreference {
    // classes on the same day should occur in clusters with tidy gaps between them
    Clustering {
        days: Vec<time::Weekday>,
        max_gap: time::Duration,
        cluster_limits: Vec<DurationWithPenalty>,
        gap_limits: Vec<DurationWithPenalty>,
    },

    // zero or more days from the list should be free of classes
    DaysOff {
        days: Vec<time::Weekday>,
        days_off: u8,
        penalty: isize,
    },

    // days that have classes should have the same number of classes
    DaysEvenlySpread {
        days: Vec<time::Weekday>,
        penalty: isize,
    },
}

#[derive(Clone)]
pub enum DurationWithPenalty {
    // a duration shorter than this gets a penalty
    TooShort {
        duration: time::Duration,
        penalty: isize,
    },

    // a duration longer than this gets a penalty
    TooLong {
        duration: time::Duration,
        penalty: isize,
    },
}

#[derive(Clone)]
pub struct InputSection {
    // prefix, e.g.: "CS"
    pub prefix: String,
    // course name, e.g.: "2810"
    pub course: String,
    // section name, e.g.: "01"
    pub section: String,

    // includes only the instructors explicity assigned to this section (not cross-listings)
    pub instructors: Vec<usize>,

    // rooms and times as input
    pub rooms: Vec<RoomWithPenalty>,
    pub time_slots: Vec<TimeWithPenalty>,

    // hard conflicts that named this section directly, not including cross-listings
    pub hard_conflicts: Vec<usize>,

    // soft conflicts that named this section directly, not including cross-listings
    pub soft_conflicts: Vec<SectionWithPenalty>,

    // if the course is cross listed it this list will have all cross-listed sections
    // in numerical order, and the first one is the canonical section (others tag along)
    // empty implies no cross listing
    pub cross_listings: Vec<usize>,

    // direct prereqs are recorded here
    // the transitive closure of prereqs is used to remove conflicts between classes
    // that cannot be taken together
    // note: the prereqs of a coreq are treated like direct prereqs
    // and the coreqs of a prereq are treated like direct prereqs
    //
    // if a course is a coreq (and optionally also a prereq) then we do nothing
    // directly but it will affect courses that require this one
    pub coreqs: Vec<usize>,
    pub prereqs: Vec<usize>,
}

impl InputSection {
    pub fn get_conflict(&self, other: usize) -> isize {
        for elt in &self.hard_conflicts {
            if *elt == other {
                return 100;
            }
        }
        for elt in &self.soft_conflicts {
            if elt.section == other {
                return elt.penalty;
            }
        }
        0
    }

    pub fn set_conflict(&mut self, other: usize, penalty: isize) {
        assert!((0..=100).contains(&penalty));
        if penalty == 0 {
            self.hard_conflicts.retain(|&elt| elt != other);
            self.soft_conflicts.retain(|elt| elt.section != other);
        } else if penalty == 100 {
            if !self.hard_conflicts.iter().any(|&elt| elt == other) {
                self.hard_conflicts.push(other);
            }
            self.soft_conflicts.retain(|elt| elt.section != other);
        } else {
            self.hard_conflicts.retain(|&elt| elt != other);
            match self
                .soft_conflicts
                .iter()
                .position(|elt| elt.section == other)
            {
                Some(i) => self.soft_conflicts[i].penalty = penalty,
                None => self.soft_conflicts.push(SectionWithPenalty {
                    section: other,
                    penalty,
                }),
            }
        }
    }

    pub fn get_name(&self) -> String {
        format!("{} {}-{}", self.prefix, self.course, self.section)
    }
}

macro_rules! holiday {
    ($input:expr, $year:expr, $month:expr, $day:expr) => {
        make_holiday($input, date($year, $month, $day)?)?
    };
}

macro_rules! room {
    ($input:expr,
            name: $name:expr,
            capacity: $capacity:expr
            $(, tags: $($tag:expr),+)?) => (
        make_room($input,
            $name,
            $capacity,
            vec![$($($tag,)+)?])?
    );
}

macro_rules! time {
    ($input:expr,
            name: $name:literal
            $(, tags: $($tag:literal),+)?) => (
        make_time($input,
            $name,
            vec![$($($tag,)+)?])?
    );
}

macro_rules! name_with_optional_penalty {
    ($name:literal with penalty $pen:literal) => {
        ($name, $pen)
    };
    ($name:literal) => {
        ($name, 0)
    };
}

macro_rules! instructor {
    ($input:expr,
            name: $name:expr,
            available: $($tag:literal $(with penalty $pen:literal)?),+ $(,)?) => {
        make_instructor($input,
            $name,
            vec![ $(name_with_optional_penalty!($tag $(with penalty $pen)?),)+ ]
        )?
    };
}

macro_rules! section {
    ($input:expr,
            course: $section:literal,
            $(instructor: $inst:literal $(and $insts:literal)*,)?
            rooms and times: $($tag:literal $(with penalty $pen:literal)?),+ $(,)?) => {
        make_section($input,
            $section,
            vec![ $($inst, $($insts, )*)? ],
            vec![ $(name_with_optional_penalty!($tag $(with penalty $pen)?),)+ ]
        )?
    };
}

macro_rules! crosslist {
    ($input:expr,
            $section:literal
            $(cross-list with $sections:literal)+) => {
        make_cross_listing($input, vec![
            $section,
            $($sections, )+
        ])?
    };
}

macro_rules! conflict {
    ($input:expr,
            set hard,
            clique: $($sections:literal),+ $(,)?) => {
        make_conflict_clique($input,
            100, true,
            vec![ $($sections, )+ ])?;
    };
    ($input:expr,
            set penalty to $penalty:expr,
            clique: $($sections:literal),+ $(,)?) => {
        make_conflict_clique($input,
            $penalty, true,
            vec![ $($sections, )+ ])?;
    };
    ($input:expr,
            remove penalty,
            clique: $($sections:literal),+ $(,)?) => {
        make_conflict_clique($input,
            0, false,
            vec![ $($sections, )+ ])?;
    };
}

macro_rules! anticonflict {
    ($input:expr,
            set penalty to $penalty:expr,
            single: $single_course:literal,
            group: $($group_course:literal),+ $(,)?) => {
        make_anti_conflict($input,
            $penalty,
            $single_course,
            vec![ $($group_course, )+ ])?;
    };
}

macro_rules! duration_penalty {
    (too short: less than $min:literal minutes incurs penalty $pen:literal) => {
        DurationWithPenalty::TooShort {
            duration: time::Duration::minutes($min),
            penalty: $pen,
        }
    };
    (too long: more than $min:literal minutes incurs penalty $pen:literal) => {
        DurationWithPenalty::TooLong {
            duration: time::Duration::minutes($min),
            penalty: $pen,
        }
    };
}

macro_rules! clustering_preferences {
    ($input:expr,
            instructor: $inst:literal,
            days: $days:literal,
            max gap within cluster: $gap:literal minutes,
            $(cluster too $ca:ident : $cb:ident than $cmin:literal minutes incurs penalty $cpen:literal),*,
            $(gap too $ga:ident : $gb:ident than $gmin:literal minutes incurs penalty $gpen:literal),* $(,)?) => {

        let i = find_instructor_by_name($input, $inst)?;
        let cluster_limits = vec![$(duration_penalty!(too $ca: $cb than $cmin minutes incurs penalty $cpen)),+];
        let gap_limits = vec![$(duration_penalty!(too $ga: $gb than $gmin minutes incurs penalty $gpen)),+];
        assert!(!cluster_limits.is_empty() || !gap_limits.is_empty());

        $input.instructors[i].distribution.push(
            DistributionPreference::Clustering {
                days: parse_days($days)?,
                max_gap: time::Duration::minutes($gap),
                cluster_limits,
                gap_limits,
            }
        );
    };
}

macro_rules! days_off_preference {
    ($input:expr,
            instructor: $inst:literal,
            days: $days:literal,
            days off: $off:literal,
            penalty: $pen:literal) => {
        let i = find_instructor_by_name($input, $inst)?;
        $input.instructors[i]
            .distribution
            .push(DistributionPreference::DaysOff {
                days: parse_days($days)?,
                days_off: $off,
                penalty: $pen,
            });
    };
}

macro_rules! evenly_spread_out_preference {
    ($input:expr,
            instructor: $inst:literal,
            days: $days:literal,
            penalty: $pen:literal) => {
        let i = find_instructor_by_name($input, $inst)?;
        $input.instructors[i]
            .distribution
            .push(DistributionPreference::DaysEvenlySpread {
                days: parse_days($days)?,
                penalty: $pen,
            });
    };
}

// default instructor distribution preferences
macro_rules! default_clustering {
    ($input:expr,
            instructor: $inst:literal,
            days: $days:literal,
            days off: $off:literal) => {
        clustering_preferences!($input,
            instructor: $inst,
            days: $days,
            max gap within cluster: 15 minutes,
            cluster too short: less than 110 minutes incurs penalty 5,
            cluster too long: more than 165 minutes incurs penalty 10,
            gap too short: less than 60 minutes incurs penalty 10,
            gap too long: more than 105 minutes incurs penalty 5,
            gap too long: more than 195 minutes incurs penalty 10);
        days_off_preference!($input,
            instructor: $inst,
            days: $days,
            days off: $off,
            penalty: 10);
        evenly_spread_out_preference!($input,
            instructor: $inst,
            days: $days,
            penalty: 10);
    };
    ($input:expr,
            instructor: $inst:literal,
            days: $days:literal) => {
        clustering_preferences!($input,
            instructor: $inst,
            days: $days,
            max gap within cluster: 15 minutes,
            cluster too short: less than 110 minutes incurs penalty 5,
            cluster too long: more than 165 minutes incurs penalty 10,
            gap too short: less than 60 minutes incurs penalty 10,
            gap too long: more than 105 minutes incurs penalty 5,
            gap too long: more than 195 minutes incurs penalty 10);
        evenly_spread_out_preference!($input,
            instructor: $inst,
            days: $days,
            penalty: 10);
    };
}

macro_rules! add_prereqs {
    ($input:expr,
            course: $course:literal,
            coreqs: $($coreqs:literal),+,
            prereqs: $($prereqs:literal),+ $(,)?) => {
        add_prereqs_fn($input, $course, vec![ $($coreqs, )+ ], vec![ $($prereqs, )+ ])?;
    };
    ($input:expr,
            course: $course:literal,
            coreqs: $($coreqs:literal),+ $(,)?) => {
        add_prereqs_fn($input, $course, vec![ $($coreqs, )+ ], vec![])?;
    };
    ($input:expr,
            course: $course:literal,
            prereqs: $($prereqs:literal),+ $(,)?) => {
        add_prereqs_fn($input, $course, vec![], vec![ $($prereqs, )+ ])?;
    };
}

macro_rules! course_with_online {
    ($course:literal with $online:literal online) => {
        ($course, $online)
    };
    ($course:literal) => {
        ($course, 0)
    };
}

macro_rules! multiple_sections_reduce_penalties {
    ($input:expr,
            courses: $($course:literal $(with $online:literal online)?),+ $(,)?) => {
        multiple_sections_reduce_penalties_fn($input, vec![ $(course_with_online!($course $(with $online online)?),)+ ])?;
    };
}

pub(crate) use {
    add_prereqs, anticonflict, clustering_preferences, conflict, course_with_online, crosslist,
    days_off_preference, default_clustering, duration_penalty, evenly_spread_out_preference,
    holiday, instructor, multiple_sections_reduce_penalties, name_with_optional_penalty, room,
    section, time,
};
