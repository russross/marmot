use super::input::*;
use super::score::*;
use rand::Rng;
use std::io::Write;

//
//
// Schedule data
// A single candidate schedule with section placements,
// records of scores associated with the schedule, etc.
// Includes placement delta types and actions.
//
//

// the complete schedule and its score
#[derive(Clone)]
pub struct Schedule {
    pub placements: Vec<Placement>,
    pub room_placements: Vec<RoomPlacements>,
    pub score: Score,
}

// placement details of a single section
#[derive(Clone)]
pub struct Placement {
    pub time_slot: Option<usize>,
    pub room: Option<usize>,
    pub score: PlacementScore,
}

// the entire effect on the score for a single section placement
#[derive(Clone)]
pub struct PlacementScore {
    pub local: Score,
    pub global: Score,
    pub deltas: Vec<ScoreDelta>,
}

pub struct PlacementLog {
    // to undo a move, undo the deltas in reverse order
    // and restore all of the scores
    pub entries: Vec<PlacementLogEntry>,

    // snapshot of the scores of all modified sections before the placement
    // includes sections adjacent to the sections that actually moved
    pub pre_scores: Vec<(usize, PlacementScore)>,
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
        for _ in 0..input.sections.len() {
            placements.push(Placement {
                time_slot: None,
                room: None,
                score: PlacementScore { local: Score::new(), global: Score::new(), deltas: Vec::new() },
            });
        }

        let mut room_placements = Vec::new();
        for _ in 0..input.rooms.len() {
            room_placements.push(RoomPlacements { used_time_slots: Vec::new() });
        }

        let mut schedule = Schedule { placements, room_placements, score: Score::new() };

        // compute initial score
        for section in 0..input.sections.len() {
            // compute the section score
            compute_section_score(input, &mut schedule, section);

            // apply it to the global score
            schedule.score += schedule.placements[section].score.global;
        }

        schedule
    }

    pub fn is_placed(&self, section: usize) -> bool {
        self.placements[section].time_slot.is_some()
    }

    // remove a section from its current room/time placement (if any)
    // this does not update scoring, it just clears the old placement if it existed
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

        evictees.sort();
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

impl PlacementScore {
    pub fn new() -> Self {
        PlacementScore { local: Score::new(), global: Score::new(), deltas: Vec::new() }
    }
}

impl Default for PlacementScore {
    fn default() -> Self {
        Self::new()
    }
}

impl PlacementLog {
    // move a section:
    // *   remove it from its old placement if applicable
    // *   displace any sections with hard conflicts in the new location
    // *   place the section in its new home
    // *   record the steps taken
    // *   update the score based on the move
    //
    // returns a log with enough information to revert the move
    pub fn move_section(
        schedule: &mut Schedule,
        input: &Input,
        section: usize,
        time_slot: usize,
        maybe_room: &Option<usize>,
    ) -> Self {
        let mut entries = Vec::new();

        // move the section and record displacements
        schedule.remove_placement(section, &mut entries);
        schedule.displace_conflicts(input, section, time_slot, maybe_room, &mut entries);
        schedule.add_placement(section, time_slot, maybe_room, &mut entries);

        // gather list of sections moved (deduped)
        let mut sections_being_moved = Vec::new();
        for elt in &entries {
            match *elt {
                PlacementLogEntry::Add { section } => sections_being_moved.push(section),
                PlacementLogEntry::Remove { section, .. } => sections_being_moved.push(section),
            }
        }
        sections_being_moved.sort();
        sections_being_moved.dedup();

        let mut pre_scores = Vec::new();
        let mut adjacent = Vec::new();

        for &section in &sections_being_moved {
            // gather adjacent sections based on the old scoring
            for neighbor in &input.sections[section].neighbors {
                if !sections_being_moved.contains(neighbor) {
                    adjacent.push(*neighbor);
                }
            }

            // move the old score records to the log and reset the section score
            let old_score = std::mem::take(&mut schedule.placements[section].score);

            // remove the old score from the global score
            schedule.score -= old_score.global;

            // add it to the log
            pre_scores.push((section, old_score));

            // compute the new score
            compute_section_score(input, schedule, section);

            // apply it to the global score
            schedule.score += schedule.placements[section].score.global;

            // gather adjacent sections based on the new scoring
            for neighbor in &input.sections[section].neighbors {
                if !sections_being_moved.contains(neighbor) {
                    adjacent.push(*neighbor);
                }
            }
        }

        // dedup adjacent section list
        adjacent.sort();
        adjacent.dedup();

        for &section in &adjacent {
            // move the old score record to the log and reset the section score
            let old_score = std::mem::take(&mut schedule.placements[section].score);

            // remove the old score from the global score
            schedule.score -= old_score.global;

            // add it to the log
            pre_scores.push((section, old_score));

            // compute the new score
            compute_section_score(input, schedule, section);

            // apply it to the global score
            schedule.score += schedule.placements[section].score.global;
        }

        PlacementLog { entries, pre_scores }
    }

    pub fn revert_move(&mut self, schedule: &mut Schedule) {
        // the section placement functions want to record their moves,
        // but we will just throw it away afterward
        let mut dev_null = Vec::new();

        // play the log in reverse order and undo the changes
        loop {
            match self.entries.pop() {
                Some(PlacementLogEntry::Add { section }) => {
                    schedule.remove_placement(section, &mut dev_null);
                }
                Some(PlacementLogEntry::Remove { section, time_slot, room }) => {
                    schedule.add_placement(section, time_slot, &room, &mut dev_null);
                }
                None => break,
            }
        }

        // revert all moved sections and adjacent sections to their pre-move scores
        while let Some((section, score)) = self.pre_scores.pop() {
            schedule.score -= schedule.placements[section].score.global;
            let _old_score = std::mem::replace(&mut schedule.placements[section].score, score);
            schedule.score += schedule.placements[section].score.global;
        }
    }
}

