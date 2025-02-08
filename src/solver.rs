use super::input::*;
use super::score::*;
use super::*;
use std::cmp::{max, min};
use std::io::Write;
use std::mem::take;
use std::time::Instant;

//
//
// Schedule data
// A single candidate schedule with section placements,
// records of scores associated with the schedule, etc.
// Includes placement log types and actions.
//
//

// the complete schedule and its score
#[derive(Clone)]
pub struct Schedule {
    pub placements: Vec<Placement>,
    pub room_placements: Vec<RoomPlacements>,
    pub penalties: Vec<Vec<Penalty>>,
    pub score: Score,
}

// placement details of a single section
#[derive(Clone)]
pub struct Placement {
    pub time_slot: Option<usize>,
    pub room: Option<usize>,
    pub score: Score,
}

// to undo a move, undo the penalties in reverse order
// and restore all of the scores
pub struct PlacementLog {
    // all of the sections that were moved
    pub moves: Vec<PlacementLogEntry>,
    pub criteria: Vec<usize>,
}

// a single change to a section's placement
pub enum PlacementLogEntry {
    // this section was placed (displacing it will undo)
    Add { section: usize },

    // this section was displaced (placing it will undo)
    Remove { section: usize, time_slot: usize, room: Option<usize> },
}

#[derive(Clone)]
pub struct RoomPlacements {
    pub used_time_slots: Vec<TimeSlotPlacement>,
}

#[derive(Clone)]
pub struct TimeSlotPlacement {
    pub time_slot: usize,
    pub section: usize,
}

impl Schedule {
    pub fn new(input: &Input) -> Self {
        let mut placements = Vec::new();
        let mut score = Score::new();
        let unplaced_score = Score::new() + LEVEL_FOR_UNPLACED_SECTION;
        for _ in 0..input.sections.len() {
            placements.push(Placement { time_slot: None, room: None, score: unplaced_score });
            score += LEVEL_FOR_UNPLACED_SECTION;
        }

        let mut room_placements = Vec::new();
        for _ in 0..input.rooms.len() {
            room_placements.push(RoomPlacements { used_time_slots: Vec::new() });
        }

        let mut penalties = Vec::new();
        for _ in 0..input.criteria.len() {
            penalties.push(Vec::new());
        }

        Schedule { placements, room_placements, penalties, score }
    }

    pub fn is_placed(&self, section: usize) -> bool {
        self.placements[section].time_slot.is_some()
    }

    // remove a section from its current room/time placement (if any)
    // this does not update scoring
    pub fn remove_placement(&mut self, section: usize, undo: &mut Vec<PlacementLogEntry>) {
        if let Some(time_slot) = take(&mut self.placements[section].time_slot) {
            // does it have a room?
            let room = if let Some(room) = take(&mut self.placements[section].room) {
                // remove it from room_placements
                self.room_placements[room]
                    .used_time_slots
                    .retain(|TimeSlotPlacement { section: elt, .. }| *elt != section);
                Some(room)
            } else {
                None
            };

            undo.push(PlacementLogEntry::Remove { section, time_slot, room });
        }
    }

    pub fn add_placement(
        &mut self,
        section: usize,
        time_slot: usize,
        maybe_room: &Option<usize>,
        undo: &mut Vec<PlacementLogEntry>,
    ) {
        let placement = &mut self.placements[section];
        assert!(placement.time_slot.is_none() && placement.room.is_none(), "add_placement: already placed");
        placement.time_slot = Some(time_slot);
        if let &Some(room) = maybe_room {
            self.room_placements[room].used_time_slots.push(TimeSlotPlacement { section, time_slot });
        }
        placement.room = *maybe_room;

        undo.push(PlacementLogEntry::Add { section });
    }

