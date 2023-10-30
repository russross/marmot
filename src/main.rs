#[derive(Debug, Clone)]
struct Input {
    name: String,
    start: time::Date,
    end: time::Date,
    slots: Bits,
    rooms: Vec<Room>,
    time_slots: Vec<TimeSlot>,
    instructors: Vec<Instructor>,
    sections: Vec<Section>,
    missing: Vec<String>,
}

impl Input {
    fn new(name: &str, start: &str, end: &str) -> Result<Self, Box<rhai::EvalAltResult>> {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        let Ok(start) = time::Date::parse(start, &format) else {
            return Err(format!(
                "semester start date should be in format '2023-09-21', not '{}'",
                start
            )
            .into());
        };
        let Ok(end) = time::Date::parse(end, &format) else {
            return Err(format!(
                "semester end date should be in format '2023-09-21', not '{}'",
                end
            )
            .into());
        };

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
        Ok(Self {
            name: name.into(),
            start: start,
            end: end,
            slots: slots,
            rooms: Vec::new(),
            time_slots: Vec::new(),
            instructors: Vec::new(),
            sections: Vec::new(),
            missing: Vec::new(),
        })
    }

    fn block_out_holiday(&mut self, date: &str) -> Result<(), Box<rhai::EvalAltResult>> {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        let Ok(holiday) = time::Date::parse(date, &format) else {
            return Err(format!(
                "unrecognized holiday date format: '{}' should be like '2023-09-21'",
                date
            )
            .into());
        };
        let mut index = ((holiday - self.start).whole_days() * 24 * 60 / 5) as usize;
        for _hour in 0..24 {
            for _min in (0..60).step_by(5) {
                self.slots.set(index, false).unwrap();
                index += 1;
            }
        }
        Ok(())
    }

    fn make_time_slot(
        &mut self,
        name: &str,
        tags: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        // example: MWF0900+50
        let re = regex::Regex::new(
            r"^([mtwrfsuMTWRFSU]+)([0-1][0-9]|2[0-3])([0-5][05])\+([1-9][0-9]?[05])$",
        )
        .unwrap();

        let Some(caps) = re.captures(name) else {
            return Err(format!(
                "unrecognized time format: '{}' should be like 'MWF0900+50'",
                name
            )
            .into());
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
            slots: slots,
            days: days,
            start_time: start_time,
            duration: duration,
            conflicts: conflicts,
            tags: get_tags(tags)?,
        });

