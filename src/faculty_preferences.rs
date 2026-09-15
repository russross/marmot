use super::error::{Result, err};
use super::input::{Days, Input, Time};
use super::score::{
    Criterion, FacultyPreference, FacultyPreferenceKind, MAX_PRIORITY, START_LEVEL_FOR_PREFERENCES,
    faculty_teaching_days,
};
use super::solver::Schedule;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Instant;

const EFFECTIVE_PRIORITY_BUCKETS: usize = (MAX_PRIORITY - START_LEVEL_FOR_PREFERENCES + 1) as usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacultyPreferencePriorityPolicy {
    Stated,
    EntropyBalancedV1,
}

impl FacultyPreferencePriorityPolicy {
    pub fn database_name(self) -> &'static str {
        match self {
            Self::Stated => "stated",
            Self::EntropyBalancedV1 => "entropy-balanced-v1",
        }
    }
}

#[derive(Clone)]
struct PreferenceTier {
    faculty: usize,
    stated_priority: u8,
    criteria: Vec<usize>,
}

struct TierImpact {
    faculty: usize,
    stated_priority: u8,
    criteria: Vec<usize>,
    effective_preferences: usize,
    total: u128,
    remaining: u128,
}

struct ImpactBucket {
    impacts: Vec<TierImpact>,
}

struct SharedPair {
    faculty: [usize; 2],
    days_to_check: Days,
    join_tiers: [usize; 2],
}

struct SharedRequest {
    faculty: usize,
    criterion: usize,
    days_to_check: Days,
}

#[derive(Clone, Copy)]
struct PrefixSide<'a> {
    tiers: &'a [PreferenceTier],
    prefix: usize,
    counts: &'a [DayHistogram],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DistributionScore {
    squared_deviation: u128,
    maximum_deviation: u128,
}

#[derive(Clone, Copy)]
struct DistributionState {
    score: DistributionScore,
    previous_end: usize,
}

#[derive(Default)]
struct CountingStats {
    time_assignments: u64,
    room_cache_misses: u64,
}

struct DayHistogram {
    counts: [u128; 128],
    total: u128,
}

impl DayHistogram {
    fn new() -> Self {
        Self { counts: [0; 128], total: 0 }
    }

    fn add(&mut self, days: Days, count: u64) -> Result<()> {
        self.total = self.total.checked_add(count as u128).ok_or("faculty schedule count overflow")?;
        self.counts[days.days as usize] =
            self.counts[days.days as usize].checked_add(count as u128).ok_or("faculty schedule count overflow")?;
        Ok(())
    }

    fn matching(&self, days: Days, mask: u8) -> u128 {
        self.counts.iter().enumerate().filter(|(m, _)| *m as u8 & days.days == mask).map(|(_, &n)| n).sum()
    }
}

pub fn rebalance_faculty_preferences(input: &mut Input, show_details: bool) -> Result<()> {
    let started = Instant::now();
    let mut grouped: BTreeMap<(usize, u8), Vec<usize>> = BTreeMap::new();
    for (criterion_index, criterion) in input.criteria.iter().enumerate() {
        if let Criterion::OwnedFacultyPreference(preference) = criterion {
            grouped.entry((preference.faculty, preference.stated_priority)).or_default().push(criterion_index);
        }
    }

    let mut tiers_by_faculty: BTreeMap<usize, Vec<PreferenceTier>> = BTreeMap::new();
    for ((faculty, stated_priority), criteria) in grouped {
        tiers_by_faculty.entry(faculty).or_default().push(PreferenceTier { faculty, stated_priority, criteria });
    }

    let shared_pairs = find_shared_pairs(input, &tiers_by_faculty)?;
    let paired_faculty: BTreeSet<usize> = shared_pairs.iter().flat_map(|pair| pair.faculty).collect();
    let mut impacts = Vec::new();
    let mut stats = CountingStats::default();
    for tiers in tiers_by_faculty.values() {
        if paired_faculty.contains(&tiers[0].faculty) {
            continue;
        }
        let counts = count_preference_impacts(input, tiers, &mut stats)?;
        if counts.iter().any(|&(total, _)| total == 0) {
            return err(format!(
                "faculty {} has no conflict-free local schedule",
                input.faculty[tiers[0].faculty].name
            ));
        }
        for (tier_index, tier) in tiers.iter().enumerate() {
            let (total, remaining) = counts[tier_index];
            let effective_preferences = tier
                .criteria
                .iter()
                .map(|&criterion_index| effective_preference_count(&input.criteria[criterion_index]))
                .sum();
            impacts.push(TierImpact {
                faculty: tier.faculty,
                stated_priority: tier.stated_priority,
                criteria: tier.criteria.clone(),
                effective_preferences,
                total,
                remaining,
            });
        }
    }
    for pair in &shared_pairs {
        add_shared_pair_impacts(input, &tiers_by_faculty, pair, &mut impacts, &mut stats)?;
    }

    let buckets = distribute_preference_tiers(impacts, EFFECTIVE_PRIORITY_BUCKETS);
    for (bucket_index, bucket) in buckets.iter().enumerate() {
        let priority = START_LEVEL_FOR_PREFERENCES + bucket_index as u8;
        for impact in &bucket.impacts {
            for &criterion_index in &impact.criteria {
                let Criterion::OwnedFacultyPreference(preference) = &mut input.criteria[criterion_index] else {
                    unreachable!("preference tier points to a non-faculty criterion");
                };
                preference.priority = priority;
            }
        }
    }

    if show_details {
        print_rebalancing_details(input, &buckets);
    }

    println!(
        "balanced {} effective faculty preferences from {} stated preferences into {} effective priorities in {}ms",
        buckets.iter().flat_map(|bucket| &bucket.impacts).map(|impact| impact.effective_preferences).sum::<usize>(),
        buckets.iter().map(|bucket| bucket.impacts.len()).sum::<usize>(),
        buckets.len(),
        started.elapsed().as_millis()
    );
    Ok(())
}

