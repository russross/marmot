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
                println!("timeslot {} conflicts with {}", name, other.name);
                conflicts.push(other_index);
                other.conflicts.push(my_index);
            }
        }

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
                        return Err(format!("time slot {} appears twice for instructor {}", time_name, name).into());
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
                    for (room_i, _room) in self.rooms.iter().enumerate().filter(|(_, r)| r.name == tag || r.tags.contains(&tag)) {
                        found = true;
                        add_room_with_penalty_keep_worst(&mut rwp, RoomWithPenalty{ room: room_i, penalty: badness });
                    }

                    // check for matching times
                    for (time_i, _time) in self.time_slots.iter().enumerate().filter(|(_, t)| t.name == tag || t.tags.contains(&tag)) {
                        found = true;
                        add_time_with_penalty_keep_worst(&mut twp, TimeWithPenalty{ time: time_i, penalty: badness });
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
        if self.sections.iter().any(|s| s.course == course.to_string() && s.section == section.to_string()) {
            return Err(format!("course {}-{} appears more than once", course, section).into());
        }
        self.sections.push(Section {
            course: course.into(),
            section: section.into(),
            instructors: instructors,
            room_times: rtp,
        });

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
        None=> list.push(twp),
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
    conflicts: Vec<usize>,
    tags: Vec<String>,
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
        .register_fn("section", Input::make_section);
    let mut term = engine.eval_file::<Input>("setup.rhai".into())?;
    let mut day = term.start;
    println!("{}", day);
    while day <= term.end {
        if day == term.end {
            println!("{}", day);
        }
        day = day.next_day().unwrap();
    }
    for room in &term.rooms {
        print!("{} {} tags:", room.name, room.capacity);
        for tag in &room.tags {
            print!(" {}", tag);
        }
        println!("");
    }
    for inst in &term.instructors {
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
    term.sections.sort_by_key(|s| format!("{}-{}", s.course, s.section));
    for sec in &term.sections {
        print!("{}-{} [", sec.course, sec.section);
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
    }
    Ok(())
}