    pub fn displace_conflicts(
        &mut self,
        input: &Input,
        section: usize,
        time_slot: usize,
        maybe_room: &Option<usize>,
        undo: &mut Vec<PlacementLogEntry>,
    ) {
        let mut evictees = Vec::new();

        // check for hard conflicts in overlapping time slots
        for &hard_conflict in &input.sections[section].hard_conflicts {
            if let &Some(other_time_slot) = &self.placements[hard_conflict].time_slot {
                if input.time_slot_conflicts[time_slot][other_time_slot] {
                    evictees.push(hard_conflict);
                }
            }
        }

        // check if the room is already occupied
        if let &Some(room) = maybe_room {
            for &TimeSlotPlacement { time_slot: other_time_slot, section: room_conflict } in
                &self.room_placements[room].used_time_slots
            {
                if input.time_slot_conflicts[time_slot][other_time_slot] {
                    evictees.push(room_conflict);
                }
            }
        }

        evictees.sort_unstable();
        evictees.dedup();
        for elt in evictees {
            self.remove_placement(elt, undo);
        }
    }

    // returns values
    //   Some((time_based, taboo)):
    //   - time_based is true if the conflict would hold regardless of room
    //   - taboo indicates one or more of the conflicts were sections in the taboo list
    //   None:
    //   - no hard conflicts
    pub fn has_hard_conflict(
        &self,
        input: &Input,
        section: usize,
        time_slot: usize,
        maybe_room: &Option<usize>,
        taboo: &[usize],
    ) -> Option<(bool, bool)> {
        let mut found = false;
        let mut time_based = false;
        let mut with_taboo = false;

        // check for hard conflicts in overlapping time slots
        for &hard_conflict in &input.sections[section].hard_conflicts {
            if let &Some(other_time_slot) = &self.placements[hard_conflict].time_slot {
                if input.time_slot_conflicts[time_slot][other_time_slot] {
                    found = true;
                    time_based = true;
                    if taboo.contains(&hard_conflict) {
                        with_taboo = true;
                    }
                }
            }
        }

        // check if the room is already occupied
        if let &Some(room) = maybe_room {
            for &TimeSlotPlacement { time_slot: other_time_slot, section: hard_conflict } in
                &self.room_placements[room].used_time_slots
            {
                if input.time_slot_conflicts[time_slot][other_time_slot] {
                    found = true;
                    if taboo.contains(&hard_conflict) {
                        with_taboo = true;
                    }
                }
            }
        }

        if found {
            Some((time_based, with_taboo))
        } else {
            None
        }
    }
}

// move a section:
// *   remove it from its old placement if applicable
// *   displace any sections with hard conflicts in the new location
// *   place the section in its new home
// *   record the steps taken
// *   update the score based on the move
//
// returns a log with enough information to revert the move
pub fn move_section(
    input: &Input,
    schedule: &mut Schedule,
    section: usize,
    time_slot: usize,
    maybe_room: &Option<usize>,
) -> PlacementLog {
    // note: we leave unplaced section penalties in place and use them
    // to track which sections were placed before we started moving

    // perform the moves without any scoring updates
    let mut moves = Vec::new();
    schedule.remove_placement(section, &mut moves);
    schedule.displace_conflicts(input, section, time_slot, maybe_room, &mut moves);
    schedule.add_placement(section, time_slot, maybe_room, &mut moves);

    // gather list of sections moved
    let sections_moved = get_sections_from_log_entry_list(&moves);

    // find all criteria affecting the neighborhood
    let criteria = get_criteria_affecting_sections(input, &sections_moved);

    // clear penalty records for the the affected criteria
    // and the affected sections
    clear_penalties_for_criteria(input, schedule, &criteria);
    reset_scores_for_sections(schedule, &sections_moved);

    // compute the new penalties and update all affected sections
    compute_penalties_for_criteria(input, schedule, &criteria);

    PlacementLog { moves, criteria }
}