fn find_shared_pairs(
    input: &Input,
    tiers_by_faculty: &BTreeMap<usize, Vec<PreferenceTier>>,
) -> Result<Vec<SharedPair>> {
    let mut requests: BTreeMap<(usize, usize), Vec<SharedRequest>> = BTreeMap::new();
    let mut faculty_pairs = BTreeMap::new();
    for (criterion, preference) in input.criteria.iter().enumerate().filter_map(|(index, criterion)| {
        let Criterion::OwnedFacultyPreference(preference) = criterion else {
            return None;
        };
        matches!(preference.kind, FacultyPreferenceKind::SameDayOffAs { .. }).then_some((index, preference))
    }) {
        let FacultyPreferenceKind::SameDayOffAs { other_faculty, days_to_check } = preference.kind else {
            unreachable!();
        };
        let key = (preference.faculty.min(other_faculty), preference.faculty.max(other_faculty));
        if faculty_pairs.insert(preference.faculty, key).is_some() {
            return err("a faculty member may participate in only one shared day-off pair");
        }
        requests.entry(key).or_default().push(SharedRequest { faculty: preference.faculty, criterion, days_to_check });
    }

    requests
        .into_iter()
        .map(|((faculty, partner), mut requests)| {
            requests.sort_unstable_by_key(|request| request.faculty);
            if requests.len() != 2
                || requests[0].faculty != faculty
                || requests[1].faculty != partner
                || requests[0].days_to_check.days != requests[1].days_to_check.days
            {
                return err("shared day-off preferences must form one reciprocal pair over the same days");
            }
            let join_tier = |owner: usize, criterion: usize| -> Result<usize> {
                tiers_by_faculty[&owner]
                    .iter()
                    .position(|tier| tier.criteria.contains(&criterion))
                    .ok_or_else(|| "shared day-off preference is missing from its faculty tier".into())
            };
            Ok(SharedPair {
                faculty: [faculty, partner],
                days_to_check: requests[0].days_to_check,
                join_tiers: [join_tier(faculty, requests[0].criterion)?, join_tier(partner, requests[1].criterion)?],
            })
        })
        .collect()
}

fn add_shared_pair_impacts(
    input: &Input,
    tiers_by_faculty: &BTreeMap<usize, Vec<PreferenceTier>>,
    pair: &SharedPair,
    impacts: &mut Vec<TierImpact>,
    stats: &mut CountingStats,
) -> Result<()> {
    let first_tiers = &tiers_by_faculty[&pair.faculty[0]];
    let second_tiers = &tiers_by_faculty[&pair.faculty[1]];
    let first_counts = count_preference_prefixes(input, first_tiers, stats)?;
    let second_counts = count_preference_prefixes(input, second_tiers, stats)?;
    if first_counts[0].total == 0 || second_counts[0].total == 0 {
        let faculty = if first_counts[0].total == 0 { pair.faculty[0] } else { pair.faculty[1] };
        return err(format!("faculty {} has no conflict-free local schedule", input.faculty[faculty].name));
    }

    for (tiers, counts, &join_tier) in
        [(first_tiers, &first_counts, &pair.join_tiers[0]), (second_tiers, &second_counts, &pair.join_tiers[1])]
    {
        for (tier_index, tier) in tiers.iter().enumerate().take(join_tier) {
            impacts.push(make_tier_impact(input, tier, counts[0].total, counts[tier_index + 1].total));
        }
    }

    let joint_counts = count_shared_pair_prefix(
        input,
        PrefixSide { tiers: first_tiers, prefix: pair.join_tiers[0], counts: &first_counts },
        PrefixSide { tiers: second_tiers, prefix: pair.join_tiers[1], counts: &second_counts },
        pair.days_to_check,
    )?;
    let mut joint_criteria = first_tiers[pair.join_tiers[0]].criteria.clone();
    joint_criteria.extend_from_slice(&second_tiers[pair.join_tiers[1]].criteria);
    let joint_effective_preferences =
        joint_criteria.iter().map(|&criterion| effective_preference_count(&input.criteria[criterion])).sum::<usize>()
            - 1;
    impacts.push(TierImpact {
        faculty: pair.faculty[0],
        stated_priority: first_tiers[pair.join_tiers[0]]
            .stated_priority
            .max(second_tiers[pair.join_tiers[1]].stated_priority),
        criteria: joint_criteria,
        effective_preferences: joint_effective_preferences,
        total: joint_counts.0,
        remaining: joint_counts.1,
    });

    for tier_index in pair.join_tiers[0] + 1..first_tiers.len() {
        let (total, remaining) = count_shared_pair_prefix(
            input,
            PrefixSide { tiers: first_tiers, prefix: tier_index, counts: &first_counts },
            PrefixSide { tiers: second_tiers, prefix: pair.join_tiers[1], counts: &second_counts },
            pair.days_to_check,
        )?;
        impacts.push(make_tier_impact(input, &first_tiers[tier_index], total, remaining));
    }
    for tier_index in pair.join_tiers[1] + 1..second_tiers.len() {
        let (total, remaining) = count_shared_pair_prefix(
            input,
            PrefixSide { tiers: first_tiers, prefix: pair.join_tiers[0], counts: &first_counts },
            PrefixSide { tiers: second_tiers, prefix: tier_index, counts: &second_counts },
            pair.days_to_check,
        )?;
        impacts.push(make_tier_impact(input, &second_tiers[tier_index], total, remaining));
    }
    Ok(())
}

fn make_tier_impact(input: &Input, tier: &PreferenceTier, total: u128, remaining: u128) -> TierImpact {
    TierImpact {
        faculty: tier.faculty,
        stated_priority: tier.stated_priority,
        criteria: tier.criteria.clone(),
        effective_preferences: tier
            .criteria
            .iter()
            .map(|&criterion| effective_preference_count(&input.criteria[criterion]))
            .sum(),
        total,
        remaining,
    }
}