        Ok(())
    }

    fn find_time_slot_by_name(&self, name: &String) -> Result<usize, String> {
        let Some(i) = self.time_slots.iter().position(|elt| elt.name == *name) else {
            return Err(format!("timeslot named \"{}\" not found", name));
        };
        Ok(i)
    }

    fn make_room(
        &mut self,
        name: &str,
        cap: i64,
        tags: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        if cap <= 0 {
            return Err("room must have capacity > 0".into());
        }
        if cap > 10000 {
            return Err("room cannot have capacity > 10000".into());
        }
        self.rooms.push(Room {
            name: name.into(),
            capacity: cap as u16,
            tags: get_tags(tags)?,
        });
        Ok(())
    }

    fn make_instructor(
        &mut self,
        name: &str,
        available_times: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        let mut times: Vec<TimeWithPenalty> = Vec::new();
        for elt in available_times {
            match split_tag_and_penalty(elt) {
                Ok((time_name, badness)) => {
                    let Ok(time_slot) = self.find_time_slot_by_name(&time_name) else {
                        return Err(format!("unknown time slot name {}", time_name).into());
                    };
                    if times.iter().any(|t| t.time == time_slot) {
                        return Err(format!(
                            "time slot {} appears twice for instructor {}",
                            time_name, name
                        )
                        .into());
                    }
                    times.push(TimeWithPenalty {
                        time: time_slot,
                        penalty: badness,
                    });
                }
                Err(_) => {
                    return Err(
                        "available time for an instructor must be \"time\" or [\"time\", badness]"
                            .into(),
                    );
                }
            };
        }
        if self.find_instructor_by_name(&name.into()).is_ok() {
            return Err(format!("duplicate instructor name: {}", name).into());
        }
        self.instructors.push(Instructor {
            name: name.into(),
            available_times: times,
            sections: Vec::new(),
        });
        Ok(())
    }

    fn find_instructor_by_name(&self, name: &String) -> Result<usize, String> {
        let Some(i) = self.instructors.iter().position(|elt| elt.name == *name) else {
            return Err(format!("instructor named \"{}\" not found", name));
        };
        Ok(i)
    }

    fn make_section(
        &mut self,
        course: &str,
        section: &str,
        instructor_names: rhai::Dynamic,
        rooms_and_times: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        // start with the instructor: "Name" one, ["Name One", "Name Two", ...] means multiple
        let mut instructors = Vec::new();
        if instructor_names.is_string() {
            instructors.push(self.find_instructor_by_name(&instructor_names.into_string()?)?);
        } else if instructor_names.is_array() {
            let mut array = instructor_names.into_array()?;
            while array.len() > 0 {
                let elt = array.pop().unwrap();
                if !elt.is_string() {
                    return Err(
                        "instructor for a section must be \"name\" or [\"name1\", \"name2\", ...]"
                            .into(),
                    );
                }
                instructors.push(self.find_instructor_by_name(&elt.into_string()?)?);
            }
            instructors.sort();
        } else {
            return Err(
                "instructor for a section must be \"name\" or [\"name1\", \"name2\", ...]".into(),
            );
        }

        // now handle constraints
        let mut rwp = Vec::new();
        let mut twp = Vec::new();

        for elt in rooms_and_times {
            match split_tag_and_penalty(elt) {
                Ok((tag, badness)) => {
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
                                time: time_i,
                                penalty: badness,
                            },
                        );
                    }

                    if !found {
                        return Err(
                            format!("unrecognized constraint tag for section: {}", tag).into()
                        );
                    }
                }
                Err(_) => {
                    return Err(
                        "constraints for a section must be of form \"tag\" or [\"tag\", badness]"
                            .into(),
                    );
                }
            }
        }

        if rwp.len() == 0 {
            return Err(format!("no rooms found for {}-{}", course, section).into());
        }
        if twp.len() == 0 {
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
        let mut rtp = Vec::new();
        'rt: for time in twp.iter() {
            let mut time_penalty = time.penalty;
            for instructor_i in instructors.iter() {
                match self.instructors[*instructor_i]
                    .available_times
                    .iter()
                    .find(|itwp| itwp.time == time.time)
                {
                    Some(itwp) => time_penalty = std::cmp::max(time_penalty, itwp.penalty),
                    None => continue 'rt,
                }
            }
            for room in rwp.iter() {
                rtp.push(RoomTimeWithPenalty {
                    room: room.room,
                    time: time.time,
                    penalty: std::cmp::min(99, room.penalty + time_penalty),
                });
            }
        }
        if rtp.len() == 0 {
            return Err(format!("no valid room/time combinations found for {}-{} after considering instructor availability",
                course, section).into());
        }
        rtp.sort();
        if self
            .sections
            .iter()
            .any(|s| s.course == course.to_string() && s.section == section.to_string())
        {
            return Err(format!("course {}-{} appears more than once", course, section).into());
        }
        for instructor in &instructors {
            self.instructors[*instructor]
                .sections
                .push(self.sections.len());
        }
        self.sections.push(Section {
            course: course.into(),
            section: section.into(),
            instructors: instructors,
            room_times: rtp,
            hard_conflicts: Vec::new(),
            soft_conflicts: Vec::new(),
            cross_listings: vec![self.sections.len()],
        });

        Ok(())
    }

    fn make_hard_conflict_clique(
        &mut self,
        sections_raw: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        self.make_conflict_clique(100, true, sections_raw)
    }

    fn clear_conflict_clique(
        &mut self,
        sections_raw: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        self.make_conflict_clique(0, false, sections_raw)
    }

    fn make_soft_conflict_clique(
        &mut self,
        badness_raw: i64,
        sections_raw: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        if badness_raw < 1 || badness_raw > 99 {
            return Err("badness for a soft conflict clique must be between 1 and 99".into());
        }
        self.make_conflict_clique(badness_raw, true, sections_raw)
    }

    fn make_conflict_clique(
        &mut self,
        badness_raw: i64,
        maximize: bool,
        sections_raw: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        // parse and sanity check the inputs
        if badness_raw < 0 || badness_raw > 100 {
            return Err("badness for a conflict clique must be between 0 and 100".into());
        }
        let badness = badness_raw as u16;
        if badness == 100 && !maximize {
            return Err("make_conflict_clique does not support badness=100 and !maximize".into());
        } else if badness == 0 && maximize {
            return Err("make_conflict_clique does not support badness=0 and maximize".into());
        }
        let mut sections = Vec::new();
        for elt in sections_raw {
            let (course, section) = split_course_and_section(elt)?;
            let mut list = self.find_sections_by_name(&course, &section);
            if list.is_empty() {
                let missing = match section {
                    Some(s) => format!("{}-{}", course, s),
                    None => course,
                };
                if !self.missing.contains(&missing) {
                    self.missing.push(missing);
                }
            } else {
                sections.append(&mut list);
            }
        }

        // process every pairing (except element vs self)
        for &left_i in sections.iter() {
            for &right_i in sections.iter() {
                if left_i == right_i {
                    continue;
                }

                let left = &mut self.sections[left_i];
                let old = left.get_conflict(right_i);
                left.set_conflict(
                    right_i,
                    if maximize {
                        std::cmp::max(old, badness)
                    } else {
                        std::cmp::min(old, badness)
                    },
                );
            }
        }
        Ok(())
    }

    fn find_sections_by_name(&self, course: &String, section: &Option<String>) -> Vec<usize> {
        let mut list = Vec::new();
        self.sections.iter().enumerate().for_each(|(i, s)| {
            if s.course == *course {
                match section {
                    None => list.push(i),
                    Some(name) if *name == s.section => list.push(i),
                    _ => (),
                }
            }
        });
        list
    }

    fn make_cross_listing(
        &mut self,
        sections_raw: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        let mut sections = Vec::new();
        for elt in sections_raw {
            let (course, section) = split_course_and_section(elt)?;
            if section == None {
                return Err(
                    "cross listings must be between specific sections, not just courses".into(),
                );
            }
            let section_list = self.find_sections_by_name(&course, &section);
            if section_list.len() != 1 {
                return Err(format!(
                    "section {}-{} not found in cross listing",
                    course,
                    section.unwrap()
                )
                .into());
            }

            sections.push(section_list[0]);
        }
        sections.sort();
        sections.dedup();
        if sections.len() < 2 {
            return Err("cross listings must include at least two sections".into());
        }
        for &left_i in &sections {
            for &right_i in &sections {
                if left_i == right_i {
                    continue;
                }
                self.sections[left_i].cross_listings.push(right_i);
            }
            self.sections[left_i].cross_listings.sort();
            self.sections[left_i].cross_listings.dedup();
        }
        Ok(())
    }
}