fn revert_move(input: &Input, schedule: &mut Schedule, log: &PlacementLog) {
    // the section placement functions want to record their moves,
    // but we will just throw it away afterward
    let mut dev_null = Vec::new();

    // gather list of sections moved
    let sections_moved = get_sections_from_log_entry_list(&log.moves);

    // perform the moves without any scoring updates
    for entry in log.moves.iter().rev() {
        match entry {
            PlacementLogEntry::Add { section } => {
                schedule.remove_placement(*section, &mut dev_null);
            }
            PlacementLogEntry::Remove { section, time_slot, room } => {
                schedule.add_placement(*section, *time_slot, room, &mut dev_null);
            }
        }
    }

    // find all criteria affecting the neighborhood
    //let criteria = get_criteria_affecting_sections(input, &sections_moved);

    // clear penalty records for the the affected criteria
    // and the affected sections
    clear_penalties_for_criteria(input, schedule, &log.criteria);
    reset_scores_for_sections(schedule, &sections_moved);

    // compute the new penalties and update all affected sections
    compute_penalties_for_criteria(input, schedule, &log.criteria);
}

// calculate the score delta that would happen if this move was applied
fn speculative_move_section(
    input: &Input,
    schedule: &mut Schedule,
    section: usize,
    time_slot: usize,
    maybe_room: &Option<usize>,
) -> Score {
    // move the sections, which does not update scoring
    let mut moves = Vec::new();
    schedule.remove_placement(section, &mut moves);
    schedule.displace_conflicts(input, section, time_slot, maybe_room, &mut moves);
    schedule.add_placement(section, time_slot, maybe_room, &mut moves);

    // gather list of sections moved
    let sections_moved = get_sections_from_log_entry_list(&moves);

    // find all criteria affecting the neighborhood
    let criteria = get_criteria_affecting_sections(input, &sections_moved);

    // calculate the score delta
    let mut delta = Score::new();
    for criterion in criteria {
        // subtract the old penalties
        for penalty in &schedule.penalties[criterion] {
            delta -= penalty.get_priority();
        }

        // add the new penalties
        for penalty in input.criteria[criterion].check(input, schedule) {
            delta += penalty.get_priority();
        }
    }

    // update unplaced section score
    for section in sections_moved {
        // if it was unplaced before, clear that from delta
        if !schedule.placements[section].score.is_placed() {
            delta -= LEVEL_FOR_UNPLACED_SECTION;
        }

        // if it is unplaced now, add that to the delta
        if !schedule.is_placed(section) {
            delta += LEVEL_FOR_UNPLACED_SECTION;
        }
    }

    // move the sections back
    let mut dev_null = Vec::new();
    for entry in moves.iter().rev() {
        match entry {
            PlacementLogEntry::Add { section } => {
                schedule.remove_placement(*section, &mut dev_null);
            }
            PlacementLogEntry::Remove { section, time_slot, room } => {
                schedule.add_placement(*section, *time_slot, room, &mut dev_null);
            }
        }
    }

    delta
}

fn get_sections_from_log_entry_list(list: &Vec<PlacementLogEntry>) -> Vec<usize> {
    let mut sections = Vec::new();
    for entry in list {
        match *entry {
            PlacementLogEntry::Add { section } => sections.push(section),
            PlacementLogEntry::Remove { section, .. } => sections.push(section),
        }
    }
    sections.sort_unstable();
    sections.dedup();
    sections
}

fn get_criteria_affecting_sections(input: &Input, sections: &[usize]) -> Vec<usize> {
    let mut criteria = Vec::new();
    for &section in sections {
        criteria.extend_from_slice(&input.sections[section].criteria);
    }
    criteria.sort_unstable();
    criteria.dedup();
    criteria
}

fn clear_penalties_for_criteria(input: &Input, schedule: &mut Schedule, criteria: &[usize]) {
    for &criterion in criteria {
        let penalties = take(&mut schedule.penalties[criterion]);
        if penalties.is_empty() {
            continue;
        }

        // merge the scores
        let mut delta = Score::new();
        for penalty in penalties {
            delta += penalty.get_priority();
        }

        // withdraw it from the global score
        schedule.score -= delta;

        // and from all affected sections
        // (including unplaced sections)
        for section in input.criteria[criterion].get_culpable_sections() {
            schedule.placements[section].score -= delta;
        }
    }
}