fn count_shared_pair_prefix(
    input: &Input,
    first: PrefixSide<'_>,
    second: PrefixSide<'_>,
    days_to_check: Days,
) -> Result<(u128, u128)> {
    let faculty = [first.tiers[0].faculty, second.tiers[0].faculty];
    let overlapping =
        input.faculty[faculty[0]].sections.iter().any(|section| input.faculty[faculty[1]].sections.contains(section));
    if overlapping {
        let tiers: Vec<PreferenceTier> =
            first.tiers[..=first.prefix].iter().chain(&second.tiers[..=second.prefix]).cloned().collect();
        return count_joint_assignments(input, &tiers, &faculty);
    }

    let total =
        first.counts[0].total.checked_mul(second.counts[0].total).ok_or("joint faculty schedule count overflow")?;
    let mut remaining = 0_u128;
    for (mask, &count) in first.counts[first.prefix + 1].counts.iter().enumerate() {
        let mask = mask as u8 & days_to_check.days;
        if mask.count_ones() as usize + 1 != days_to_check.len() {
            continue;
        }
        let matching = second.counts[second.prefix + 1].matching(days_to_check, mask);
        remaining = remaining
            .checked_add(count.checked_mul(matching).ok_or("joint faculty schedule count overflow")?)
            .ok_or("joint faculty schedule count overflow")?;
    }
    Ok((total, remaining))
}

pub fn merge_shared_day_off_preferences(criteria: &mut Vec<Criterion>) -> Result<()> {
    let mut retained = Vec::with_capacity(criteria.len());
    let mut pairs: BTreeMap<(usize, usize, u8), Vec<FacultyPreference>> = BTreeMap::new();
    let mut faculty_pairs = BTreeMap::new();

    for criterion in criteria.iter().cloned() {
        match criterion {
            Criterion::OwnedFacultyPreference(preference) => {
                if let FacultyPreferenceKind::SameDayOffAs { other_faculty, days_to_check } = preference.kind {
                    let faculty = preference.faculty.min(other_faculty);
                    let partner = preference.faculty.max(other_faculty);
                    if faculty_pairs.insert(preference.faculty, (faculty, partner)).is_some() {
                        return err("a faculty member may participate in only one shared day-off pair");
                    }
                    pairs.entry((faculty, partner, days_to_check.days)).or_default().push(preference);
                } else {
                    retained.push(Criterion::OwnedFacultyPreference(preference));
                }
            }
            other => retained.push(other),
        }
    }

    for ((faculty, partner, _), mut preferences) in pairs {
        preferences.sort_unstable_by_key(|preference| preference.faculty);
        if preferences.len() != 2
            || preferences[0].faculty != faculty
            || preferences[1].faculty != partner
            || !matches!(
                preferences[0].kind,
                FacultyPreferenceKind::SameDayOffAs { other_faculty, .. } if other_faculty == partner
            )
            || !matches!(
                preferences[1].kind,
                FacultyPreferenceKind::SameDayOffAs { other_faculty, .. } if other_faculty == faculty
            )
        {
            return err("shared day-off preferences must form one reciprocal pair");
        }
        let FacultyPreferenceKind::SameDayOffAs { days_to_check, .. } = preferences[0].kind else {
            unreachable!();
        };
        let mut sections =
            preferences.iter().flat_map(|preference| preference.sections.iter().copied()).collect::<Vec<_>>();
        sections.sort_unstable();
        sections.dedup();
        retained.push(Criterion::SharedDayOffPreference {
            faculty: [faculty, partner],
            sections,
            days_to_check,
            stated_priorities: [preferences[0].stated_priority, preferences[1].stated_priority],
            priority: preferences[0].priority.max(preferences[1].priority),
        });
    }

    *criteria = retained;
    Ok(())
}

fn print_rebalancing_details(input: &Input, buckets: &[ImpactBucket]) {
    println!("Faculty preference priority redistribution:");
    let mut printed_shared_pairs = BTreeSet::new();
    for (bucket_index, bucket) in buckets.iter().enumerate() {
        let priority = START_LEVEL_FOR_PREFERENCES + bucket_index as u8;
        println!("  priority {priority}:");
        for impact in &bucket.impacts {
            let entropy =
                if impact.remaining == 0 { "infinite".to_string() } else { format!("{:.6} bits", entropy(impact)) };
            for &criterion_index in &impact.criteria {
                let Criterion::OwnedFacultyPreference(preference) = &input.criteria[criterion_index] else {
                    unreachable!("preference tier points to a non-faculty criterion");
                };
                if let FacultyPreferenceKind::SameDayOffAs { other_faculty, .. } = preference.kind {
                    let pair = (preference.faculty.min(other_faculty), preference.faculty.max(other_faculty));
                    if !printed_shared_pairs.insert(pair) {
                        continue;
                    }
                    let stated_priorities: Vec<u8> = impact
                        .criteria
                        .iter()
                        .filter_map(|&index| {
                            let Criterion::OwnedFacultyPreference(candidate) = &input.criteria[index] else {
                                return None;
                            };
                            matches!(
                                candidate.kind,
                                FacultyPreferenceKind::SameDayOffAs { other_faculty, .. }
                                    if (candidate.faculty.min(other_faculty), candidate.faculty.max(other_faculty)) == pair
                            )
                            .then_some(candidate.stated_priority)
                        })
                        .collect();
                    println!(
                        "    stated {:2} and {:2}, entropy {:>13}: {} and {} want one matching day off",
                        stated_priorities[0],
                        stated_priorities[1],
                        entropy,
                        input.faculty[pair.0].name,
                        input.faculty[pair.1].name,
                    );
                    continue;
                }
                println!(
                    "    stated {:2}, entropy {:>13}: {}",
                    impact.stated_priority,
                    entropy,
                    preference.description(input)
                );
            }
        }
    }
}