fn get_tags(tags: Vec<rhai::Dynamic>) -> Result<Vec<String>, Box<rhai::EvalAltResult>> {
    let mut t = Vec::<String>::new();
    for elt in tags {
        t.push(elt.into_string()?)
    }
    Ok(t)
}

fn split_tag_and_penalty(input: rhai::Dynamic) -> Result<(String, u16), String> {
    if input.is_string() {
        Ok((input.into_string()?, 0))
    } else if input.is_array() {
        let mut array = input.into_array()?;
        if array.len() != 2 || !array[0].is_string() || !array[1].is_int() {
            return Err("expecting a \"tag\" or [\"tag\", badness]".into());
        }
        let badness = array.pop().unwrap().as_int()?;
        if badness < 0 || badness > 99 {
            return Err("badness must be in range [0,99]".into());
        }
        Ok((array.pop().unwrap().into_string()?, badness as u16))
    } else {
        Err("badness must be in range [0,99]".into())
    }
}

fn split_course_and_section(input: rhai::Dynamic) -> Result<(String, Option<String>), String> {
    if input.is_string() {
        Ok((input.into_string()?, None))
    } else if input.is_array() {
        let mut array = input.into_array()?;
        if array.len() != 2 || !array[0].is_string() || !array[1].is_string() {
            Err("course must be in form \"COURSE\" or [\"COURSE\", \"SECTION\"]".into())
        } else {
            let sec = array.pop().unwrap().into_string()?;
            let course = array.pop().unwrap().into_string()?;
            Ok((course, Some(sec)))
        }
    } else {
        Err("course must be in form \"COURSE\" or [\"COURSE\", \"SECTION\"]".into())
    }
}