fn reset_scores_for_sections(schedule: &mut Schedule, sections: &[usize]) {
    for &section in sections {
        // if it was unplaced before, clear that from global and local score
        if !schedule.placements[section].score.is_placed() {
            schedule.score -= LEVEL_FOR_UNPLACED_SECTION;
            schedule.placements[section].score -= LEVEL_FOR_UNPLACED_SECTION;
        }

        // section scores should now be zero
        assert!(schedule.placements[section].score.is_zero());

        // if it is unplaced now, add that to both scores
        if !schedule.is_placed(section) {
            schedule.score += LEVEL_FOR_UNPLACED_SECTION;
            schedule.placements[section].score += LEVEL_FOR_UNPLACED_SECTION;
        }
    }
}

fn compute_penalties_for_criteria(input: &Input, schedule: &mut Schedule, criteria: &[usize]) {
    for &criterion in criteria {
        assert!(schedule.penalties[criterion].is_empty());

        let penalties = input.criteria[criterion].check(input, schedule);
        if penalties.is_empty() {
            continue;
        }

        // merge the scores
        let mut delta = Score::new();
        for penalty in &penalties {
            delta += penalty.get_priority();
        }

        // apply to the global score
        schedule.score += delta;

        // apply to any culpable sections
        // (including unplaced sections)
        for section in input.criteria[criterion].get_culpable_sections() {
            schedule.placements[section].score += delta;
        }

        schedule.penalties[criterion] = penalties;
    }
}

pub fn solve(
    config: &GenOpts,
    input: &Input,
    schedule: &mut Schedule,
    seconds: u64,
    save_id: &mut Option<i64>,
) -> Schedule {
    let mut best = schedule.clone();
    let mut walk = Walk::new(best.score);
    let mut bias = config.bias_min;
    let mut bias_delta = config.bias_step;

    let start = Instant::now();
    let mut last_report_seconds = 0;

    // one big step per iteration
    loop {
        // check if we need to report and adjust the bias
        let elapsed = start.elapsed().as_secs();
        if elapsed != last_report_seconds && elapsed % config.update_seconds == 0 {
            last_report_seconds = elapsed;
            println!(
                "{}: best {}, home {}, bias {}, walked [{},{}] steps away from home in {}",
                sec_to_string(last_report_seconds),
                best.score,
                walk.best_score_since_rehome,
                bias,
                commas(walk.min_distance_this_interval),
                commas(walk.max_distance_this_interval),
                sec_to_string(config.update_seconds),
            );
            if last_report_seconds >= seconds {
                break;
            }
            bias += bias_delta;
            if bias <= config.bias_min || bias >= config.bias_max {
                bias_delta = -bias_delta;
            }
            walk.max_distance_this_interval = walk.distance();
            walk.min_distance_this_interval = walk.distance();
        }

        // random walk: back up or move forward one big step
        // add bias to stepping backward if we have unplaced sections
        let roll = fastrand::f64() * 100.0;
        if walk.distance() == 0 || roll < 50.0 + bias {
            // make one big step forward
            if !walk.step_forward(input, schedule) {
                // unrecoverable failure?
                if schedule.score.is_zero() {
                    println!("perfect score found, quitting search");
                    break;
                } else if walk.distance() == 0 {
                    println!("cannot go forward or backward, giving up");
                    break;
                }

                // fall back
                let pre_distance = walk.distance();
                walk.fall_back(input, schedule);
                let post_distance = walk.distance();
                println!(
                    "random walk hit a wall, falling back from {} to {} steps from home",
                    commas(pre_distance),
                    commas(post_distance)
                );

                // time to rehome?
                let since_rehome = walk.time_of_rehome.elapsed().as_secs();
                if schedule.score == best.score && since_rehome >= config.rehome_global_seconds
                    || schedule.score != best.score && since_rehome >= config.rehome_local_seconds
                {
                    println!("no improvement for {} seconds, rehoming", commas(since_rehome));
                    walk.rehome(schedule.score);
                    bias = config.bias_min;
                    bias_delta = config.bias_step;
                } else if bias_delta > 0.0 && bias > config.bias_min {
                    bias_delta = -bias_delta;
                }
                continue;
            }

            if schedule.score < best.score {
                println!("new best found {} steps from home", commas(walk.distance()));
                //walk.try_dfs(input, schedule, config.dfs_depth, true);
                best = schedule.clone();
                walk.rehome(schedule.score);
                bias = config.bias_min;
                bias_delta = config.bias_step;
                let msg = format!(
                    "found with random walk after {} seconds, {} big steps, and {} little steps",
                    commas(start.elapsed().as_secs()),
                    commas(walk.big_step_count),
                    commas(walk.little_step_count)
                );
                match save_schedule(&config.db_path, input, schedule, &msg, *save_id) {
                    Ok(new_id) => *save_id = Some(new_id),
                    Err(e) => {
                        println!("quitting due to save error: {}", e);
                        return best;
                    }
                }
            } else if schedule.score < walk.best_score_since_rehome {
                println!("new local best found {} steps from home", commas(walk.distance()));
                //walk.try_dfs(input, schedule, config.dfs_depth, true);
                walk.rehome(schedule.score);
                bias = config.bias_min;
                bias_delta = config.bias_step;
            }
        } else {
            walk.step_back(input, schedule);
        }
    }
    println!(
        "took {} big steps, average of {:.1} little steps each",
        commas(walk.big_step_count),
        walk.little_step_count as f64 / walk.big_step_count as f64
    );

    best
}

