#[derive(Debug, Clone)]
struct Input {
    name: String,
    start: time::Date,
    end: time::Date,
    slots: Bits,
    buildings: Vec<Building>,
    rooms: Vec<Room>,
    timeslots: Vec<Timeslot>,
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
        let mut i: usize = 0;
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
            buildings: Vec::new(),
            rooms: Vec::new(),
            timeslots: Vec::new(),
        })
    }

    fn holiday(&mut self, date: &str) -> Result<(), Box<rhai::EvalAltResult>> {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        let Ok(holiday) = time::Date::parse(date, &format) else {
            return Err(format!(
                "unrecognized holiday date format: '{}' should be like '2023-09-21'",
                date
            )
            .into());
        };
        let mut index: usize = ((holiday - self.start).whole_days() * 24 * 60 / 5) as usize;
        for _hour in 0..24 {
            for _min in (0..60).step_by(5) {
                self.slots.set(index, false).unwrap();
                index += 1;
            }
        }
        Ok(())
    }

    fn time(
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
        let start_hour = hour_part.parse::<u32>().unwrap();
        let start_minute = minute_part.parse::<u32>().unwrap();
        let length = length_part.parse::<u32>().unwrap();

        // set up the vector of 5-minute intervals used over the term
        let mut slots = Bits::new(date_range_slots(self.start, self.end));
        let mut i: usize = 0;
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
        let my_index = self.timeslots.len();
        for (other_index, other) in self.timeslots.iter_mut().enumerate() {
            if !slots.is_disjoint(&other.slots)? {
                println!("timeslot {} conflicts with {}", name, other.name);
                conflicts.push(other_index);
                other.conflicts.push(my_index);
            }
        }

        self.timeslots.push(Timeslot {
            name: name.into(),
            slots: slots,
            conflicts: conflicts,
            tags: get_tags(tags)?,
        });

        Ok(())
    }

    fn building(
        &mut self,
        name: &str,
        tags: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        self.buildings.push(Building {
            name: name.into(),
            rooms: Vec::new(),
            tags: get_tags(tags)?,
        });

        Ok(())
    }

    fn room(
        &mut self,
        name: &str,
        cap: i64,
        tags: Vec<rhai::Dynamic>,
    ) -> Result<(), Box<rhai::EvalAltResult>> {
        self.rooms.push(Room {
            name: name.into(),
            building: self.buildings.len() - 1,
            capacity: cap as u16,
            tags: get_tags(tags)?,
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

fn date_range_slots(start: time::Date, end: time::Date) -> usize {
    let size = ((end - start).whole_days() + 1) * 24 * 60 / 5;
    if size <= 0 {
        panic!("date_range_slots must have start < end");
    }
    size as usize
}

#[derive(Debug, Clone)]
struct Building {
    name: String,
    rooms: Vec<usize>,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    building: usize,
    capacity: u16,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
struct Timeslot {
    name: String,
    slots: Bits,
    conflicts: Vec<usize>,
    tags: Vec<String>,
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

    fn get(&self, index: usize) -> Result<bool, String> {
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
        .register_fn("holiday", Input::holiday)
        .register_fn("time", Input::time)
        .register_fn("building", Input::building)
        .register_fn("room", Input::room);
    let term = engine.eval_file::<Input>("setup.rhai".into())?;
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
    Ok(())
}
