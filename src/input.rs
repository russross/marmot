use super::bits::*;

pub fn setup() -> Result<Input, String> {
    let mut input = super::data::input()?;
    input.missing.sort();
    input.missing.dedup();

    // add hard conflicts between all the sections an instructor teaches
    for instructor in &input.instructors {
        for &left in &instructor.sections {
            // we only care about primary cross listings, so ignore others
            if !input.sections[left].cross_listings.is_empty()
                && input.sections[left].cross_listings[0] != left
            {
                continue;
            }
            for &right in &instructor.sections {
                // we only care about primary cross listings, so ignore others
                if !input.sections[right].cross_listings.is_empty()
                    && input.sections[right].cross_listings[0] != right
                {
                    continue;
                }
                if left == right {
                    continue;
                };
                input.sections[left].set_conflict(right, 100);
            }
        }
    }

    // compute time slot conflict lookup table
    // used by time_slots_conflict
    for i in 0..input.time_slots.len() {
        for j in 0..input.time_slots.len() {
            input
                .time_slot_conflicts
                .push(input.time_slots[i].conflicts.contains(&j));
        }
    }

    Ok(input)
}

pub struct Input {
    // the name of the term
    pub name: String,

    // the start and end dates (inclusive) of the term
    pub start: time::Date,
    pub end: time::Date,

    // every 5-minute interval during the semester, with holidays blocked out
    pub slots: Bits,

    // core schedule data
    pub rooms: Vec<Room>,
    pub time_slots: Vec<TimeSlot>,
    pub instructors: Vec<Instructor>,
    pub sections: Vec<Section>,

    // list of sections mentioned in conflict/scoring but not actually defined
    // note that a section must be created before any references to it are valid
    pub missing: Vec<String>,

    // matrix of which time slots overlap which for fast lookup
    pub time_slot_conflicts: Vec<bool>,

    // scoring data
    pub anticonflicts: Vec<(isize, usize, Vec<usize>)>,
}

impl Input {
    pub fn new(name: &str, start: time::Date, end: time::Date) -> Self {
        // set up the term with 5-minute intervals
        let mut slots = Bits::new(date_range_slots(start, end));
        let mut day = start;
        let mut i = 0;
        while day <= end {
            for _hour in 0..24 {
                for _min in (0..60).step_by(5) {
                    slots.set(i, true).unwrap();
                    i += 1;
                }
            }
            day = day.next_day().unwrap();
        }
        Self {
            name: name.into(),
            start,
            end,
            slots,
            rooms: Vec::new(),
            time_slots: Vec::new(),
            instructors: Vec::new(),
            sections: Vec::new(),
            missing: Vec::new(),
            time_slot_conflicts: Vec::new(),
            anticonflicts: Vec::new(),
        }
    }

    // cross a holiday off the 5-minute interval list for the semester
    pub fn make_holiday(&mut self, holiday: time::Date) -> Result<(), String> {
        if holiday < self.start || holiday > self.end {
            return Err(format!(
                "block_out_holiday: {} is outside the term",
                holiday
            ));
        }
        let mut index = ((holiday - self.start).whole_days() * 24 * 60 / 5) as usize;
        for _hour in 0..24 {
            for _min in (0..60).step_by(5) {
                self.slots.set(index, false).unwrap();
                index += 1;
            }
        }
        Ok(())
    }