pub struct Walk {
    pub taboo: Vec<usize>,
    pub step_log: Vec<PlacementLog>,
    pub big_step_size: Vec<usize>,

    pub max_distance_this_interval: usize,
    pub min_distance_this_interval: usize,

    pub best_score_since_rehome: Score,
    pub time_of_rehome: Instant,

    pub big_step_count: usize,
    pub little_step_count: usize,
}

impl Walk {
    pub fn new(score: Score) -> Self {
        Walk {
            taboo: Vec::new(),
            step_log: Vec::new(),
            big_step_size: Vec::new(),

            max_distance_this_interval: 0,
            min_distance_this_interval: 0,

            best_score_since_rehome: score,
            time_of_rehome: Instant::now(),

            big_step_count: 0,
            little_step_count: 0,
        }
    }

    pub fn distance(&self) -> usize {
        self.big_step_size.len()
    }

    pub fn rehome(&mut self, score: Score) {
        self.taboo.clear();
        self.step_log.clear();
        self.big_step_size.clear();

        self.max_distance_this_interval = 0;
        self.min_distance_this_interval = 0;

        self.best_score_since_rehome = score;
        self.time_of_rehome = Instant::now();
    }

    pub fn step_forward(&mut self, input: &Input, schedule: &mut Schedule) -> bool {
        let pre_steps = self.step_log.len();

        if !step_down(input, schedule, self) {
            return false;
        }

        climb(input, schedule, &mut self.step_log, &self.taboo);
        let new_steps = self.step_log.len() - pre_steps;

        self.big_step_size.push(new_steps);
        self.max_distance_this_interval = max(self.max_distance_this_interval, self.distance());

        self.little_step_count += new_steps;
        self.big_step_count += 1;

        true
    }

    pub fn try_dfs(&mut self, input: &Input, schedule: &mut Schedule, depth: usize, repeat: bool) {
        let start = Instant::now();
        let pre_steps = self.step_log.len();
        let mut post_steps = pre_steps;
        loop {
            print!(".");
            _ = std::io::stdout().flush();
            depth_first_search(input, schedule, self, depth);
            let latest = self.step_log.len();
            if latest == post_steps {
                break;
            }
            post_steps = latest;
            if !repeat {
                break;
            }
        }
        let elapsed = start.elapsed().as_millis();
        let new_steps = post_steps - pre_steps;

        if new_steps > 0 {
            if !self.big_step_size.is_empty() {
                // consider any steps taken as part of the previous big step
                *self.big_step_size.last_mut().unwrap() += new_steps;
                self.little_step_count += new_steps;
            }
            println!(" dfs improved in {} steps in {}", new_steps, ms_to_string(elapsed));
        } else {
            println!(" dfs attempt took {}", ms_to_string(elapsed));
        }
    }