fn date_range_slots(start: time::Date, end: time::Date) -> usize {
    let size = ((end - start).whole_days() + 1) * 24 * 60 / 5;
    if size <= 0 {
        panic!("date_range_slots must have start < end");
    }
    size as usize
}

fn add_room_with_penalty_keep_worst(list: &mut Vec<RoomWithPenalty>, rwp: RoomWithPenalty) {
    match list.iter().position(|elt| elt.room == rwp.room) {
        Some(i) => list[i].penalty = std::cmp::max(list[i].penalty, rwp.penalty),
        None => list.push(rwp),
    }
}

fn add_time_with_penalty_keep_worst(list: &mut Vec<TimeWithPenalty>, twp: TimeWithPenalty) {
    match list.iter().position(|elt| elt.time == twp.time) {
        Some(i) => list[i].penalty = std::cmp::max(list[i].penalty, twp.penalty),
        None => list.push(twp),
    }
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    capacity: u16,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct TimeSlot {
    name: String,
    slots: Bits,
    days: Vec<time::Weekday>,
    start_time: time::Time,
    duration: time::Duration,

    // a list of all overlapping time slots
    // a time slot IS always in its own conflict list
    conflicts: Vec<usize>,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct RoomTime {
    room: usize,
    time_slot: usize,
}

#[derive(Debug, Clone)]
struct TimeWithPenalty {
    time: usize,
    penalty: u16,
}

#[derive(Debug, Clone)]
struct RoomWithPenalty {
    room: usize,
    penalty: u16,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord)]
struct RoomTimeWithPenalty {
    room: usize,
    time: usize,
    penalty: u16,
}

impl PartialOrd for RoomTimeWithPenalty {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.room != other.room {
            return Some(self.room.cmp(&other.room));
        }
        if self.time != other.time {
            return Some(self.time.cmp(&other.time));
        }
        return Some(self.penalty.cmp(&other.penalty));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord)]
struct SectionWithPenalty {
    section: usize,
    penalty: u16,
}

impl PartialOrd for SectionWithPenalty {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.section != other.section {
            return Some(self.section.cmp(&other.section));
        }
        return Some(self.penalty.cmp(&other.penalty));
    }
}

#[derive(Debug, Clone)]
struct Instructor {
    name: String,
    available_times: Vec<TimeWithPenalty>,
    sections: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Section {
    course: String,
    section: String,
    instructors: Vec<usize>,
    room_times: Vec<RoomTimeWithPenalty>,
    hard_conflicts: Vec<usize>,
    soft_conflicts: Vec<SectionWithPenalty>,

    // every course is in its own cross_listings vector, which must be a clique
    // placement occurs on the section with the lowest index, others tag along
    cross_listings: Vec<usize>,
}

impl Section {
    fn get_conflict(&self, other: usize) -> u16 {
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

    fn set_conflict(&mut self, other: usize, penalty: u16) {
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
                    penalty: penalty,
                }),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Bits {
    size: usize,
    bits: Vec<u64>,
}

impl Bits {
    fn new(size: usize) -> Self {
        let chunks = (size + 63) / 64;
        let mut bits = Vec::with_capacity(chunks);
        for _i in 0..chunks {
            bits.push(0);
        }
        Bits {
            size: size,
            bits: bits,
        }
    }

    fn _get(&self, index: usize) -> Result<bool, String> {
        if index >= self.size {
            return Err(format!(
                "Bits::get index out of range: {} requested but size is {}",
                index, self.size
            )
            .into());
        }
        Ok(self.bits[index / 64] & (1 << (index % 64)) != 0)
    }

