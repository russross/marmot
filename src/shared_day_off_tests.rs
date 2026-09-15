use crate::cnf::Encoding;
use crate::faculty_preferences::{
    FacultyPreferencePriorityPolicy, merge_shared_day_off_preferences, rebalance_faculty_preferences,
};
use crate::input::{
    CreditHours, Days, Duration, Faculty, Input, Room, RoomWithOptionalPriority, Section, Time, TimeSlot,
    TimeSlotWithOptionalPriority,
};
use crate::sat_criteria::SatCriteria;
use crate::sat_encoders::encode_criterion;
use crate::score::{Criterion, FacultyPreference, FacultyPreferenceKind};
use crate::solver::Schedule;
use kissat::Solver;

pub fn owned_input() -> Input {
    let days = Days::parse("MT").unwrap();
    Input {
        term_name: "test".into(),
        rooms: vec![Room { name: "A".into() }, Room { name: "B".into() }],
        time_slots: ["M", "T", "W"]
            .iter()
            .map(|day| TimeSlot {
                name: (*day).into(),
                days: Days::parse(day).unwrap(),
                start_time: Time::new(540),
                duration: Duration::new(50),
            })
            .collect(),
        faculty: vec![
            Faculty { name: "A".into(), sections: vec![0, 1] },
            Faculty { name: "B".into(), sections: vec![2, 3] },
        ],
        sections: (0..4)
            .map(|i| Section {
                name: format!("S{i}"),
                credit_hours: CreditHours::new(3.0),
                rooms: (0..2).map(|room| RoomWithOptionalPriority { room, priority: None }).collect(),
                time_slots: (0..3)
                    .map(|time_slot| TimeSlotWithOptionalPriority { time_slot, priority: None })
                    .collect(),
                faculty: vec![i / 2],
                hard_conflicts: vec![],
                criteria: vec![0, 1],
                neighbors: (0..4).filter(|&j| i != j).collect(),
            })
            .collect(),
        criteria: (0..2)
            .map(|faculty| {
                Criterion::OwnedFacultyPreference(FacultyPreference {
                    faculty,
                    sections: vec![0, 1, 2, 3],
                    stated_priority: 20 + faculty as u8,
                    priority: 20 + faculty as u8,
                    kind: FacultyPreferenceKind::SameDayOffAs { other_faculty: 1 - faculty, days_to_check: days },
                })
            })
            .collect(),
        faculty_preference_priority_policy: FacultyPreferencePriorityPolicy::Stated,
        time_slot_conflicts: vec![vec![true, false, false], vec![false, true, false], vec![false, false, true]],
    }
}

pub fn input() -> Input {
    let mut input = owned_input();
    merge_shared_day_off_preferences(&mut input.criteria).unwrap();
    for section in &mut input.sections {
        section.criteria = vec![0];
    }
    input
}

fn satisfiable(encoding: &Encoding) -> bool {
    let mut solver = Solver::new();
    let vars: Vec<_> = (0..encoding.last_var).map(|_| solver.var()).collect();
    for clause in &encoding.clauses {
        let literals: Vec<_> = clause
            .iter()
            .map(|&literal| {
                let var = vars[literal.unsigned_abs() as usize - 1];
                if literal > 0 { var } else { !var }
            })
            .collect();
        solver.add(&literals);
    }
    solver.sat().is_some()
}

#[test]
fn sat_conversion_rejects_unmerged_shared_day_off_requests() {
    assert!(SatCriteria::from_input(&owned_input()).is_err());
}

#[test]
fn individual_requests_are_optional_and_day_links_are_reused_in_either_order() {
    for individual_owners in 0_u8..4 {
        for individual_first in [true, false] {
            let mut input = input();
            for faculty in 0..2 {
                if individual_owners & (1 << faculty) != 0 {
                    let priority = if individual_first { 10 } else { 25 };
                    input.criteria.push(Criterion::OwnedFacultyPreference(FacultyPreference {
                        faculty,
                        sections: input.faculty[faculty].sections.clone(),
                        stated_priority: priority,
                        priority,
                        kind: FacultyPreferenceKind::DaysOff { days_to_check: Days::parse("MT").unwrap(), desired: 1 },
                    }));
                }
            }
            let criteria = SatCriteria::from_input(&input).unwrap();
            for times in [[0, 2, 0, 2], [0, 2, 1, 2], [0, 1, 0, 2], [0, 1, 0, 1]] {
                let mut schedule = Schedule::new(&input);
                for (section, time) in times.into_iter().enumerate() {
                    schedule.placements[section].time_slot = Some(time);
                }
                let violations: usize = input.criteria.iter().map(|c| c.check(&input, &schedule).len()).sum();
                for allowed in [violations, violations.saturating_sub(1)] {
                    let mut encoding = Encoding::new();
                    for (section, time) in times.into_iter().enumerate() {
                        for option in 0..3 {
                            let var = encoding.new_var();
                            encoding.section_time_vars.insert((section, option), var);
                            encoding.add_clause(vec![if time == option { var } else { -var }]);
                        }
                    }
                    for priority in criteria.priorities() {
                        for criterion in criteria.criteria_at_priority(priority) {
                            encode_criterion(&input, &mut encoding, criterion, &criteria).unwrap();
                        }
                    }
                    let hallpasses: Vec<_> =
                        encoding.hallpasses.values().flat_map(|group| group.iter().copied()).collect();
                    assert_eq!(hallpasses.len(), 1 + individual_owners.count_ones() as usize);
                    assert_eq!(encoding.faculty_day_vars.len(), 4);
                    // Twelve assignment variables, four day variables, one shared hallpass, and optional individual hallpasses.
                    assert_eq!(encoding.last_var as usize, 16 + hallpasses.len());
                    encoding.totalizer_at_most_k(&hallpasses, allowed, None);
                    assert_eq!(satisfiable(&encoding), allowed >= violations);
                }
            }
        }
    }
}

