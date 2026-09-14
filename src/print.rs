use super::error::Result;
use super::input::*;
use super::score::*;
use super::solver::*;
use std::cmp::max;
use std::fmt::Write;

#[derive(Clone, Copy)]
enum DaySequence {
    MondayWednesdayFriday,
    TuesdayThursday,
}

impl DaySequence {
    fn includes(self, days: Days) -> bool {
        let mask = match self {
            Self::MondayWednesdayFriday => 0b0010101,
            Self::TuesdayThursday => 0b0001010,
        };
        days.days & mask != 0
    }
}

type Cell = Vec<(String, String)>;

#[derive(Clone, Copy, PartialEq, Eq)]
struct RowKey {
    days: u8,
    start_time: Time,
}

pub fn print_schedule(input: &Input, schedule: &Schedule) {
    print!("{}", render_schedule(input, schedule));
}

fn render_schedule(input: &Input, schedule: &Schedule) -> String {
    let sequences = [DaySequence::MondayWednesdayFriday, DaySequence::TuesdayThursday];
    let mut rooms: Vec<usize> = schedule
        .placements
        .iter()
        .filter_map(|Placement { time_slot, room, .. }| {
            let (Some(time_slot), Some(room)) = (time_slot, room) else {
                return None;
            };
            sequences.iter().any(|sequence| sequence.includes(input.time_slots[*time_slot].days)).then_some(*room)
        })
        .collect();
    rooms.sort_unstable();
    rooms.dedup();
    let mut width = 1;
    let tables: Vec<Vec<Vec<Cell>>> =
        sequences.iter().map(|&sequence| build_table(input, schedule, &rooms, sequence, &mut width)).collect();
    width += 2;

    let mut output = String::new();
    for (table_index, table) in tables.iter().enumerate() {
        if table_index > 0 {
            output.push('\n');
        }
        write_table(&mut output, table, width);
    }
    for (section, Placement { time_slot, room, .. }) in schedule.placements.iter().enumerate() {
        let (&Some(time_slot), None) = (time_slot, room) else {
            continue;
        };
        writeln!(&mut output, "{} at {} with no room", input.sections[section].name, input.time_slots[time_slot].name)
            .unwrap();
    }
    output
}

fn build_table(
    input: &Input,
    schedule: &Schedule,
    rooms: &[usize],
    sequence: DaySequence,
    width: &mut usize,
) -> Vec<Vec<Cell>> {
    let mut row_keys: Vec<RowKey> = schedule
        .placements
        .iter()
        .filter_map(|Placement { time_slot, room, .. }| {
            let time_slot = &input.time_slots[(*time_slot)?];
            (room.is_some() && sequence.includes(time_slot.days))
                .then_some(RowKey { days: time_slot.days.days, start_time: time_slot.start_time })
        })
        .collect();
    row_keys.sort_unstable_by_key(|key| (key.start_time, key.days));
    row_keys.dedup();

    let mut table = Vec::with_capacity(row_keys.len() + 1);
    let mut header = vec![vec![(String::new(), String::new())]];
    for &room in rooms {
        let room_name = input.rooms[room].name.clone();
        *width = max(*width, room_name.len());
        header.push(vec![(room_name, String::new())]);
    }
    table.push(header);

    for row_key in row_keys {
        let row_days = Days { days: row_key.days };
        let durations: Vec<Duration> = schedule
            .placements
            .iter()
            .filter_map(|Placement { time_slot, room, .. }| {
                let time_slot = &input.time_slots[(*time_slot)?];
                (room.is_some() && time_slot.days.days == row_key.days && time_slot.start_time == row_key.start_time)
                    .then_some(time_slot.duration)
            })
            .collect();
        let canonical_duration = (150 % row_days.len() == 0).then(|| Duration::new((150 / row_days.len()) as u16));
        let row_duration = canonical_duration
            .filter(|duration| durations.contains(duration))
            .unwrap_or_else(|| *durations.iter().min().expect("each row has a starting class"));
        let row_name = format!(
            "{}{:02}{:02}+{}",
            row_days,
            row_key.start_time.minutes / 60,
            row_key.start_time.minutes % 60,
            row_duration.minutes
        );
        *width = max(*width, row_name.len());
        let mut row = vec![vec![(row_name, String::new())]];

        for &room in rooms {
            let mut cell = Cell::new();
            let mut is_in_use = false;
            for (section, Placement { time_slot, room: placed_room, .. }) in schedule.placements.iter().enumerate() {
                let (Some(time_slot), Some(placed_room)) = (time_slot, placed_room) else {
                    continue;
                };
                if *placed_room != room {
                    continue;
                }
                let time_slot = &input.time_slots[*time_slot];
                if time_slot.days.days == row_key.days && time_slot.start_time == row_key.start_time {
                    let section = &input.sections[section];
                    let mut section_name = section.name.clone();
                    if time_slot.duration != row_duration {
                        write!(&mut section_name, " +{}", time_slot.duration.minutes).unwrap();
                    }
                    let faculty_name = match section.faculty.as_slice() {
                        [] => String::new(),
                        [faculty] => input.faculty[*faculty].name.clone(),
                        [faculty, ..] => format!("{}+", input.faculty[*faculty].name),
                    };
                    *width = max(*width, section_name.len());
                    *width = max(*width, faculty_name.len());
                    cell.push((section_name, faculty_name));
                } else if !time_slot.days.intersect(&row_days).is_empty()
                    && time_slot.start_time < row_key.start_time
                    && time_slot.start_time + time_slot.duration > row_key.start_time
                {
                    is_in_use = true;
                }
            }
            if is_in_use {
                *width = max(*width, "(in use)".len());
                cell.push(("(in use)".to_string(), String::new()));
            }
            if cell.is_empty() {
                cell.push((String::new(), String::new()));
            }
            row.push(cell);
        }
        table.push(row);
    }
    table
}