fn count_preference_impacts(
    input: &Input,
    tiers: &[PreferenceTier],
    stats: &mut CountingStats,
) -> Result<Vec<(u128, u128)>> {
    let local = count_preference_prefixes(input, tiers, stats)?;
    Ok((0..tiers.len()).map(|tier| (local[0].total, local[tier + 1].total)).collect())
}

// Shared sections have one placement even when several local domains contain them.
struct JointCounter<'a> {
    input: &'a Input,
    tiers: &'a [PreferenceTier],
    members: &'a [usize],
    sections: Vec<usize>,
    schedule: Schedule,
    total: u128,
    remaining: u128,
}

fn count_joint_assignments(input: &Input, tiers: &[PreferenceTier], members: &[usize]) -> Result<(u128, u128)> {
    let mut sections: Vec<usize> = members.iter().flat_map(|&f| input.faculty[f].sections.iter().copied()).collect();
    sections.sort_unstable();
    sections.dedup();
    sections.sort_unstable_by_key(|&s| input.sections[s].time_slots.len());
    let mut counter =
        JointCounter { input, tiers, members, sections, schedule: Schedule::new(input), total: 0, remaining: 0 };
    counter.times(0)?;
    Ok((counter.total, counter.remaining))
}

impl JointCounter<'_> {
    fn times(&mut self, index: usize) -> Result<()> {
        if index == self.sections.len() {
            return self.rooms(0);
        }
        let section = self.sections[index];
        for option in &self.input.sections[section].time_slots {
            let conflict = self.sections[..index].iter().any(|&other| {
                self.members.iter().any(|&f| {
                    self.input.faculty[f].sections.contains(&section) && self.input.faculty[f].sections.contains(&other)
                }) && self.schedule.placements[other]
                    .time_slot
                    .is_some_and(|t| self.input.time_slot_conflicts[option.time_slot][t])
            });
            if conflict {
                continue;
            }
            self.schedule.placements[section].time_slot = Some(option.time_slot);
            self.times(index + 1)?;
        }
        self.schedule.placements[section].time_slot = None;
        Ok(())
    }

    fn rooms(&mut self, index: usize) -> Result<()> {
        if index == self.sections.len() {
            self.total = self.total.checked_add(1).ok_or("joint faculty schedule count overflow")?;
            if self.tiers.iter().all(|tier| {
                tier.criteria.iter().all(|&c| self.input.criteria[c].check(self.input, &self.schedule).is_empty())
            }) {
                self.remaining = self.remaining.checked_add(1).ok_or("joint faculty schedule count overflow")?;
            }
            return Ok(());
        }
        let section = self.sections[index];
        if self.input.sections[section].rooms.is_empty() {
            return self.rooms(index + 1);
        }
        for option in &self.input.sections[section].rooms {
            self.schedule.placements[section].room = Some(option.room);
            self.rooms(index + 1)?;
        }
        self.schedule.placements[section].room = None;
        Ok(())
    }
}

fn count_preference_prefixes(
    input: &Input,
    tiers: &[PreferenceTier],
    stats: &mut CountingStats,
) -> Result<Vec<DayHistogram>> {
    let faculty = tiers[0].faculty;
    let mut ordered_sections = input.faculty[faculty].sections.clone();
    ordered_sections.sort_unstable_by_key(|&section| input.sections[section].time_slots.len());
    let mut schedule = Schedule::new(input);
    let mut counts: Vec<DayHistogram> = (0..=tiers.len()).map(|_| DayHistogram::new()).collect();
    let mut room_cache: HashMap<(usize, Vec<usize>), u64> = HashMap::new();

    enumerate_time_assignments(input, tiers, &ordered_sections, 0, &mut schedule, &mut counts, &mut room_cache, stats)?;
    Ok(counts)
}

#[allow(clippy::too_many_arguments)]
fn enumerate_time_assignments(
    input: &Input,
    tiers: &[PreferenceTier],
    ordered_sections: &[usize],
    section_index: usize,
    schedule: &mut Schedule,
    counts: &mut [DayHistogram],
    room_cache: &mut HashMap<(usize, Vec<usize>), u64>,
    stats: &mut CountingStats,
) -> Result<()> {
    if section_index < ordered_sections.len() {
        let section = ordered_sections[section_index];
        for option in &input.sections[section].time_slots {
            let time_slot = option.time_slot;
            let overlaps = ordered_sections[..section_index].iter().any(|&other_section| {
                schedule.placements[other_section]
                    .time_slot
                    .is_some_and(|other_time| input.time_slot_conflicts[time_slot][other_time])
            });
            if overlaps {
                continue;
            }
            schedule.placements[section].time_slot = Some(time_slot);
            enumerate_time_assignments(
                input,
                tiers,
                ordered_sections,
                section_index + 1,
                schedule,
                counts,
                room_cache,
                stats,
            )?;
            schedule.placements[section].time_slot = None;
        }
        return Ok(());
    }

    stats.time_assignments = stats.time_assignments.checked_add(1).ok_or("faculty time-assignment count overflow")?;
    for prefix in 0..=tiers.len() {
        if prefix > 0 && !tier_time_preferences_satisfied(input, &tiers[prefix - 1], schedule) {
            break;
        }
        let room_count = count_room_assignments(input, tiers, prefix, schedule, room_cache, stats)?;
        if room_count == 0 {
            break;
        }
        counts[prefix].add(faculty_teaching_days(input, schedule, tiers[0].faculty, Days { days: 127 }), room_count)?;
    }
    Ok(())
}

fn tier_time_preferences_satisfied(input: &Input, tier: &PreferenceTier, schedule: &Schedule) -> bool {
    tier.criteria.iter().all(|&criterion_index| {
        let Criterion::OwnedFacultyPreference(preference) = &input.criteria[criterion_index] else {
            unreachable!("preference tier points to a non-faculty criterion");
        };
        match preference.kind {
            FacultyPreferenceKind::AvoidRooms { .. }
            | FacultyPreferenceKind::SameDayOffAs { .. }
            | FacultyPreferenceKind::NoRoomSwitch { .. }
            | FacultyPreferenceKind::TooManyRooms { .. } => true,
            _ => preference.check(input, schedule).is_empty(),
        }
    })
}