#[test]
fn shared_day_off_sat_matches_scoring_with_one_joint_penalty() {
    for (same_priority, balanced) in [(false, false), (true, false), (false, true)] {
        let mut input = owned_input();
        if same_priority {
            let Criterion::OwnedFacultyPreference(preference) = &mut input.criteria[1] else { unreachable!() };
            preference.priority = 20;
        }
        if balanced {
            input.faculty_preference_priority_policy = FacultyPreferencePriorityPolicy::EntropyBalancedV1;
            rebalance_faculty_preferences(&mut input, false).unwrap();
        }
        merge_shared_day_off_preferences(&mut input.criteria).unwrap();
        let criteria = SatCriteria::from_input(&input).unwrap();
        for pattern in 0..81 {
            let mut schedule = Schedule::new(&input);
            let mut value = pattern;
            let mut masks = [0, 0];
            for section in 0..4 {
                let time = value % 3;
                value /= 3;
                schedule.placements[section].time_slot = Some(time);
                if time < 2 {
                    masks[section / 2] |= 1 << time;
                }
            }
            let satisfied = matches!((masks[0], masks[1]), (1, 1) | (2, 2));
            let penalties: Vec<_> = input.criteria.iter().flat_map(|c| c.check(&input, &schedule)).collect();
            assert_eq!(penalties.len(), usize::from(!satisfied));
            for penalty in &penalties {
                assert_eq!(penalty.faculty(), vec![0, 1]);
                assert_eq!(penalty.get_sections(&input), vec![0, 1, 2, 3]);
                assert_eq!(
                    penalty.get_priority(),
                    if balanced {
                        10
                    } else if same_priority {
                        20
                    } else {
                        21
                    }
                );
            }
            for allowed in 0..=1 {
                let mut encoding = Encoding::new();
                for section in 0..4 {
                    for time in 0..3 {
                        let var = encoding.new_var();
                        encoding.section_time_vars.insert((section, time), var);
                        encoding.add_clause(vec![if schedule.placements[section].time_slot == Some(time) {
                            var
                        } else {
                            -var
                        }]);
                    }
                }
                for priority in criteria.priorities() {
                    for criterion in criteria.criteria_at_priority(priority) {
                        encode_criterion(&input, &mut encoding, criterion, &criteria).unwrap();
                    }
                }
                let hallpasses: Vec<_> = encoding.hallpasses.values().flat_map(|group| group.iter().copied()).collect();
                assert_eq!(hallpasses.len(), 1);
                encoding.totalizer_at_most_k(&hallpasses, allowed, None);
                assert_eq!(satisfiable(&encoding), satisfied || allowed == 1, "pattern {pattern}, budget {allowed}");
            }
        }
    }
}

#[test]
fn shared_day_off_requires_exactly_one_empty_checked_day() {
    let mut input = input();
    let Criterion::SharedDayOffPreference { days_to_check, .. } = &mut input.criteria[0] else { unreachable!() };
    *days_to_check = Days::parse("MTW").unwrap();
    let mut schedule = Schedule::new(&input);
    for (times, satisfied) in [([0, 0, 0, 0], false), ([0, 1, 0, 1], true), ([0, 1, 0, 2], false)] {
        for (section, time) in times.into_iter().enumerate() {
            schedule.placements[section].time_slot = Some(time);
        }
        assert_eq!(input.criteria[0].check(&input, &schedule).is_empty(), satisfied);
    }
    schedule = Schedule::new(&input);
    assert_eq!(input.criteria[0].check(&input, &schedule).len(), 1);
}