    pub fn step_back(&mut self, input: &Input, schedule: &mut Schedule) {
        for _ in 0..self.big_step_size.pop().unwrap() {
            revert_move(input, schedule, &self.step_log.pop().unwrap());
        }
        self.taboo.pop();
        self.min_distance_this_interval = min(self.min_distance_this_interval, self.distance());
    }

    pub fn fall_back(&mut self, input: &Input, schedule: &mut Schedule) {
        for _ in self.big_step_size.len() / 4..self.big_step_size.len() {
            self.step_back(input, schedule);
        }
    }
}

fn commas<T: TryInto<i64>>(n: T) -> String {
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

pub fn string_to_sec(duration: &str) -> Result<u64, String> {
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
            _ => return Err(format!("failed to parse {duration}; expected, e.g., 2h5m13s")),
        }
    }
    if digits != 0 {
        Err(format!("failed to parse {duration}; expected, e.g.: 2h5m13s but found extra digits at end"))
    } else {
        Ok(seconds as u64)
    }
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

pub fn sec_to_string(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{}s", seconds);
    }
    if seconds < 3600 {
        return format!("{}m{:02}s", seconds / 60, seconds % 60);
    }
    format!("{}h{:02}m{:02}s", seconds / 3600, (seconds % 3600) / 60, seconds % 60)
}

fn rooms_adapter(rooms: &[RoomWithOptionalPriority]) -> Vec<Option<usize>> {
    if rooms.is_empty() {
        vec![None]
    } else {
        rooms.iter().map(|&RoomWithOptionalPriority { room, .. }| Some(room)).collect()
    }
}

