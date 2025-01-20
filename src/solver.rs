//use std::sync::{Arc, Mutex};
//use std::sync::mpsc;
//use std::thread;
use super::input::*;
use super::score::*;
use rand::Rng;

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
    pub neighborhood: Vec<usize>,
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
        if let Some(time_slot) = std::mem::take(&mut self.placements[section].time_slot) {
            // does it have a room?
            let room = if let Some(room) = std::mem::take(&mut self.placements[section].room) {
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
fn move_section(
    schedule: &mut Schedule,
    input: &Input,
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

    // gather the scoring neighborhood around the moved sections
    let neighborhood = get_neigborhood_of_sections_list(input, &sections_moved);

    // find all criteria affecting the neighborhood
    let criteria = get_criteria_affecting_sections(input, &neighborhood);

    // clear penalty records for the the affected criteria
    // and the affected sections
    clear_penalties_for_criteria(schedule, &criteria);
    reset_scores_for_sections(schedule, &neighborhood);

    // compute the new penalties and update all affected sections
    compute_penalties_for_criteria(input, schedule, &criteria);

    PlacementLog { moves, neighborhood, criteria }
}

fn revert_move(input: &Input, schedule: &mut Schedule, log: &PlacementLog) {
    // the section placement functions want to record their moves,
    // but we will just throw it away afterward
    let mut dev_null = Vec::new();

    // gather list of sections moved
    //let sections_moved = get_sections_from_log_entry_list(&log.moves);

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

    // gether the scoring neighborhood around the moved sections
    //let neighborhood = get_neigborhood_of_sections_list(input, &sections_moved);

    // find all criteria affecting the neighborhood
    //let criteria = get_criteria_affecting_sections(input, &neighborhood);

    // clear penalty records for the the affected criteria
    // and the affected sections
    clear_penalties_for_criteria(schedule, &log.criteria);
    reset_scores_for_sections(schedule, &log.neighborhood);

    // compute the new penalties and update all affected sections
    compute_penalties_for_criteria(input, schedule, &log.criteria);
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

fn get_neigborhood_of_sections_list(input: &Input, sections: &[usize]) -> Vec<usize> {
    let mut neighborhood = Vec::new();
    for &section in sections {
        neighborhood.push(section);
        for &neighbor in &input.sections[section].neighbors {
            neighborhood.push(neighbor);
        }
    }
    neighborhood.sort_unstable();
    neighborhood.dedup();
    neighborhood
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

fn clear_penalties_for_criteria(schedule: &mut Schedule, criteria: &[usize]) {
    for &criterion in criteria {
        // clear the impact of these scores on the global score
        for penalty in std::mem::take(&mut schedule.penalties[criterion]) {
            schedule.score -= penalty.get_priority();
        }
    }
}

fn reset_scores_for_sections(schedule: &mut Schedule, sections: &[usize]) {
    for &section in sections {
        // if it was unplaced before, clear that from the global score
        if !schedule.placements[section].score.is_placed() {
            schedule.score -= LEVEL_FOR_UNPLACED_SECTION;
        }

        // clear the score
        schedule.placements[section].score = Score::new();

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
        for elt in &penalties {
            delta += elt.get_priority();
        }

        // apply to the global score
        schedule.score += delta;

        // apply to any culpable sections that are placed
        for section in input.criteria[criterion].get_culpable_sections() {
            if schedule.is_placed(section) {
                schedule.placements[section].score += delta;
            }
        }

        schedule.penalties[criterion] = penalties;
    }
}

/*
pub fn concucrrent_solve(input: &Input, schedule: &Schedule, start: std::time::Instance, seconds: u64) -> Schedule {
    let best = Arc::new(Mutex::new(schedule.clone()));
    let (tx, rx) = mpsc::channel();
    let cpus = thread::available_parallelism().unwrap().get();

    // fan out
    for i in 0..cpus {
        let tx = tx.clone();
        let best = Arc::clone(best);
        let input = input.clone();
        thread::spawn(move || {
        });
    }
}
*/

const TABOO_LIMIT: usize = 50;
const BIAS: i64 = -5;

pub fn solve(input: &Input, schedule: &mut Schedule, start: std::time::Instant, seconds: u64) -> Schedule {
    let mut best = schedule.clone();
    let mut last_report = start.elapsed().as_secs();
    let mut big_steps = 0;
    let mut little_steps = 0;

    let mut log = Vec::new();
    let mut taboo = Vec::new();
    let mut big_step_size = Vec::new();
    let mut failed_forward = false;

    loop {
        assert!(log.len() == taboo.len());
        if start.elapsed().as_secs() != last_report {
            last_report = start.elapsed().as_secs();
            println!(
                "{:3} second{}: best {}, current is {:3} steps away with score {}",
                last_report,
                if last_report == 1 { "" } else { "s" },
                best.score,
                big_step_size.len(),
                schedule.score,
            );
            if last_report >= seconds {
                break;
            }
        }
        if failed_forward && big_step_size.is_empty() {
            println!("cannot go forward or backward, giving up");
            break;
        }

        // random walk: back up or move forward one big step
        // add bias to stepping backward if we have unplaced sections
        let roll = rand::thread_rng().gen_range(1..=100);
        let cutoff = 50 + BIAS - std::cmp::min(10, schedule.score.unplaced() as i64);
        if big_step_size.is_empty() || roll <= cutoff {
            // make one big step forward
            let pre_steps = taboo.len();
            failed_forward = !step_down(input, schedule, &mut log, &mut taboo);
            if failed_forward {
                if schedule.score.is_zero() {
                    println!("perfect score found, quitting search");
                    break;
                }
                println!("failed to make step down move");
                continue;
            }
            climb(input, schedule, &mut log, &mut taboo);
            let steps = taboo.len() - pre_steps;
            big_step_size.push(steps);

            big_steps += 1;
            little_steps += steps;

            if schedule.score < best.score {
                println!(
                    "new best found {:3} big steps and {:3} small steps from previous best",
                    big_step_size.len(),
                    taboo.len()
                );

                // reset so this is now the starting point
                log.clear();
                taboo.clear();
                big_step_size.clear();
                best = schedule.clone();
            }
        } else {
            // step backward
            let steps = big_step_size.pop().unwrap();
            for _ in 0..steps {
                let _ = taboo.pop().unwrap();
                let undo = log.pop().unwrap();
                revert_move(input, schedule, &undo);
            }
        }
    }
    println!("took {} big steps and {} little steps", big_steps, little_steps);

    best
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
            let winner = rand::thread_rng().gen_range(1..=options);

            // find that placement
            let mut count = 0;
            'time_loop: for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                for maybe_room in &rooms_adapter(&input.sections[section].rooms) {
                    if !schedule.has_hard_conflict(input, section, time_slot, maybe_room) {
                        count += 1;
                        if count == winner {
                            let _undo = move_section(&mut schedule, input, section, time_slot, maybe_room);
                            break 'time_loop;
                        }
                    }
                }
            }
        }

        // is this a new best?
        match best {
            Some(Schedule { score, .. }) if score <= schedule.score => {}

            _ => {
                // do a climb
                let mut log = Vec::new();
                let mut taboo = Vec::new();
                climb(input, &mut schedule, &mut log, &mut taboo);
                let score = schedule.score;
                best = Some(schedule);
                if score.is_zero() {
                    println!("perfect score found, quitting warmup");
                    break;
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

pub fn climb(input: &Input, schedule: &mut Schedule, log: &mut Vec<PlacementLog>, taboo: &mut Vec<Move>) {
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
                    let taboo_last = std::cmp::min(std::cmp::max(taboo.len() / 2, TABOO_LIMIT), taboo.len());
                    if taboo[taboo.len() - taboo_last..].contains(&candidate) {
                        continue;
                    }

                    // the current location is off limits, too
                    if schedule.placements[section].time_slot == candidate.time_slot
                        && schedule.placements[section].room == candidate.room
                    {
                        continue;
                    }

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

        // apply the move and add the configuration it displaced to the taboo list
        taboo.push(Move {
            section,
            time_slot: schedule.placements[section].time_slot,
            room: schedule.placements[section].room,
        });
        let log_entry = move_section(schedule, input, section, ts, &room);
        log.push(log_entry);
    }
}

// try a single section move then undo it, and return the score delta it created
pub fn try_one_move(input: &Input, schedule: &mut Schedule, candidate_move: &Move) -> Score {
    let &Move { section, time_slot: Some(ts), room } = candidate_move else {
        panic!("try_one_move called with no time slot");
    };
    let old_score = schedule.score;
    let undo = move_section(schedule, input, section, ts, &room);
    let new_score = schedule.score;
    revert_move(input, schedule, &undo);
    new_score - old_score
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
                let taboo_last = std::cmp::min(std::cmp::max(taboo.len() / 2, TABOO_LIMIT), taboo.len());
                if taboo[taboo.len() - taboo_last..].contains(&candidate) {
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
        if rand::thread_rng().gen_range(0..=1) == 0 {
            // group this priority level by section
            let by_section: Vec<&[(u8, Move)]> =
                chunk.chunk_by(|(_, Move { section: a, .. }), (_, Move { section: b, .. })| a == b).collect();

            // pick a section
            let by_section_index = rand::thread_rng().gen_range(0..by_section.len());
            let one_section = &by_section[by_section_index];

            // pick a placement for that section
            let index = rand::thread_rng().gen_range(0..one_section.len());
            candidate = Some(one_section[index].1.clone());
            break;
        }
    }
    if candidate.is_none() {
        let index = rand::thread_rng().gen_range(0..candidates.len());
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
    let log_entry = move_section(schedule, input, section, ts, &room);
    log.push(log_entry);

    true
}