    fn set(&mut self, index: usize, val: bool) -> Result<(), String> {
        if index >= self.size {
            return Err(format!(
                "Bits::set index out of range: {} requested but size is {}",
                index, self.size
            ));
        }
        if val {
            self.bits[index / 64] |= 1 << (index % 64);
        } else {
            self.bits[index / 64] &= !(1 << (index % 64));
        }
        Ok(())
    }

    fn intersect_in_place(&mut self, other: &Bits) -> Result<(), String> {
        if self.size != other.size {
            return Err(format!(
                "Bits::intersect_in_place size mismatch: {} vs {}",
                self.size, other.size
            ));
        }
        for (i, elt) in other.bits.iter().enumerate() {
            self.bits[i] &= elt;
        }
        Ok(())
    }

    fn is_disjoint(&self, other: &Bits) -> Result<bool, String> {
        if self.size != other.size {
            return Err(format!(
                "Bits::is_disjoint size mismatch: {} vs {}",
                self.size, other.size
            ));
        }
        for (i, elt) in other.bits.iter().enumerate() {
            if self.bits[i] & elt != 0 {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn main() -> Result<(), Box<rhai::EvalAltResult>> {
    let mut engine = rhai::Engine::new();
    engine
        .register_type_with_name::<Input>("Input")
        .register_fn("term", Input::new)
        .register_fn("holiday", Input::block_out_holiday)
        .register_fn("time", Input::make_time_slot)
        .register_fn("room", Input::make_room)
        .register_fn("instructor", Input::make_instructor)
        .register_fn("section", Input::make_section)
        .register_fn("hard_conflict_clique", Input::make_hard_conflict_clique)
        .register_fn("clear_conflict_clique", Input::clear_conflict_clique)
        .register_fn("conflict_clique", Input::make_soft_conflict_clique)
        .register_fn("crosslist", Input::make_cross_listing);
    let mut term = engine.eval_file::<Input>("setup.rhai".into())?;

    // add hard conflicts between all the sections an instructor teaches
    for instructor in &term.instructors {
        for left in &instructor.sections {
            for right in &instructor.sections {
                if left == right {
                    continue;
                };
                term.sections[*left].set_conflict(*right, 100);
            }
        }
    }

    println!("term: {} from {} to {}", term.name, term.start, term.end);

    for (i, time_slot) in term.time_slots.iter().enumerate() {
        print!("time slot {}: ", time_slot.name);
        let mut sep = "";
        for elt in &time_slot.days {
            print!("{}{}", sep, elt);
            sep = ", ";
        }
        print!(" at {} for {}", time_slot.start_time, time_slot.duration);
        if time_slot.conflicts.len() > 1 {
            print!(", conflicts with ");
            sep = "";
            for elt in &time_slot.conflicts {
                if *elt == i {
                    continue;
                }
                print!("{}{}", sep, term.time_slots[*elt].name);
                sep = ", ";
            }
        }
        println!("");
    }

    for room in &term.rooms {
        print!("{} {} tags:", room.name, room.capacity);
        for tag in &room.tags {
            print!(" {}", tag);
        }
        println!("");
    }
    let mut instructor_order = Vec::new();
    for i in 0..term.instructors.len() {
        instructor_order.push(i);
    }
    instructor_order.sort_by_key(|&i| term.instructors[i].name.clone());
    for inst_i in instructor_order {
        let inst = &term.instructors[inst_i];
        print!("{}", inst.name);
        for time in &inst.available_times {
            if time.penalty == 0 {
                print!(" {}", term.time_slots[time.time].name);
            } else {
                print!(" {}:{}", term.time_slots[time.time].name, time.penalty);
            }
        }
        println!("");
    }
    let mut section_order = Vec::new();
    for i in 0..term.sections.len() {
        section_order.push(i);
    }
    section_order
        .sort_by_key(|&i| format!("{}-{}", term.sections[i].course, term.sections[i].section));
    for sec_i in section_order {
        let sec = &term.sections[sec_i];
        print!("{}-{}", sec.course, sec.section);
        if sec.cross_listings.len() > 1 {
            for &other in sec.cross_listings.iter() {
                if sec_i == other {
                    continue;
                }
                print!(
                    " x {}-{}",
                    term.sections[other].course, term.sections[other].section
                );
            }
        }
        print!(" [");
        for (i, inst_i) in sec.instructors.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", &term.instructors[*inst_i].name);
        }
        print!("]");
        let mut prev_room = term.rooms.len();
        for rtp in sec.room_times.iter() {
            if rtp.room != prev_room {
                prev_room = rtp.room;
                print!("\n    {}:", &term.rooms[rtp.room].name);
            }
            print!(" {}", &term.time_slots[rtp.time].name);
            if rtp.penalty > 0 {
                print!(":{}", rtp.penalty);
            }
        }
        println!("");
        if !sec.hard_conflicts.is_empty() {
            print!("    hard conflicts:");
            for &i in sec.hard_conflicts.iter() {
                print!(" {}-{}", term.sections[i].course, term.sections[i].section);
            }
            println!("");
        }
        if !sec.soft_conflicts.is_empty() {
            print!("    soft conflicts:");
            for elt in sec.soft_conflicts.iter() {
                print!(
                    " {}-{}:{}",
                    term.sections[elt.section].course,
                    term.sections[elt.section].section,
                    elt.penalty
                );
            }
            println!("");
        }
    }
    term.missing.sort();
    if term.missing.len() > 0 {
        print!("unknown courses:");
        let mut sep = " ";
        for elt in &term.missing {
            print!("{}{}", sep, elt);
            sep = ", ";
        }
        println!("");
    }

    Ok(())
}

struct Solver {
    room_placements: Vec<RoomPlacements>,
    sections: Vec<SolverSection>,
}

struct RoomPlacements {
    time_slot_placements: Vec<Option<usize>>,
}

struct SolverSection {
    placement: Option<RoomTime>,
    tickets: u64,
}

impl Solver {
    fn new(input: &Input) -> Self {
        let mut room_placements = Vec::with_capacity(input.rooms.len());
        for _ in 0..input.rooms.len() {
            room_placements.push(RoomPlacements {
                time_slot_placements: vec![None; input.time_slots.len()],
            });
        }
        let mut sections = Vec::with_capacity(input.sections.len());
        for _ in 0..input.sections.len() {
            sections.push(SolverSection {
                placement: None,
                tickets: 0,
            });
        }
        Solver {
            room_placements: room_placements,
            sections: sections,
        }
    }

    // remove a section from its current room/time placement (if any)
    // remove it from both sections and room_placements
    fn remove_placement(&mut self, section: usize) {
        if let Some(RoomTime { room, time_slot }) =
            std::mem::take(&mut self.sections[section].placement)
        {
            assert!(std::mem::take(&mut self.room_placements[room].time_slot_placements[time_slot]) == Some(section),
            "Solver::remove_placement: placement by section does not match placement by room and time");
        }
    }

    // remove any sections that will be in conflict with a section about to be placed
    //
    // this includes:
    // * anything in the same room in an overlapping time slot
    // * anything in the hard conflict list of this section (or a cross listing)
    //   in the same/an overlapping time slot
    fn displace_conflict(&mut self, input: &Input, section: usize, room: usize, time_slot: usize) {
        // is this slot (or an overlapping time in the same room) already occupied?
        for overlapping in &input.time_slots[time_slot].conflicts {
            if let Some(existing) = self.room_placements[room].time_slot_placements[*overlapping] {
                self.remove_placement(existing);
            }
        }
        //cases:
        //any overlapping time in self? weird case;
        //same room & overlapping time,
        //hard conflict courses from conflict list
        let existing =
            std::mem::take(&mut self.room_placements[room].time_slot_placements[time_slot]);
        if let Some(other) = existing {
            let section_placement = std::mem::take(&mut self.sections[other].placement);

            // sanity check
            let Some(RoomTime {
                room: other_room,
                time_slot: other_time_slot,
            }) = section_placement
            else {
                panic!("Solver::displace_conflict room placement found but solver section is None");
            };
            if other_room != room || other_time_slot != time_slot {
                panic!("Solver::displace_conflict room placement does not match solver section");
            };
        }
    }
}
