use crate::input::Section;
use crate::score::*;

pub fn compute_neighbors(sections: &mut [Section], criteria: &[Criterion]) {
    for (i, criterion) in criteria.iter().enumerate() {
        let neighbors = criterion.get_culpable_sections();
        for &section in &neighbors {
            sections[section].criteria.push(i);
            sections[section].neighbors.append(&mut neighbors.clone());
        }
    }
    for (i, section) in sections.iter_mut().enumerate() {
        section.neighbors.retain(|&elt| elt != i);
        section.neighbors.sort_unstable();
        section.neighbors.dedup();
    }
}

pub fn expand_bundled_faculty_preferences(criteria: &mut Vec<Criterion>) {
    let mut expanded = Vec::with_capacity(criteria.len());
    for criterion in criteria.drain(..) {
        let Criterion::FacultyPreference {
            faculty,
            sections,
            days_to_check,
            days_off,
            evenly_spread,
            no_room_switch,
            too_many_rooms,
            max_gap_within_cluster,
            distribution_intervals,
        } = criterion
        else {
            expanded.push(criterion);
            continue;
        };

        let mut push = |priority: u8, kind: FacultyPreferenceKind| {
            expanded.push(Criterion::OwnedFacultyPreference(FacultyPreference {
                faculty,
                sections: sections.clone(),
                stated_priority: priority,
                priority,
                kind,
            }));
        };
        if let Some((priority, desired)) = days_off {
            push(priority, FacultyPreferenceKind::DaysOff { days_to_check, desired });
        }
        if let Some(priority) = evenly_spread {
            push(priority, FacultyPreferenceKind::EvenlySpread { days_to_check });
        }
        if let Some(priority) = no_room_switch {
            push(priority, FacultyPreferenceKind::NoRoomSwitch { days_to_check, max_gap: max_gap_within_cluster });
        }
        if let Some((priority, desired_max_rooms)) = too_many_rooms {
            push(priority, FacultyPreferenceKind::TooManyRooms { desired_max_rooms });
        }
        for interval in distribution_intervals {
            let (priority, kind) = match interval {
                DistributionInterval::GapTooLong { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::GapTooLong { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
                DistributionInterval::GapTooShort { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::GapTooShort { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
                DistributionInterval::ClusterTooLong { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::ClusterTooLong { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
                DistributionInterval::ClusterTooShort { priority, duration } => (
                    priority,
                    FacultyPreferenceKind::ClusterTooShort { days_to_check, duration, max_gap: max_gap_within_cluster },
                ),
            };
            push(priority, kind);
        }
    }
    *criteria = expanded;
}
