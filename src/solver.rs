use super::input::*;
use super::score::*;
use super::*;
use std::mem::take;
use std::cmp::{min,max};

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

    pub fn has_hard_conflict(
        &self,
        input: &Input,
        section: usize,
        time_slot: usize,
        maybe_room: &Option<usize>,
    ) -> bool {
        // check for hard conflicts in overlapping time slots
        for &hard_conflict in &input.sections[section].hard_conflicts {
            if let &Some(other_time_slot) = &self.placements[hard_conflict].time_slot {
                if input.time_slot_conflicts[time_slot][other_time_slot] {
                    return true;
                }
            }
        }

        // check if the room is already occupied
        if let &Some(room) = maybe_room {
            for &TimeSlotPlacement { time_slot: other_time_slot, .. } in &self.room_placements[room].used_time_slots {
                if input.time_slot_conflicts[time_slot][other_time_slot] {
                    return true;
                }
            }
        }

        false
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
    input: &Input,
    schedule: &mut Schedule,
    start: std::time::Instant,
    seconds: u64,
    save_id: i64,
) -> Schedule {
    let mut best = schedule.clone();
    let mut last_report = start.elapsed().as_secs();
    let mut best_seconds = last_report;
    let mut big_steps = 0;
    let mut little_steps = 0;

    let mut log = Vec::new();
    let mut taboo = Vec::new();
    let mut big_step_size = Vec::new();
    let mut big_steps_min = 0;
    let mut big_steps_max = 0;
    let mut failed_forward = false;

    let mut best_score_this_interval = schedule.score;
    let mut best_since_rebase = schedule.score;
    let mut bias_delta = BIAS_STEP;
    let mut bias = MIN_BIAS;

    let mut moves = 0;
    let warmup_seconds = start.elapsed().as_secs();

    loop {
        let now = start.elapsed().as_secs();
        if now != last_report && now % REPORT_SECONDS == 0 {
            last_report = now;
            println!(
                "{} seconds: best {}, bias {}, [{},{}] steps away, best of last {} seconds is {}",
                commas(last_report),
                best.score,
                bias,
                commas(big_steps_min),
                commas(big_steps_max),
                REPORT_SECONDS,
                best_score_this_interval,
            );
            best_score_this_interval = schedule.score;
            big_steps_min = big_step_size.len();
            big_steps_max = big_step_size.len();
            if last_report >= seconds {
                break;
            }
            bias += bias_delta;
            if bias <= MIN_BIAS || bias >= MAX_BIAS {
                bias_delta = -bias_delta;
            }
            if last_report - best_seconds >= REBASE_SECONDS {
                println!("no improvement for {} seconds, rebasing", commas(REBASE_SECONDS));
                log.clear();
                big_step_size.clear();
                big_steps_min = 0;
                big_steps_max = 0;
                best_seconds = last_report;
                best_since_rebase = schedule.score;
                bias = MIN_BIAS;
                bias_delta = BIAS_STEP;
            }
        }
        if failed_forward && big_step_size.is_empty() {
            println!("cannot go forward or backward, giving up");
            break;
        }

        // random walk: back up or move forward one big step
        // add bias to stepping backward if we have unplaced sections
        let roll = fastrand::i64(1..=100);
        if big_step_size.is_empty() || roll <= 50 + bias {
            // make one big step forward
            let pre_steps = log.len();
            failed_forward = !step_down(input, schedule, &mut log, &mut taboo);
            if failed_forward {
                if schedule.score.is_zero() {
                    println!("perfect score found, quitting search");
                    break;
                }
                println!("failed to make step down move");
                continue;
            }
            moves += 1;
            climb(input, schedule, &mut log, &mut taboo, &mut moves);
            let steps = log.len() - pre_steps;
            big_step_size.push(steps);
            big_steps_max = max(big_steps_max, big_step_size.len());

            big_steps += 1;
            little_steps += steps;

            if schedule.score < best_score_this_interval {
                best_score_this_interval = schedule.score;
            }
            if schedule.score < best.score {
                println!("new best found {} steps from previous best", commas(big_step_size.len()));

                // reset so this is now the starting point
                log.clear();
                big_step_size.clear();
                big_steps_min = 0;
                big_steps_max = 0;
                best_since_rebase = schedule.score;
                best_seconds = start.elapsed().as_secs();
                bias = MIN_BIAS;
                bias_delta = BIAS_STEP;
                best = schedule.clone();
                let msg = format!(
                    "random walk found after {} seconds, {} big steps, and {} little steps",
                    commas(start.elapsed().as_secs()),
                    commas(big_steps),
                    commas(little_steps)
                );
                if let Err(e) = save_schedule(super::DB_PATH, input, schedule, &msg, Some(save_id)) {
                    println!("quitting due to save error: {}", e);
                    return best;
                }
            } else if schedule.score < best_since_rebase {
                log.clear();
                big_step_size.clear();
                big_steps_min = 0;
                big_steps_max = 0;
                best_since_rebase = schedule.score;
                best_seconds = start.elapsed().as_secs();
                bias = MIN_BIAS;
                bias_delta = BIAS_STEP;
                println!("new local best, rebasing");
            }
        } else {
            // step backward
            let steps = big_step_size.pop().unwrap();
            big_steps_min = min(big_steps_min, big_step_size.len());
            for _ in 0..steps {
                let undo = log.pop().unwrap();
                revert_move(input, schedule, &undo);
            }
            taboo.clear();
        }
    }
    println!("took {} big steps, average of {:.1} little steps each", commas(big_steps), little_steps as f64 / big_steps as f64);
    let solve_seconds = (seconds - warmup_seconds) as usize;
    println!("total of {} section moves ({}/s)", commas(moves), commas(moves / solve_seconds));

    best
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
        s = format!(",{:03}{}", n%1000, s);
        n /= 1000;
    }
    format!("{minus}{s}")
}