fn write_table(output: &mut String, table: &[Vec<Cell>], width: usize) {
    let columns = table.first().map_or(0, Vec::len);
    let divider = format!("+{}+\n", vec!["-".repeat(width); columns].join("+"));
    output.push_str(&divider);
    for row in table {
        let height = row.iter().map(Vec::len).max().unwrap_or(1);
        for line in 0..height {
            output.push('|');
            for cell in row {
                let section_name = cell.get(line).map_or("", |(section, _)| section.as_str());
                write_cell(output, section_name, width);
            }
            output.push('\n');
            output.push('|');
            for cell in row {
                let faculty_name = cell.get(line).map_or("", |(_, faculty)| faculty.as_str());
                write_cell(output, faculty_name, width);
            }
            output.push('\n');
        }
        output.push_str(&divider);
    }
}

fn write_cell(output: &mut String, value: &str, width: usize) {
    write!(output, " {:<width$}|", value, width = width - 1).unwrap();
}

pub fn print_problems(input: &Input, schedule: &Schedule) {
    let mut lst = Vec::new();
    for (section, placement) in schedule.placements.iter().enumerate() {
        if placement.time_slot.is_none() {
            lst.push((
                LEVEL_FOR_UNPLACED_SECTION,
                String::new(),
                format!("{} is not placed", input.sections[section].name),
            ));
            continue;
        }
    }
    for penalty_list in &schedule.penalties {
        for penalty in penalty_list {
            let mut faculty = penalty.faculty().map_or_else(Vec::new, |owner| vec![owner]);
            if faculty.is_empty() {
                for section in penalty.get_sections(input) {
                    faculty.extend_from_slice(&input.sections[section].faculty);
                }
                faculty.sort_unstable();
                faculty.dedup();
            }
            let (priority, msg) = penalty.get_score_message(input, schedule);

            // curriculum conflicts are displayed once, preferences are per-faculty
            if faculty.is_empty() || priority < START_LEVEL_FOR_PREFERENCES {
                lst.push((priority, String::new(), msg));
            } else {
                for elt in faculty {
                    lst.push((priority, input.faculty[elt].name.clone(), msg.clone()));
                }
            }
        }
    }
    lst.sort_unstable_by(|a, b| {
        if a.0 != b.0 && (a.0 < START_LEVEL_FOR_PREFERENCES || b.0 < START_LEVEL_FOR_PREFERENCES) {
            a.0.cmp(&b.0)
        } else if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    for (priority, _faculty, msg) in lst {
        // To omit priority numbers for preferences, print only `msg` when
        // `priority >= START_LEVEL_FOR_PREFERENCES`.
        println!("{priority:2}: {msg}");
    }
}

pub fn dump_input(departments: &[String], input: &Input) {
    if departments.is_empty() {
        print!("{} for all departments: ", input.term_name);
    } else if departments.len() == 1 {
        print!("{} for {}: ", input.term_name, departments[0]);
    } else {
        let mut sep = "";
        print!("{} for ", input.term_name);
        for (i, name) in departments.iter().enumerate() {
            print!("{}{}", sep, name);
            if i + 2 == departments.len() && i >= 1 {
                sep = ", and ";
            } else if i + 2 == departments.len() {
                sep = " and ";
            } else {
                sep = ", ";
            }
        }
        print!(": ");
    }
    println!("{} rooms, {} time slots", input.rooms.len(), input.time_slots.len());

    print!("\nRooms: ");
    let mut sep = "";
    for elt in &input.rooms {
        print!("{sep}{elt}");
        sep = ", ";
    }
    println!();
    print!("\nTime slots: ");
    sep = "";
    for elt in &input.time_slots {
        print!("{sep}{elt}");
        sep = ", ";
    }
    println!();

    println!("\nFaculty:");
    for faculty in &input.faculty {
        println!("{}", faculty.debug(input));
    }

    println!("\nSections:");
    for section in &input.sections {
        print!(
            "section {} ({}), with {} rooms and {} times",
            section.name,
            section.credit_hours,
            section.rooms.len(),
            section.time_slots.len()
        );
        if !section.faculty.is_empty() {
            print!(", faculty");
            for faculty in &section.faculty {
                print!(" {faculty}");
            }
        }
        println!();
        let mut sep = "    hard conflicts: ";
        for &elt in &section.hard_conflicts {
            print!("{}{}", sep, input.sections[elt].name);
            sep = " ";
        }
        if !section.hard_conflicts.is_empty() {
            println!();
        }
    }

    println!("\nScoring criteria:");
    for elt in &input.criteria {
        println!("{}", elt.debug(input));
    }
}

pub fn commas<T: TryInto<i64>>(n: T) -> String {
    let mut n = n.try_into().unwrap_or(0);
    let mut minus = "";
    if n < 0 {
        n = -n;
        minus = "-";
    }
    let mut s = String::new();
    loop {
        if n < 1000 {
            s = format!("{}{}", n, s);
            break;
        }
        s = format!(",{:03}{}", n % 1000, s);
        n /= 1000;
    }
    format!("{minus}{s}")
}

pub fn ms_to_string(ms: u128) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 10000 {
        format!("{:.1}s", (ms as f64) / 1000.0)
    } else {
        sec_to_string((ms as u64) / 1000)
    }
}

pub fn string_to_sec(duration: &str) -> Result<u64> {
    let mut seconds = 0;
    let mut digits = 0;
    for ch in duration.chars() {
        match ch {
            '0'..='9' => {
                digits *= 10;
                digits += ch.to_digit(10).unwrap();
            }
            'h' => {
                seconds += digits * 60 * 60;
                digits = 0;
            }
            'm' => {
                seconds += digits * 60;
                digits = 0;
            }
            's' => {
                seconds += digits;
                digits = 0;
            }
            _ => return Err(format!("failed to parse {duration}; expected, e.g., 2h5m13s").into()),
        }
    }
    if digits != 0 {
        Err(format!("failed to parse {duration}; expected, e.g.: 2h5m13s but found extra digits at end").into())
    } else {
        Ok(seconds as u64)
    }
}

pub fn sec_to_string(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{}s", seconds);
    }
    if seconds < 3600 && seconds.is_multiple_of(60) {
        return format!("{}m", seconds / 60);
    }
    if seconds < 3600 {
        return format!("{}m{:02}s", seconds / 60, seconds % 60);
    }
    if seconds.is_multiple_of(3600) {
        return format!("{}h", seconds / 3600);
    }
    if seconds.is_multiple_of(60) {
        return format!("{}h{}m", seconds / 3600, (seconds % 3600) / 60);
    }
    format!("{}h{:02}m{:02}s", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::faculty_preferences::FacultyPreferencePriorityPolicy;

    fn test_input(time_slots: &[(&str, u16, u16)]) -> Input {
        let time_slots: Vec<TimeSlot> = time_slots
            .iter()
            .map(|(days, start_time, duration)| TimeSlot {
                name: format!("{days}{start_time}+{duration}"),
                days: Days::parse(days).unwrap(),
                start_time: Time::new(*start_time),
                duration: Duration::new(*duration),
            })
            .collect();
        let sections = (0..time_slots.len())
            .map(|index| Section {
                name: format!("Section {index}"),
                credit_hours: CreditHours::new(3.0),
                rooms: Vec::new(),
                time_slots: Vec::new(),
                faculty: Vec::new(),
                hard_conflicts: Vec::new(),
                criteria: Vec::new(),
                neighbors: Vec::new(),
            })
            .collect();
        let time_slot_count = time_slots.len();
        Input {
            term_name: "Test".to_string(),
            rooms: vec![
                Room { name: "Room A".to_string() },
                Room { name: "Room B".to_string() },
                Room { name: "Room C".to_string() },
            ],
            time_slots,
            faculty: Vec::new(),
            sections,
            criteria: Vec::new(),
            faculty_preference_priority_policy: FacultyPreferencePriorityPolicy::Stated,
            time_slot_conflicts: vec![vec![false; time_slot_count]; time_slot_count],
        }
    }

    #[test]
    fn schedule_tables_merge_only_matching_days_and_starts() {
        let input = test_input(&[
            ("MWF", 540, 50),
            ("MWF", 540, 75),
            ("MW", 540, 75),
            ("MTWR", 720, 75),
            ("F", 780, 50),
            ("T", 900, 150),
            ("T", 990, 50),
            ("T", 990, 150),
            ("R", 990, 150),
        ]);
        let mut schedule = Schedule::new(&input);
        for (section, room) in [0, 1, 2, 0, 0, 0, 1, 2, 1].into_iter().enumerate() {
            let time_slot = section;
            schedule.placements[section].time_slot = Some(time_slot);
            schedule.placements[section].room = Some(room);
        }

        let output = render_schedule(&input, &schedule);

        assert_eq!(output.matches("MWF0900+50").count(), 1);
        assert!(output.contains("Section 1 +75"));
        assert!(output.contains("MW0900+75"));
        assert!(output.contains("F1300+50"));
        assert!(output.contains("T1630+150"));
        assert!(output.contains("Section 6 +50"));
        assert!(!output.contains("Section 7 +150"));
        assert!(output.contains("R1630+150"));
        assert!(!output.contains("TR1630"));
        assert_eq!(output.matches("Section 3").count(), 2);
        assert_eq!(output.matches("Room A").count(), 2);
        assert!(output.contains("\n\n+"));
        assert!(output.find("MTWR1200+75").unwrap() < output.rfind("MTWR1200+75").unwrap());

        let friday_row = row_text(&output, "F1300+50");
        assert!(!friday_row.contains("(in use)"));
        let tuesday_row = row_text(&output, "T1630+150");
        assert!(tuesday_row.contains("(in use)"));
        let thursday_row = row_text(&output, "R1630+150");
        assert!(!thursday_row.contains("(in use)"));
    }

    #[test]
    fn non_bell_rows_use_the_shortest_starting_duration() {
        let input = test_input(&[("F", 780, 80), ("F", 780, 50)]);
        let mut schedule = Schedule::new(&input);
        schedule.placements[0].time_slot = Some(0);
        schedule.placements[0].room = Some(0);
        schedule.placements[1].time_slot = Some(1);
        schedule.placements[1].room = Some(1);

        let output = render_schedule(&input, &schedule);

        assert!(output.contains("F1300+50"));
        assert!(output.contains("Section 0 +80"));
        assert!(!output.contains("Section 1 +50"));
    }

    fn row_text<'a>(output: &'a str, row_name: &str) -> &'a str {
        let row_start = output.find(row_name).unwrap();
        let row_end = output[row_start..].find('\n').map_or(output.len(), |offset| row_start + offset);
        &output[row_start..row_end]
    }
}