pub fn solve(input: &Input, schedule: &mut Schedule, start: std::time::Instant, seconds: u64) -> Schedule {
    {
        let mut log = Vec::new();
        let mut taboo = Vec::new();
        climb(input, schedule, &mut log, &mut taboo);
        let climb_steps = taboo.len();
        println!(
            "initial climb did {} little step{} and ended with {}",
            climb_steps,
            if climb_steps == 1 { "" } else { "s" },
            schedule.score
        );
    }

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
        let cutoff = 50 - std::cmp::min(10, schedule.score.unplaced());
        if big_step_size.is_empty() || roll <= cutoff {
            // make one big step forward
            let pre_steps = taboo.len();
            failed_forward = !step_down(input, schedule, &mut log, &mut taboo);
            if failed_forward {
                println!("failed to make step down move");
                continue;
            }
            climb(input, schedule, &mut log, &mut taboo);
            let steps = taboo.len() - pre_steps;
            big_step_size.push(steps);

            big_steps += 1;
            little_steps += steps;

            if schedule.score < best.score {
                // reset so this is now the starting point
                log.clear();
                taboo.clear();
                big_step_size.clear();

                best = schedule.clone();
                println!("new best found");
            }
        } else {
            // step backward
            let steps = big_step_size.pop().unwrap();
            for _ in 0..steps {
                let _ = taboo.pop().unwrap();
                let mut undo = log.pop().unwrap();
                undo.revert_move(schedule);
            }
        }
    }
    println!("took {} big steps and {} little steps", big_steps, little_steps);

    best
}

pub fn warmup(input: &Input, start: std::time::Instant, seconds: u64) -> Option<Schedule> {
    let mut best = None;
    let mut count = 0;
    while start.elapsed().as_secs() < seconds {
        count += 1;
        let mut schedule = Schedule::new(input);
        std::io::stdout().flush().unwrap();
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
                    if input.sections[section].rooms.is_empty() {
                        if !schedule.has_hard_conflict(input, section, time_slot, &None) {
                            count += 1;
                        }
                    } else {
                        for &RoomWithOptionalPriority { room, .. } in &input.sections[section].rooms {
                            if !schedule.has_hard_conflict(input, section, time_slot, &Some(room)) {
                                count += 1;
                            }
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
                if input.sections[section].rooms.is_empty() {
                    if !schedule.has_hard_conflict(input, section, time_slot, &None) {
                        count += 1;
                        if count == winner {
                            let _undo = PlacementLog::move_section(&mut schedule, input, section, time_slot, &None);
                            break 'time_loop;
                        }
                    }
                } else {
                    for &RoomWithOptionalPriority { room, .. } in &input.sections[section].rooms {
                        if !schedule.has_hard_conflict(input, section, time_slot, &Some(room)) {
                            count += 1;
                            if count == winner {
                                let _undo =
                                    PlacementLog::move_section(&mut schedule, input, section, time_slot, &Some(room));
                                break 'time_loop;
                            }
                        }
                    }
                }
            }
        }

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

    // keep making greedy, single-step changes until no single-step improvements are possible
    loop {
        let mut best_delta = None;
        let mut best_move = None;

        // for each section
        for section in 0..input.sections.len() {
            // try each time slot
            for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
                // try each Some(room), or None if there are no rooms
                let rooms = if input.sections[section].rooms.is_empty() {
                    vec![None]
                } else {
                    input.sections[section]
                        .rooms
                        .iter()
                        .map(|&RoomWithOptionalPriority { room, .. }| Some(room))
                        .collect()
                };
                for room in rooms {
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
        let log_entry = PlacementLog::move_section(schedule, input, section, ts, &room);
        log.push(log_entry);
    }
}

// try a single section move then undo it, and return the score delta it created
pub fn try_one_move(input: &Input, schedule: &mut Schedule, candidate_move: &Move) -> Score {
    let &Move { section, time_slot: Some(ts), room } = candidate_move else {
        panic!("try_one_move called with no time slot");
    };
    let old_score = schedule.score;
    let mut undo = PlacementLog::move_section(schedule, input, section, ts, &room);
    let new_score = schedule.score;
    undo.revert_move(schedule);
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
        if schedule.placements[section].score.local == zero {
            continue;
        }

        // try each time slot
        for &TimeSlotWithOptionalPriority { time_slot, .. } in &input.sections[section].time_slots {
            // try each Some(room), or None if there are no rooms
            let rooms = if input.sections[section].rooms.is_empty() {
                vec![None]
            } else {
                input.sections[section].rooms.iter().map(|&RoomWithOptionalPriority { room, .. }| Some(room)).collect()
            };
            for room in rooms {
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

                candidates.push((schedule.placements[section].score.local.first_nonzero(), candidate));
            }
        }
    }

    // no moves possible?
    if candidates.is_empty() {
        return false;
    }

    // sort by highest priority first
    candidates.sort();

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
    let log_entry = PlacementLog::move_section(schedule, input, section, ts, &room);
    log.push(log_entry);

    true
}