    // create a time slot from user input
    pub fn make_time(&mut self, name: &str, tags: Vec<&str>) -> Result<(), String> {
        // example: MWF0900+50
        let re = regex::Regex::new(
            r"^([mtwrfsuMTWRFSU]+)([0-1][0-9]|2[0-3])([0-5][05])\+([1-9][0-9]?[05])$",
        )
        .unwrap();

        if self.time_slots.iter().any(|elt| elt.name == *name) {
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
        let mut days = Vec::new();
        for day in weekday_part.chars() {
            match day {
                'm' | 'M' => days.push(time::Weekday::Monday),
                't' | 'T' => days.push(time::Weekday::Tuesday),
                'w' | 'W' => days.push(time::Weekday::Wednesday),
                'r' | 'R' => days.push(time::Weekday::Thursday),
                'f' | 'F' => days.push(time::Weekday::Friday),
                's' | 'S' => days.push(time::Weekday::Saturday),
                'u' | 'U' => days.push(time::Weekday::Sunday),
                _ => return Err("Unknown day of week: I only understand mtwrfsu".into()),
            }
        }

        // get start time
        let start_hour = hour_part.parse::<u8>().unwrap();
        let start_minute = minute_part.parse::<u8>().unwrap();
        let start_time = time::Time::from_hms(start_hour, start_minute, 0).unwrap();
        let length = length_part.parse::<i64>().unwrap();
        let duration = time::Duration::minutes(length);

        // set up the vector of 5-minute intervals used over the term
        let mut slots = Bits::new(date_range_slots(self.start, self.end));
        let mut i = 0;
        let mut day = self.start;
        while day <= self.end {
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
        slots.intersect_in_place(&self.slots)?;

        // check for conflicts
        let mut conflicts = Vec::new();
        let my_index = self.time_slots.len();
        for (other_index, other) in self.time_slots.iter_mut().enumerate() {
            if !slots.is_disjoint(&other.slots)? {
                conflicts.push(other_index);
                other.conflicts.push(my_index);
            }
        }

        // a time slot always conflicts with itself
        conflicts.push(self.time_slots.len());

        self.time_slots.push(TimeSlot {
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

    pub fn find_time_slot_by_name(&self, name: &String) -> Result<usize, String> {
        let Some(i) = self.time_slots.iter().position(|elt| elt.name == *name) else {
            return Err(format!("timeslot named \"{}\" not found", name));
        };
        Ok(i)
    }

    pub fn make_room(&mut self, name: &str, cap: u16, tags: Vec<&str>) -> Result<(), String> {
        if cap == 0 {
            return Err(format!("room {name} must have capacity > 0"));
        }
        if cap > 10000 {
            return Err(format!("room {name} cannot have capacity > 10000"));
        }
        let name_s = name.to_string();
        if self.rooms.iter().any(|elt| elt.name == *name) {
            return Err(format!("cannot have two rooms with name \"{}\"", name_s));
        }
        self.rooms.push(Room {
            name: name_s,
            capacity: cap,
            tags: tags.iter().map(|s| s.to_string()).collect(),
        });
        Ok(())
    }

    pub fn make_instructor(
        &mut self,
        name: &str,
        available_times: Vec<(String, isize)>,
    ) -> Result<(), String> {
        let mut times: Vec<TimeWithPenalty> = Vec::new();
        for (time_name, penalty) in available_times {
            let Ok(time_slot) = self.find_time_slot_by_name(&time_name) else {
                return Err(format!("unknown time slot name {}", time_name));
            };
            if times.iter().any(|t| t.time_slot == time_slot) {
                return Err(format!(
                    "time slot {} appears twice for instructor {}",
                    time_name, name
                ));
            }
            if !(0..=99).contains(&penalty) {
                return Err(format!(
                    "penalty for instructor {} time slots must be between 0 and 99",
                    name
                ));
            }
            times.push(TimeWithPenalty { time_slot, penalty });
        }
        if self.find_instructor_by_name(&name.into()).is_ok() {
            return Err(format!("duplicate instructor name: {}", name));
        }
        self.instructors.push(Instructor {
            name: name.into(),
            available_times: times,
            sections: Vec::new(),
        });
        Ok(())
    }

    pub fn find_instructor_by_name(&self, name: &String) -> Result<usize, String> {
        let Some(i) = self.instructors.iter().position(|elt| elt.name == *name) else {
            return Err(format!("instructor named \"{}\" not found", name));
        };
        Ok(i)
    }

    pub fn make_section(
        &mut self,
        section_raw: &str,
        instructor_names: Vec<String>,
        rooms_and_times: Vec<(String, isize)>,
    ) -> Result<(), String> {
        let (course, Some(section)) = parse_section_name(section_raw)? else {
            return Err(format!(
                "section name {section_raw} must include section, like 'CS 1400-01'"
            ));
        };

        // start with instructors
        let mut instructors = Vec::new();
        for name in &instructor_names {
            instructors.push(self.find_instructor_by_name(name)?);
        }
        instructors.sort();
        instructors.dedup();

        // handle constraints
        let mut rwp = Vec::new();
        let mut twp = Vec::new();

        for (tag, badness) in rooms_and_times {
            let mut found = false;

            // check for matching rooms
            for (room_i, _room) in self
                .rooms
                .iter()
                .enumerate()
                .filter(|(_, r)| r.name == tag || r.tags.contains(&tag))
            {
                found = true;
                add_room_with_penalty_keep_worst(
                    &mut rwp,
                    RoomWithPenalty {
                        room: room_i,
                        penalty: badness,
                    },
                );
            }

            // check for matching times
            for (time_i, _time) in self
                .time_slots
                .iter()
                .enumerate()
                .filter(|(_, t)| t.name == tag || t.tags.contains(&tag))
            {
                found = true;
                add_time_with_penalty_keep_worst(
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
            if self.instructors.is_empty() {
                return Err(format!("section {} does not specify any time slots and has no instructors to inherit them from", section_raw));
            }

            // just copy the availability of the first instructor and that will be
            // intersected with other instructors below
            twp.extend(
                self.instructors[instructors[0]]
                    .available_times
                    .iter()
                    .cloned(),
            );
        }

        // calculate badness for each room/time and filter by
        // instructor availability (including badness)
        let mut room_times = Vec::new();
        'rt: for time_slot in &twp {
            let mut time_penalty = time_slot.penalty;

            // if there is no instructor, the time list will not be filtered by instructor
            // so the section list will be kept as-is
            for &instructor_i in &instructors {
                match self.instructors[instructor_i]
                    .available_times
                    .iter()
                    .find(|itwp| itwp.time_slot == time_slot.time_slot)
                {
                    Some(itwp) => time_penalty = std::cmp::max(time_penalty, itwp.penalty),
                    None => continue 'rt,
                }
            }

            // every room/time combination available for this section is
            // recorded in the list
            for room in &rwp {
                room_times.push(RoomTimeWithPenalty {
                    room: room.room,
                    time_slot: time_slot.time_slot,
                    penalty: std::cmp::min(99, room.penalty + time_penalty),
                });
            }
        }
        if room_times.is_empty() {
            return Err(format!("no valid room/time combinations found for {} after considering instructor availability",
                section_raw));
        }
        room_times.sort_by_key(|elt| (elt.room, elt.time_slot, elt.penalty));
        if self
            .sections
            .iter()
            .any(|s| s.course == course && s.section == section)
        {
            return Err(format!("section {} appears more than once", section_raw));
        }
        for &instructor in &instructors {
            self.instructors[instructor]
                .sections
                .push(self.sections.len());
        }
        self.sections.push(Section {
            course,
            section,
            instructors,
            room_times,
            hard_conflicts: Vec::new(),
            soft_conflicts: Vec::new(),
            cross_listings: Vec::new(),
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
    pub fn make_conflict_clique(
        &mut self,
        badness: isize,
        maximize: bool,
        sections_raw: Vec<&str>,
    ) -> Result<(), String> {
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
            let list = self.find_sections_by_name(section_raw)?;
            if list.is_empty() {
                self.missing.push(section_raw.to_string());
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
                        let left = &mut self.sections[left_section_i];
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

    pub fn make_anti_conflict(
        &mut self,
        badness: isize,
        single_raw: &str,
        group_raw: Vec<&str>,
    ) -> Result<(), String> {
        // parse and sanity check the inputs
        if !(1..100).contains(&badness) {
            return Err("badness for an anticonflict must be between 1 and 99".into());
        }

        // look up the group sections first
        let mut group = Vec::new();
        for raw in group_raw {
            let mut list = self.find_sections_by_name(raw)?;
            if list.is_empty() {
                self.missing.push(raw.to_string());
            } else {
                group.append(&mut list);
            }
        }

        // see if the single section is present in the data
        let single_list = self.find_sections_by_name(single_raw)?;
        if single_list.is_empty() {
            self.missing.push(single_raw.to_string());
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

        self.anticonflicts.push((badness, single, group));

        Ok(())
    }

    pub fn find_sections_by_name(&self, course_raw: &str) -> Result<Vec<usize>, String> {
        let (course, section) = parse_section_name(course_raw)?;
        let mut list = Vec::new();
        self.sections.iter().enumerate().for_each(|(i, s)| {
            if s.course == *course {
                match &section {
                    None => list.push(i),
                    Some(name) if *name == s.section => list.push(i),
                    _ => (),
                }
            }
        });
        Ok(list)
    }

    pub fn make_cross_listing(&mut self, sections_raw: Vec<&str>) -> Result<(), String> {
        let mut sections = Vec::new();
        for section_raw in &sections_raw {
            let section_list = self.find_sections_by_name(section_raw)?;
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
                self.sections[sections[0]].get_name()
            ));
        }
        for &i in &sections {
            if !self.sections[i].cross_listings.is_empty() {
                return Err(format!(
                    "cannot cross list {} because it is already cross-listed",
                    self.sections[i].get_name()
                ));
            }
            self.sections[i].cross_listings = sections.clone();
        }

        Ok(())
    }

    pub fn time_slots_conflict(&self, a: usize, b: usize) -> bool {
        self.time_slot_conflicts[a * self.time_slots.len() + b]
        //self.time_slots[a].conflicts.contains(&b)
    }

    pub fn is_primary_cross_listing(&self, index: usize) -> bool {
        self.sections[index].cross_listings.is_empty()
            || index == self.sections[index].cross_listings[0]
    }

    pub fn get_primary_cross_listing(&self, index: usize) -> usize {
        if self.sections[index].cross_listings.is_empty() {
            return index;
        }
        self.sections[index].cross_listings[0]
    }
}

pub fn parse_section_name(name: &str) -> Result<(String, Option<String>), String> {
    // example CS 1400-01
    let re = regex::Regex::new(r"^([^ ]+ [^- ]+)(?:-([^ ]+))?$").unwrap();
    let Some(caps) = re.captures(name) else {
        return Err(format!(
            "unrecognized section name format: '{}' should be like 'CS 1400-01' or 'CS 1400'",
            name
        ));
    };
    let course_part = caps.get(1).unwrap().as_str().to_string();
    let section_part = caps.get(2).map(|s| s.as_str().to_string());
    Ok((course_part, section_part))
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

pub struct Room {
    pub name: String,
    pub capacity: u16,
    pub tags: Vec<String>,
}

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

pub struct Instructor {
    pub name: String,
    pub available_times: Vec<TimeWithPenalty>,
    pub sections: Vec<usize>,
}

pub struct Section {
    // course name, e.g.: "CS 2810"
    pub course: String,
    // section name, e.g.: "01"
    pub section: String,

    // includes only the instructors explicity assigned to this section (not cross-listings)
    pub instructors: Vec<usize>,

    // a combined list of room+times that are acceptable to all instructors and cross-listings
    // with the worst penalty found
    pub room_times: Vec<RoomTimeWithPenalty>,

    // hard conflicts that named this section directly, not including cross-listings
    pub hard_conflicts: Vec<usize>,

    // soft conflicts that named this section directly, not including cross-listings
    pub soft_conflicts: Vec<SectionWithPenalty>,

    // if the course is cross listed it this list will have all cross-listed sections
    // in numerical order, and the first one is the canonical section (others tag along)
    // empty implies no cross listing
    pub cross_listings: Vec<usize>,
}

impl Section {
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
        format!("{}-{}", self.course, self.section)
    }
}

macro_rules! holiday {
    ($input:expr, $year:expr, $month:expr, $day:expr) => {
        $input.make_holiday(date($year, $month, $day)?)?
    };
}

macro_rules! room {
    ($input:expr,
            name: $name:expr,
            capacity: $capacity:expr
            $(, tags: $($tag:expr),+)?) => (
        $input.make_room(
            $name,
            $capacity,
            vec![$($($tag,)+)?])?
    );
}

macro_rules! time {
    ($input:expr,
            name: $name:literal
            $(, tags: $($tag:literal),+)?) => (
        $input.make_time(
            $name,
            vec![$($($tag,)+)?])?
    );
}

macro_rules! name_with_optional_penalty {
    ($name:literal with penalty $pen:literal) => {
        ($name.to_string(), $pen)
    };
    ($name:literal) => {
        ($name.to_string(), 0)
    };
}

macro_rules! instructor {
    ($input:expr,
            name: $name:expr,
            available: $($tag:literal $(with penalty $pen:literal)?),+ $(,)?) => {
        $input.make_instructor(
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
        $input.make_section(
            $section,
            vec![ $($inst.to_string(), $($insts.to_string(), )*)? ],
            vec![ $(name_with_optional_penalty!($tag $(with penalty $pen)?),)+ ]
        )?
    };
}

macro_rules! crosslist {
    ($input:expr,
            $section:literal
            $(cross-list with $sections:literal)+) => {
        $input.make_cross_listing(vec![
            $section,
            $($sections, )+
        ])?
    };
}

macro_rules! conflict {
    ($input:expr,
            set hard,
            clique: $($sections:literal),+ $(,)?) => {
        $input.make_conflict_clique(
            100, true,
            vec![ $($sections, )+ ])?;
    };
    ($input:expr,
            set penalty to $penalty:expr,
            clique: $($sections:literal),+ $(,)?) => {
        $input.make_conflict_clique(
            $penalty, true,
            vec![ $($sections, )+ ])?;
    };
    ($input:expr,
            remove penalty,
            clique: $($sections:literal),+ $(,)?) => {
        $input.make_conflict_clique(
            0, false,
            vec![ $($sections, )+ ])?;
    };
}

macro_rules! anticonflict {
    ($input:expr,
            set penalty to $penalty:expr,
            single: $single_course:literal,
            group: $($group_course:literal),+ $(,)?) => {
        $input.make_anti_conflict(
            $penalty,
            $single_course,
            vec![ $($group_course, )+ ])?;
    };
}

pub(crate) use {
    anticonflict, conflict, crosslist, holiday, instructor, name_with_optional_penalty, room,
    section, time,
};