fn count_room_assignments(
    input: &Input,
    tiers: &[PreferenceTier],
    prefix: usize,
    schedule: &Schedule,
    cache: &mut HashMap<(usize, Vec<usize>), u64>,
    stats: &mut CountingStats,
) -> Result<u64> {
    let faculty = tiers[0].faculty;
    let sections = &input.faculty[faculty].sections;
    let mut allowed_rooms: Vec<Vec<usize>> = sections
        .iter()
        .map(|&section| input.sections[section].rooms.iter().map(|option| option.room).collect())
        .collect();
    let mut parent: Vec<usize> = (0..sections.len()).collect();
    let mut desired_max_rooms = None;

    for tier in &tiers[..prefix] {
        for &criterion_index in &tier.criteria {
            let Criterion::OwnedFacultyPreference(preference) = &input.criteria[criterion_index] else {
                unreachable!("preference tier points to a non-faculty criterion");
            };
            match &preference.kind {
                FacultyPreferenceKind::AvoidRooms { section, rooms } => {
                    let local = sections.iter().position(|candidate| candidate == section).unwrap();
                    allowed_rooms[local].retain(|room| !rooms.contains(room));
                }
                FacultyPreferenceKind::NoRoomSwitch { days_to_check, max_gap } => {
                    add_adjacent_room_equalities(
                        input,
                        sections,
                        *days_to_check,
                        max_gap.minutes,
                        schedule,
                        &mut parent,
                    );
                }
                FacultyPreferenceKind::TooManyRooms { desired_max_rooms: desired } => {
                    desired_max_rooms =
                        Some(desired_max_rooms.map_or(*desired, |current: usize| current.min(*desired)));
                }
                _ => {}
            }
        }
    }

    let partition = canonical_partition(&mut parent);
    let key = (prefix, partition.clone());
    if let Some(&count) = cache.get(&key) {
        return Ok(count);
    }
    stats.room_cache_misses += 1;

    let mut component_rooms: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (local, rooms) in allowed_rooms.into_iter().enumerate() {
        if rooms.is_empty() && input.sections[sections[local]].rooms.is_empty() {
            continue;
        }
        let root = partition[local];
        component_rooms.entry(root).and_modify(|common| common.retain(|room| rooms.contains(room))).or_insert(rooms);
    }
    let component_rooms: Vec<Vec<usize>> = component_rooms.into_values().collect();
    let count = if component_rooms.iter().any(Vec::is_empty) {
        0
    } else if let Some(max_rooms) = desired_max_rooms {
        count_with_distinct_room_limit(&component_rooms, 0, &mut Vec::new(), max_rooms)?
    } else {
        component_rooms.iter().try_fold(1_u64, |count, rooms| -> Result<u64> {
            count.checked_mul(rooms.len() as u64).ok_or_else(|| "faculty room-assignment count overflow".into())
        })?
    };
    cache.insert(key, count);
    Ok(count)
}

fn add_adjacent_room_equalities(
    input: &Input,
    sections: &[usize],
    days_to_check: Days,
    max_gap_minutes: u16,
    schedule: &Schedule,
    parent: &mut [usize],
) {
    for day in days_to_check {
        let mut day_sections: Vec<(Time, Time, usize)> = sections
            .iter()
            .enumerate()
            .filter_map(|(local, &section)| {
                let time_slot = schedule.placements[section].time_slot?;
                let time = &input.time_slots[time_slot];
                time.days.contains(day).then_some((time.start_time, time.start_time + time.duration, local))
            })
            .collect();
        day_sections.sort_unstable_by_key(|entry| entry.0);
        for pair in day_sections.windows(2) {
            let (_, first_end, first) = pair[0];
            let (second_start, _, second) = pair[1];
            if (second_start - first_end).minutes > max_gap_minutes {
                continue;
            }
            let first_section = sections[first];
            let second_section = sections[second];
            let has_common_preferred_room = input.sections[first_section].rooms.iter().any(|first_room| {
                first_room.priority.is_none()
                    && input.sections[second_section]
                        .rooms
                        .iter()
                        .any(|second_room| second_room.priority.is_none() && second_room.room == first_room.room)
            });
            if has_common_preferred_room {
                union(parent, first, second);
            }
        }
    }
}

fn find(parent: &mut [usize], item: usize) -> usize {
    if parent[item] != item {
        parent[item] = find(parent, parent[item]);
    }
    parent[item]
}

fn union(parent: &mut [usize], a: usize, b: usize) {
    let a = find(parent, a);
    let b = find(parent, b);
    if a != b {
        parent[b] = a;
    }
}

fn canonical_partition(parent: &mut [usize]) -> Vec<usize> {
    let roots: Vec<usize> = (0..parent.len()).map(|item| find(parent, item)).collect();
    let mut labels = HashMap::new();
    roots
        .into_iter()
        .map(|root| {
            let next = labels.len();
            *labels.entry(root).or_insert(next)
        })
        .collect()
}

fn count_with_distinct_room_limit(
    components: &[Vec<usize>],
    component_index: usize,
    used_rooms: &mut Vec<usize>,
    max_rooms: usize,
) -> Result<u64> {
    if component_index == components.len() {
        return Ok(1);
    }
    let mut count = 0_u64;
    for &room in &components[component_index] {
        let is_new = !used_rooms.contains(&room);
        if is_new && used_rooms.len() == max_rooms {
            continue;
        }
        if is_new {
            used_rooms.push(room);
        }
        count = count
            .checked_add(count_with_distinct_room_limit(components, component_index + 1, used_rooms, max_rooms)?)
            .ok_or("faculty room-assignment count overflow")?;
        if is_new {
            used_rooms.pop();
        }
    }
    Ok(count)
}