pub fn warmup(input: &Input, seconds: u64) -> Option<Schedule> {
    let start = Instant::now();
    let mut best = None;
    let mut best_pre_climb_score = None;
    let mut count = 0;
    let taboo = Vec::new();
    while start.elapsed().as_secs() < seconds {
        count += 1;
        let mut schedule = Schedule::new(input);
        while schedule.score.unplaced() > 0 {
            // find the most-constrained section
            // and the number of room/time combos available to it
            let mut most_constrained = (None, None);
            for section in 0..schedule.placements.len() {
                // only examine unplaced sections
                if schedule.placements[section].time_slot.is_some() {
                    continue;
                }

                // find the number of placement options for this section
                let mut count = 0;
                'time_loop: for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                    for maybe_room in &rooms_adapter(&input.sections[section].rooms) {
                        match schedule.has_hard_conflict(input, section, time_slot, maybe_room, &taboo) {
                            Some((true, _)) => continue 'time_loop,
                            Some(_) => continue,
                            None => count += 1,
                        }
                    }
                }

                // if this section cannot be placed, skip it
                if count == 0 {
                    continue;
                }

                // is this the most constrained section so far?
                match most_constrained {
                    (_, Some(n)) if count >= n => {}
                    _ => {
                        most_constrained = (Some(section), Some(count));
                    }
                }
            }

            // did we find any sections that can be placed?
            let (Some(section), Some(options)) = most_constrained else {
                break;
            };

            // randomly choose one of the available placements
            let winner = fastrand::usize(1..=options);

            // find that placement
            let mut count = 0;
            'time_loop: for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                for maybe_room in &rooms_adapter(&input.sections[section].rooms) {
                    match schedule.has_hard_conflict(input, section, time_slot, maybe_room, &taboo) {
                        Some((true, _)) => continue 'time_loop,
                        Some(_) => continue,
                        None => {
                            count += 1;
                            if count == winner {
                                let _undo = move_section(input, &mut schedule, section, time_slot, maybe_room);
                                break 'time_loop;
                            }
                        }
                    }
                }
            }
        }

        // best pre-climb score we've seen?
        match best_pre_climb_score {
            Some(best_pre_climb_score) if best_pre_climb_score < schedule.score => {}

            _ => {
                best_pre_climb_score = Some(schedule.score);

                // do a climb
                let mut log = Vec::new();
                climb(input, &mut schedule, &mut log, &taboo);

                // is this a new best?
                match best {
                    Some(Schedule { score, .. }) if score <= schedule.score => {}

                    _ => {
                        let score = schedule.score;
                        best = Some(schedule);
                        if score.is_zero() {
                            println!("perfect score found, quitting warmup");
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("warmup tried {} schedules", count);
    best
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub section: usize,
    pub time_slot: Option<usize>,
    pub room: Option<usize>,
}

pub fn climb(input: &Input, schedule: &mut Schedule, log: &mut Vec<PlacementLog>, taboo: &[usize]) {
    let zero = Score::new();
    let mut by_score = Vec::new();
    for i in 0..input.sections.len() {
        by_score.push(i);
    }

    // keep making greedy, single-step changes until no single-step improvements are possible
    loop {
        let mut best_delta = None;
        let mut best_move = None;

        // examine sections from highest current score to lowest
        by_score.sort_unstable_by_key(|section| zero - schedule.placements[*section].score);
        for &section in &by_score {
            // moving a section with zero score can only make things worse
            if schedule.placements[section].score.is_zero() {
                break;
            }

            // the best we can hope for is dropping this section's score to zero,
            // so if we already have a move better than that then stop searching
            if let Some(best) = best_delta {
                if zero - best > schedule.placements[section].score {
                    break;
                }
            }

            // skip taboo moves
            if taboo.contains(&section) {
                continue;
            }

            // try each time slot/room combination
            'time_loop: for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                for room in rooms_adapter(&input.sections[section].rooms) {
                    // the current location is off limits, i.e., no moves that do not move anything
                    if schedule.placements[section].time_slot == Some(time_slot)
                        && schedule.placements[section].room == room
                    {
                        continue;
                    }

                    // not allowed to displace anything from the taboo list
                    if let Some((time_based, taboo)) =
                        schedule.has_hard_conflict(input, section, time_slot, &room, taboo)
                    {
                        if taboo {
                            if time_based {
                                continue 'time_loop;
                            }
                            continue;
                        }
                    };

                    let candidate = Move { section, time_slot: Some(time_slot), room };
                    let delta = try_one_move(input, schedule, &candidate);

                    // only consider moves that were improvements
                    if delta < zero && best_delta.map_or(true, |best| delta < best) {
                        best_delta = Some(delta);
                        best_move = Some(candidate);
                    }
                }
            }
        }

        // did we find an improving move?
        let Some(Move { section, time_slot: Some(ts), room }) = best_move else {
            // no viable moves found
            break;
        };

        // apply the move, but do not add it to the taboo list
        let log_entry = move_section(input, schedule, section, ts, &room);
        log.push(log_entry);
    }
}

// try a single section move then undo it, and return the score delta it created
pub fn try_one_move(input: &Input, schedule: &mut Schedule, candidate_move: &Move) -> Score {
    let &Move { section, time_slot: Some(ts), room } = candidate_move else {
        panic!("try_one_move called with no time slot");
    };
    speculative_move_section(input, schedule, section, ts, &room)
}

pub fn step_down(input: &Input, schedule: &mut Schedule, walk: &mut Walk) -> bool {
    // gather a list of potential moves with their local scores

    // for each section
    let mut candidates = Vec::new();
    for section in 0..input.sections.len() {
        // skip taboo moves
        if walk.taboo.contains(&section) {
            continue;
        }

        // try each time slot
        'time_loop: for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
            for room in rooms_adapter(&input.sections[section].rooms) {
                let candidate = Move { section, time_slot: Some(time_slot), room };

                // the current location is off limits, i.e., no moves that do not move anything
                if schedule.placements[section].time_slot == candidate.time_slot
                    && schedule.placements[section].room == candidate.room
                {
                    continue;
                }

                // not allowed to displace anything from the taboo list
                if let Some((time_based, taboo)) =
                    schedule.has_hard_conflict(input, section, time_slot, &room, &walk.taboo)
                {
                    if taboo {
                        if time_based {
                            continue 'time_loop;
                        }
                        continue;
                    }
                };

                candidates.push((schedule.placements[section].score.first_nonzero(), candidate));
            }
        }
    }

    // no moves possible?
    if candidates.is_empty() {
        return false;
    }

    // sort by highest priority first
    candidates.sort_unstable();

    // group by priority levels
    let mut candidate = None;
    for chunk in candidates.chunk_by(|(a, _), (b, _)| a == b) {
        // toss a coin at each priority level to use it or move on
        if fastrand::bool() {
            // group this priority level by section
            let by_section: Vec<&[(u8, Move)]> =
                chunk.chunk_by(|(_, Move { section: a, .. }), (_, Move { section: b, .. })| a == b).collect();

            // pick a section
            let by_section_index = fastrand::usize(0..by_section.len());
            let one_section = &by_section[by_section_index];

            // pick a placement for that section
            let index = fastrand::usize(0..one_section.len());
            candidate = Some(one_section[index].1.clone());
            break;
        }
    }
    if candidate.is_none() {
        let index = fastrand::usize(0..candidates.len());
        candidate = Some(candidates[index].1.clone());
    }
    let Some(Move { section, time_slot: Some(ts), room }) = candidate else {
        return false;
    };

    // apply the move and add the section that was moved to the taboo list
    walk.taboo.push(section);
    let log_entry = move_section(input, schedule, section, ts, &room);
    walk.step_log.push(log_entry);

    true
}

pub fn depth_first_search(input: &Input, schedule: &mut Schedule, walk: &mut Walk, depth: usize) {
    // the best sequence of steps we have found so far and its score
    let mut best_moves = Vec::new();
    let mut best_score = schedule.score;

    // the current sequence
    let mut current = Vec::new();

    // kick off the search
    dfs_helper(input, schedule, walk, depth - 1, &mut best_moves, &mut best_score, &mut current);

    // apply the moves if any
    for elt in best_moves {
        let Move { section, time_slot, room } = elt;
        walk.step_log.push(move_section(input, schedule, section, time_slot.unwrap(), &room));
    }
}

fn dfs_helper(
    input: &Input,
    schedule: &mut Schedule,
    walk: &mut Walk,
    depth: usize,
    best_moves: &mut Vec<Move>,
    best_score: &mut Score,
    current: &mut Vec<Move>,
) {
    // for each section
    for section in 0..input.sections.len() {
        // ignore taboo sections and sections with zero scores
        if walk.taboo.contains(&section) || schedule.placements[section].score.is_zero() {
            continue;
        }

        // at a leaf in the search? see if moving this section even has a possiblity
        // of improving on the best score so far
        if depth == 0 && schedule.score - schedule.placements[section].score >= *best_score {
            continue;
        }

        // try each time slot/room combination
        'time_loop: for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
            for room in rooms_adapter(&input.sections[section].rooms) {
                // the current location is off limits, i.e., no moves that do not move anything
                if schedule.placements[section].time_slot == Some(time_slot)
                    && schedule.placements[section].room == room
                {
                    continue;
                }

                if let Some((time_based, taboo)) =
                    schedule.has_hard_conflict(input, section, time_slot, &room, &walk.taboo)
                {
                    // not allowed to displace anything from the taboo list
                    // and if we are at a leaf in the search, displacing anything
                    // is unlikely to help matters
                    if taboo || depth == 0 {
                        if time_based {
                            continue 'time_loop;
                        }
                        continue;
                    }
                };

                // make the move
                let candidate = Move { section, time_slot: Some(time_slot), room };
                if depth == 0 {
                    // special case for leaf of search
                    let delta = try_one_move(input, schedule, &candidate);
                    if schedule.score + delta < *best_score {
                        current.push(candidate);
                        *best_moves = current.clone();
                        *best_score = schedule.score + delta;
                        current.pop();
                    }
                } else {
                    walk.taboo.push(section);
                    walk.step_log.push(move_section(input, schedule, section, time_slot, &room));
                    current.push(candidate);

                    // improvement?
                    if schedule.score < *best_score {
                        *best_moves = current.clone();
                        *best_score = schedule.score;
                    }

                    // recursive call
                    dfs_helper(input, schedule, walk, depth - 1, best_moves, best_score, current);

                    // undo the move
                    walk.taboo.pop();
                    revert_move(input, schedule, &walk.step_log.pop().unwrap());
                    current.pop();
                }
            }
        }
    }
}