fn rooms_adapter(rooms: &[RoomWithOptionalPriority]) -> Vec<Option<usize>> {
    if rooms.is_empty() {
        vec![None]
    } else {
        rooms.iter().map(|&RoomWithOptionalPriority { room, .. }| Some(room)).collect()
    }
}

pub fn warmup(input: &Input, start: std::time::Instant, seconds: u64) -> Option<Schedule> {
    let mut best = None;
    let mut best_pre_climb_score = None;
    let mut count = 0;
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
                for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                    for maybe_room in &rooms_adapter(&input.sections[section].rooms) {
                        if !schedule.has_hard_conflict(input, section, time_slot, maybe_room) {
                            count += 1;
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
                    if !schedule.has_hard_conflict(input, section, time_slot, maybe_room) {
                        count += 1;
                        if count == winner {
                            let _undo = move_section(input, &mut schedule, section, time_slot, maybe_room);
                            break 'time_loop;
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
                let mut taboo = Vec::new();
                let mut moves = 0;
                climb(input, &mut schedule, &mut log, &mut taboo, &mut moves);

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

pub fn climb(
    input: &Input,
    schedule: &mut Schedule,
    log: &mut Vec<PlacementLog>,
    taboo: &mut Vec<Move>,
    moves: &mut usize,
) {
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

            // try each time slot
            for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                for room in rooms_adapter(&input.sections[section].rooms) {
                    let candidate = Move { section, time_slot: Some(time_slot), room };

                    // skip taboo moves
                    if taboo.contains(&candidate) {
                        continue;
                    }

                    // the current location is off limits, too
                    if schedule.placements[section].time_slot == candidate.time_slot
                        && schedule.placements[section].room == candidate.room
                    {
                        continue;
                    }

                    let delta = try_one_move(input, schedule, &candidate);
                    *moves += 1;

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

        // apply the move and add the configuration it displaced to the taboo list
        taboo.push(Move {
            section,
            time_slot: schedule.placements[section].time_slot,
            room: schedule.placements[section].room,
        });
        let log_entry = move_section(input, schedule, section, ts, &room);
        *moves += 1;
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

pub fn step_down(input: &Input, schedule: &mut Schedule, log: &mut Vec<PlacementLog>, taboo: &mut Vec<Move>) -> bool {
    let zero = Score::new();

    // gather a list of potential moves with their local scores
    // but only those with a non-zero potential for improvement

    // for each section
    let mut candidates = Vec::new();
    for section in 0..input.sections.len() {
        // skip sections with no bad scores
        if schedule.placements[section].score == zero {
            continue;
        }

        // try each time slot
        for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
            for room in rooms_adapter(&input.sections[section].rooms) {
                let candidate = Move { section, time_slot: Some(time_slot), room };

                // skip taboo moves
                if taboo.contains(&candidate) {
                    continue;
                }

                // the current location is off limits, too
                if schedule.placements[section].time_slot == candidate.time_slot
                    && schedule.placements[section].room == candidate.room
                {
                    continue;
                }

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

    // apply the move and add the configuration it displaced to the taboo list
    taboo.push(Move {
        section,
        time_slot: schedule.placements[section].time_slot,
        room: schedule.placements[section].room,
    });
    let log_entry = move_section(input, schedule, section, ts, &room);
    log.push(log_entry);

    true
}