fn effective_preference_count(criterion: &Criterion) -> usize {
    let Criterion::OwnedFacultyPreference(preference) = criterion else {
        unreachable!("faculty preference tier contains a non-faculty criterion");
    };
    match &preference.kind {
        FacultyPreferenceKind::AvoidRooms { rooms, .. } => rooms.len(),
        FacultyPreferenceKind::AvoidTimeSlots { time_slots, .. } => time_slots.len(),
        FacultyPreferenceKind::DaysOff { .. }
        | FacultyPreferenceKind::SameDayOffAs { .. }
        | FacultyPreferenceKind::EvenlySpread { .. }
        | FacultyPreferenceKind::NoRoomSwitch { .. }
        | FacultyPreferenceKind::TooManyRooms { .. }
        | FacultyPreferenceKind::GapTooLong { .. }
        | FacultyPreferenceKind::GapTooShort { .. }
        | FacultyPreferenceKind::ClusterTooLong { .. }
        | FacultyPreferenceKind::ClusterTooShort { .. }
        | FacultyPreferenceKind::TimePatternMatch { .. } => 1,
    }
}

fn distribute_preference_tiers(mut impacts: Vec<TierImpact>, available_levels: usize) -> Vec<ImpactBucket> {
    assert!(available_levels > 0, "faculty preferences require at least one effective priority level");
    impacts.sort_by(|a, b| {
        compare_impact(a, b)
            .then_with(|| a.faculty.cmp(&b.faculty))
            .then_with(|| a.stated_priority.cmp(&b.stated_priority))
    });
    if impacts.is_empty() {
        return Vec::new();
    }

    let level_count = impacts.len().min(available_levels);
    let mut prefix_weights = Vec::with_capacity(impacts.len() + 1);
    prefix_weights.push(0_u64);
    for impact in &impacts {
        let next = prefix_weights.last().unwrap() + impact.effective_preferences as u64;
        prefix_weights.push(next);
    }
    let total_weight = *prefix_weights.last().unwrap();

    let mut states = vec![vec![None; impacts.len() + 1]; level_count + 1];
    states[0][0] = Some(DistributionState {
        score: DistributionScore { squared_deviation: 0, maximum_deviation: 0 },
        previous_end: 0,
    });

    for level in 1..=level_count {
        for end in level..=impacts.len() {
            for previous_end in level - 1..end {
                let Some(previous) = states[level - 1][previous_end] else {
                    continue;
                };
                let weight = prefix_weights[end] - prefix_weights[previous_end];
                let scaled_weight = level_count as u128 * weight as u128;
                let deviation = scaled_weight.abs_diff(total_weight as u128);
                let score = DistributionScore {
                    squared_deviation: previous.score.squared_deviation + deviation * deviation,
                    maximum_deviation: previous.score.maximum_deviation.max(deviation),
                };
                let candidate = DistributionState { score, previous_end };
                if states[level][end].is_none_or(|current| {
                    (candidate.score, candidate.previous_end) < (current.score, current.previous_end)
                }) {
                    states[level][end] = Some(candidate);
                }
            }
        }
    }

    let mut boundaries = vec![impacts.len()];
    let mut end = impacts.len();
    for level in (1..=level_count).rev() {
        let state = states[level][end].expect("preference distribution has no complete partition");
        end = state.previous_end;
        boundaries.push(end);
    }
    boundaries.reverse();

    let mut remaining = impacts.into_iter();
    let buckets = boundaries
        .windows(2)
        .map(|boundary| ImpactBucket { impacts: remaining.by_ref().take(boundary[1] - boundary[0]).collect() })
        .collect();
    debug_assert!(remaining.next().is_none());
    buckets
}

fn compare_impact(a: &TierImpact, b: &TierImpact) -> Ordering {
    match (a.remaining, b.remaining) {
        (0, 0) => Ordering::Equal,
        (0, _) => Ordering::Greater,
        (_, 0) => Ordering::Less,
        _ => compare_ratios(a.total, a.remaining, b.total, b.remaining),
    }
}

fn compare_ratios(mut a: u128, mut b: u128, mut c: u128, mut d: u128) -> Ordering {
    let mut reversed = false;
    loop {
        let comparison = (a / b).cmp(&(c / d));
        if comparison != Ordering::Equal {
            return if reversed { comparison.reverse() } else { comparison };
        }
        let (r, s) = (a % b, c % d);
        if r == 0 || s == 0 {
            let comparison = r.cmp(&s);
            return if reversed { comparison.reverse() } else { comparison };
        }
        (a, b, c, d) = (b, r, d, s);
        reversed = !reversed;
    }
}

