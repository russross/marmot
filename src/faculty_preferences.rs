use super::error::{Result, err};
use super::input::{Days, Input, Time};
use super::score::{Criterion, FacultyPreferenceKind, MAX_PRIORITY, START_LEVEL_FOR_PREFERENCES};
use super::solver::Schedule;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
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

struct PreferenceTier {
    faculty: usize,
    stated_priority: u8,
    criteria: Vec<usize>,
}

struct TierImpact {
    faculty: usize,
    stated_priority: u8,
    criteria: Vec<usize>,
    total: u64,
    remaining: u64,
}

struct ImpactBucket {
    impacts: Vec<TierImpact>,
}

#[derive(Default)]
struct CountingStats {
    time_assignments: u64,
    room_cache_misses: u64,
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

    let mut impacts = Vec::new();
    let mut stats = CountingStats::default();
    for tiers in tiers_by_faculty.values() {
        let counts = count_preference_prefixes(input, tiers, &mut stats)?;
        let total = counts[0];
        if total == 0 {
            return err(format!(
                "faculty {} has no conflict-free local schedule",
                input.faculty[tiers[0].faculty].name
            ));
        }
        for (tier_index, tier) in tiers.iter().enumerate() {
            impacts.push(TierImpact {
                faculty: tier.faculty,
                stated_priority: tier.stated_priority,
                criteria: tier.criteria.clone(),
                total,
                remaining: counts[tier_index + 1],
            });
        }
    }

    let buckets = bucket_preference_tiers(impacts);
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
        "balanced {} faculty preference tiers into {} priorities in {}ms",
        buckets.iter().map(|bucket| bucket.impacts.len()).sum::<usize>(),
        buckets.len(),
        started.elapsed().as_millis()
    );
    Ok(())
}

fn print_rebalancing_details(input: &Input, buckets: &[ImpactBucket]) {
    println!("Faculty preference priority redistribution:");
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

fn count_preference_prefixes(input: &Input, tiers: &[PreferenceTier], stats: &mut CountingStats) -> Result<Vec<u64>> {
    let faculty = tiers[0].faculty;
    let mut ordered_sections = input.faculty[faculty].sections.clone();
    ordered_sections.sort_unstable_by_key(|&section| input.sections[section].time_slots.len());
    let mut schedule = Schedule::new(input);
    let mut counts = vec![0_u64; tiers.len() + 1];
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
    counts: &mut [u64],
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
        counts[prefix] = counts[prefix]
            .checked_add(room_count)
            .ok_or("faculty schedule count overflow; reduce the local schedule domain")?;
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

fn bucket_preference_tiers(mut impacts: Vec<TierImpact>) -> Vec<ImpactBucket> {
    impacts.sort_by(|a, b| {
        compare_impact(a, b)
            .then_with(|| a.faculty.cmp(&b.faculty))
            .then_with(|| a.stated_priority.cmp(&b.stated_priority))
    });
    let mut buckets: Vec<ImpactBucket> = Vec::new();
    for impact in impacts {
        if buckets.last().is_some_and(|bucket| same_impact(&bucket.impacts[0], &impact)) {
            buckets.last_mut().unwrap().impacts.push(impact);
        } else {
            buckets.push(ImpactBucket { impacts: vec![impact] });
        }
    }

    while buckets.len() > EFFECTIVE_PRIORITY_BUCKETS {
        let mut merge_index = 0;
        let mut smallest_spread = bucket_merge_spread(&buckets, 0);
        for candidate in 1..buckets.len() - 1 {
            let spread = bucket_merge_spread(&buckets, candidate);
            if spread < smallest_spread {
                merge_index = candidate;
                smallest_spread = spread;
            }
        }
        let right = buckets.remove(merge_index + 1);
        buckets[merge_index].impacts.extend(right.impacts);
    }
    buckets
}

fn compare_impact(a: &TierImpact, b: &TierImpact) -> Ordering {
    match (a.remaining, b.remaining) {
        (0, 0) => Ordering::Equal,
        (0, _) => Ordering::Greater,
        (_, 0) => Ordering::Less,
        _ => (a.total as u128 * b.remaining as u128).cmp(&(b.total as u128 * a.remaining as u128)),
    }
}

fn same_impact(a: &TierImpact, b: &TierImpact) -> bool {
    compare_impact(a, b) == Ordering::Equal
}

fn entropy(impact: &TierImpact) -> f64 {
    if impact.remaining == 0 { f64::INFINITY } else { (impact.total as f64).log2() - (impact.remaining as f64).log2() }
}

fn bucket_merge_spread(buckets: &[ImpactBucket], left: usize) -> f64 {
    let lowest = entropy(&buckets[left].impacts[0]);
    let highest = entropy(buckets[left + 1].impacts.last().unwrap());
    highest - lowest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{
        Duration, Faculty, Room, RoomWithOptionalPriority, Section, TimeSlot, TimeSlotWithOptionalPriority,
    };
    use crate::score::{FacultyPreference, FacultyPreferenceKind};

    fn section(name: &str, time_slots: &[usize]) -> Section {
        Section {
            name: name.to_string(),
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

        let counts = count_preference_prefixes(&input, &tiers, &mut stats).unwrap();

        let mut brute_force_counts = vec![0_u64; 5];
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
        let impacts = (0..impact_count)
            .map(|index| TierImpact {
                faculty: index,
                stated_priority: 10,
                criteria: vec![index],
                total: 100,
                remaining: if index + 1 == impact_count { 0 } else { 100 - index as u64 * 3 },
            })
            .collect();

        let buckets = bucket_preference_tiers(impacts);

        assert_eq!(buckets.len(), EFFECTIVE_PRIORITY_BUCKETS);
        assert_eq!(buckets.last().unwrap().impacts.last().unwrap().remaining, 0);
    }
}