fn entropy(impact: &TierImpact) -> f64 {
    if impact.remaining == 0 { f64::INFINITY } else { (impact.total as f64).log2() - (impact.remaining as f64).log2() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{
        CreditHours, Duration, Faculty, Room, RoomWithOptionalPriority, Section, TimeSlot, TimeSlotWithOptionalPriority,
    };
    use crate::score::{FacultyPreference, FacultyPreferenceKind};
    use crate::shared_day_off_tests::owned_input as shared_input;

    fn section(name: &str, time_slots: &[usize]) -> Section {
        Section {
            name: name.to_string(),
            credit_hours: CreditHours::new(3.0),
            rooms: vec![
                RoomWithOptionalPriority { room: 0, priority: None },
                RoomWithOptionalPriority { room: 1, priority: None },
            ],
            time_slots: time_slots
                .iter()
                .map(|&time_slot| TimeSlotWithOptionalPriority { time_slot, priority: None })
                .collect(),
            faculty: vec![0],
            hard_conflicts: vec![],
            criteria: vec![],
            neighbors: vec![],
        }
    }

    fn preference(priority: u8, kind: FacultyPreferenceKind) -> Criterion {
        Criterion::OwnedFacultyPreference(FacultyPreference {
            faculty: 0,
            sections: vec![0, 1],
            stated_priority: priority,
            priority,
            kind,
        })
    }

    fn impact(faculty: usize, effective_preferences: usize, remaining: u128) -> TierImpact {
        TierImpact {
            faculty,
            stated_priority: 10,
            criteria: (0..effective_preferences).collect(),
            effective_preferences,
            total: 100,
            remaining,
        }
    }

    fn bucket_weights(buckets: &[ImpactBucket]) -> Vec<usize> {
        buckets.iter().map(|bucket| bucket.impacts.iter().map(|impact| impact.effective_preferences).sum()).collect()
    }

    #[test]
    fn shared_pair_prefixes_inherit_both_pre_join_histories() {
        let mut input = shared_input();
        for criterion in &mut input.criteria {
            let Criterion::OwnedFacultyPreference(preference) = criterion else { unreachable!() };
            preference.stated_priority = 12;
            preference.priority = 12;
        }
        input.criteria.extend([
            preference(10, FacultyPreferenceKind::AvoidRooms { section: 0, rooms: vec![0] }),
            preference(14, FacultyPreferenceKind::AvoidRooms { section: 1, rooms: vec![1] }),
            Criterion::OwnedFacultyPreference(FacultyPreference {
                faculty: 1,
                sections: vec![2, 3],
                stated_priority: 10,
                priority: 10,
                kind: FacultyPreferenceKind::AvoidRooms { section: 2, rooms: vec![0] },
            }),
            Criterion::OwnedFacultyPreference(FacultyPreference {
                faculty: 1,
                sections: vec![2, 3],
                stated_priority: 14,
                priority: 14,
                kind: FacultyPreferenceKind::AvoidRooms { section: 3, rooms: vec![1] },
            }),
        ]);
        let first_tiers = vec![
            PreferenceTier { faculty: 0, stated_priority: 10, criteria: vec![2] },
            PreferenceTier { faculty: 0, stated_priority: 12, criteria: vec![0] },
            PreferenceTier { faculty: 0, stated_priority: 14, criteria: vec![3] },
        ];
        let second_tiers = vec![
            PreferenceTier { faculty: 1, stated_priority: 10, criteria: vec![4] },
            PreferenceTier { faculty: 1, stated_priority: 12, criteria: vec![1] },
            PreferenceTier { faculty: 1, stated_priority: 14, criteria: vec![5] },
        ];
        let mut stats = CountingStats::default();
        let first_counts = count_preference_prefixes(&input, &first_tiers, &mut stats).unwrap();
        let second_counts = count_preference_prefixes(&input, &second_tiers, &mut stats).unwrap();
        let side = |tiers, prefix, counts| PrefixSide { tiers, prefix, counts };

        let join = count_shared_pair_prefix(
            &input,
            side(&first_tiers, 1, &first_counts),
            side(&second_tiers, 1, &second_counts),
            Days::parse("MT").unwrap(),
        )
        .unwrap();
        let join_tiers: Vec<_> = first_tiers[..=1].iter().chain(&second_tiers[..=1]).cloned().collect();
        assert_eq!(join, count_joint_assignments(&input, &join_tiers, &[0, 1]).unwrap());

        let first_after_join = count_shared_pair_prefix(
            &input,
            side(&first_tiers, 2, &first_counts),
            side(&second_tiers, 1, &second_counts),
            Days::parse("MT").unwrap(),
        )
        .unwrap();
        let after_tiers: Vec<_> = first_tiers.iter().chain(&second_tiers[..=1]).cloned().collect();
        assert_eq!(first_after_join, count_joint_assignments(&input, &after_tiers, &[0, 1]).unwrap());
    }

    #[test]
    fn shared_sections_are_counted_once() {
        let mut input = shared_input();
        input.faculty[0].sections = vec![0];
        input.faculty[1].sections = vec![0];
        let first_tiers = vec![PreferenceTier { faculty: 0, stated_priority: 20, criteria: vec![0] }];
        let second_tiers = vec![PreferenceTier { faculty: 1, stated_priority: 21, criteria: vec![1] }];
        let mut stats = CountingStats::default();
        let first_counts = count_preference_prefixes(&input, &first_tiers, &mut stats).unwrap();
        let second_counts = count_preference_prefixes(&input, &second_tiers, &mut stats).unwrap();
        let counts = count_shared_pair_prefix(
            &input,
            PrefixSide { tiers: &first_tiers, prefix: 0, counts: &first_counts },
            PrefixSide { tiers: &second_tiers, prefix: 0, counts: &second_counts },
            Days::parse("MT").unwrap(),
        )
        .unwrap();
        // Three times and two rooms for the one shared section, four with one MT day off.
        assert_eq!(counts, (6, 4));
    }

    #[test]
    fn merging_rejects_an_incomplete_pair_without_changing_the_criteria() {
        let mut input = shared_input();
        input.criteria.pop();
        let original = input.criteria[0].debug(&input);

        assert!(merge_shared_day_off_preferences(&mut input.criteria).is_err());
        assert_eq!(input.criteria.len(), 1);
        assert_eq!(input.criteria[0].debug(&input), original);
    }

    #[test]
    fn merging_rejects_multiple_partners_for_one_faculty() {
        let mut input = shared_input();
        input.faculty.push(Faculty { name: "C".into(), sections: vec![4, 5] });
        input.sections.extend([section("C1", &[0, 1, 2]), section("C2", &[0, 1, 2])]);
        input.criteria.push(preference(
            22,
            FacultyPreferenceKind::SameDayOffAs { other_faculty: 2, days_to_check: Days::parse("MT").unwrap() },
        ));
        input.criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
            faculty: 2,
            sections: vec![0, 1, 4, 5],
            stated_priority: 22,
            priority: 22,
            kind: FacultyPreferenceKind::SameDayOffAs { other_faculty: 0, days_to_check: Days::parse("MT").unwrap() },
        }));

        assert!(merge_shared_day_off_preferences(&mut input.criteria).is_err());
    }

    #[test]
    fn rational_impact_comparison_handles_full_width_counts() {
        assert_eq!(compare_ratios(u128::MAX, u128::MAX - 1, u128::MAX - 1, u128::MAX - 2), Ordering::Less);
        assert_eq!(compare_ratios(u128::MAX, 3, u128::MAX / 3, 1), Ordering::Equal);
        for a in 0..20 {
            for b in 1..20 {
                for c in 0..20 {
                    for d in 1..20 {
                        assert_eq!(compare_ratios(a, b, c, d), (a * d).cmp(&(c * b)));
                    }
                }
            }
        }
    }

    #[test]
    fn weighted_room_count_matches_complete_schedule_count() {
        let monday = Days::parse("M").unwrap();
        let mut input = Input {
            term_name: "test".to_string(),
            rooms: vec![Room { name: "A".to_string() }, Room { name: "B".to_string() }],
            time_slots: (0..3)
                .map(|hour| TimeSlot {
                    name: format!("T{hour}"),
                    days: monday,
                    start_time: Time::new(9 * 60 + hour * 60),
                    duration: Duration::new(60),
                })
                .collect(),
            faculty: vec![Faculty { name: "Faculty".to_string(), sections: vec![0, 1] }],
            sections: vec![section("A", &[0, 1]), section("B", &[1, 2])],
            criteria: vec![
                preference(10, FacultyPreferenceKind::AvoidTimeSlots { section: 0, time_slots: vec![1] }),
                preference(
                    11,
                    FacultyPreferenceKind::NoRoomSwitch { days_to_check: monday, max_gap: Duration::new(0) },
                ),
                preference(12, FacultyPreferenceKind::TooManyRooms { desired_max_rooms: 1 }),
                preference(13, FacultyPreferenceKind::AvoidRooms { section: 0, rooms: vec![0] }),
            ],
            faculty_preference_priority_policy: FacultyPreferencePriorityPolicy::EntropyBalancedV1,
            time_slot_conflicts: vec![vec![true, false, false], vec![false, true, false], vec![false, false, true]],
        };
        let tiers: Vec<PreferenceTier> = (0..4)
            .map(|criterion| PreferenceTier {
                faculty: 0,
                stated_priority: 10 + criterion as u8,
                criteria: vec![criterion],
            })
            .collect();
        let mut stats = CountingStats::default();

        let counts: Vec<u128> = count_preference_prefixes(&input, &tiers, &mut stats)
            .unwrap()
            .iter()
            .map(|histogram| histogram.total)
            .collect();

        let mut brute_force_counts = vec![0_u128; 5];
        let mut schedule = Schedule::new(&input);
        for &first_time in &[0, 1] {
            for &second_time in &[1, 2] {
                if input.time_slot_conflicts[first_time][second_time] {
                    continue;
                }
                schedule.placements[0].time_slot = Some(first_time);
                schedule.placements[1].time_slot = Some(second_time);
                for first_room in 0..2 {
                    for second_room in 0..2 {
                        schedule.placements[0].room = Some(first_room);
                        schedule.placements[1].room = Some(second_room);
                        brute_force_counts[0] += 1;
                        for (criterion, prefix_count) in
                            input.criteria.iter().zip(brute_force_counts.iter_mut().skip(1))
                        {
                            if criterion.check(&input, &schedule).is_empty() {
                                *prefix_count += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(counts, vec![12, 8, 6, 4, 2]);
        assert_eq!(counts, brute_force_counts);
        assert_eq!(stats.time_assignments, 3);

        rebalance_faculty_preferences(&mut input, false).unwrap();
        let effective_priorities: Vec<u8> = input
            .criteria
            .iter()
            .map(|criterion| match criterion {
                Criterion::OwnedFacultyPreference(preference) => preference.priority,
                _ => unreachable!(),
            })
            .collect();
        assert!(effective_priorities.windows(2).all(|pair| pair[0] <= pair[1]));
    }

    #[test]
    fn bucketing_keeps_infinite_impact_last_and_caps_levels() {
        let impact_count = EFFECTIVE_PRIORITY_BUCKETS + 4;
        let finite_total = impact_count as u128 * 3;
        let impacts = (0..impact_count)
            .map(|index| TierImpact {
                faculty: index,
                stated_priority: 10,
                criteria: vec![index],
                effective_preferences: 1,
                total: finite_total,
                remaining: if index + 1 == impact_count { 0 } else { finite_total - index as u128 * 3 },
            })
            .collect();

        let buckets = distribute_preference_tiers(impacts, EFFECTIVE_PRIORITY_BUCKETS);

        assert_eq!(buckets.len(), EFFECTIVE_PRIORITY_BUCKETS);
        assert_eq!(buckets.last().unwrap().impacts.last().unwrap().remaining, 0);
    }

    #[test]
    fn dynamic_programming_minimizes_effective_preference_variance() {
        let impacts = [8, 7, 6, 5, 4]
            .into_iter()
            .enumerate()
            .map(|(index, weight)| impact(index, weight, 95 - index as u128 * 5))
            .collect();

        let buckets = distribute_preference_tiers(impacts, 3);

        assert_eq!(bucket_weights(&buckets), vec![8, 13, 9]);
        assert_eq!(buckets[1].impacts.len(), 2);
    }

    #[test]
    fn sparse_effective_preferences_get_distinct_high_priority_levels() {
        let impacts = vec![impact(0, 1, 90), impact(1, 1, 80), impact(2, 1, 70)];

        let buckets = distribute_preference_tiers(impacts, EFFECTIVE_PRIORITY_BUCKETS);
        let assigned: Vec<(usize, u8)> = buckets
            .iter()
            .enumerate()
            .flat_map(|(bucket_index, bucket)| {
                bucket
                    .impacts
                    .iter()
                    .map(move |impact| (impact.faculty, START_LEVEL_FOR_PREFERENCES + bucket_index as u8))
            })
            .collect();

        assert_eq!(assigned, vec![(0, 10), (1, 11), (2, 12)]);
    }
}
