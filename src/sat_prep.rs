use crate::cnf::Encoding;
use crate::error::Result;
use crate::input::Input;
use crate::sat_criteria::{SatCriteria, SatCriterion};
use crate::sat_encoders::encode_criterion;
use crate::sat_solver::{create_basic_variables, encode_basic_constraints, encode_room_conflicts};
use crate::score::Score;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

// Only priority-level hallpass counters participate in the ordering experiment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TotalizerOrder {
    Current,
    Shuffle,
    Cluster,
}

impl FromStr for TotalizerOrder {
    type Err = &'static str;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "current" => Ok(Self::Current),
            "shuffle" => Ok(Self::Shuffle),
            "cluster" => Ok(Self::Cluster),
            _ => Err("expected current, shuffle, or cluster"),
        }
    }
}

impl fmt::Display for TotalizerOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Current => "current",
            Self::Shuffle => "shuffle",
            Self::Cluster => "cluster",
        })
    }
}

// Prefix boundaries exclude all clauses and variables belonging to later criteria.
struct PreparedLevel {
    clause_end: usize,
    last_var: i32,
    hallpasses: Vec<i32>,
}

pub(super) struct PreparedCnf {
    template: Encoding,
    levels: Vec<PreparedLevel>,
}

impl PreparedCnf {
    pub fn new(input: &Input, criteria: &SatCriteria, order: TotalizerOrder, seed: u64) -> Result<Self> {
        let mut template = Encoding::new();
        create_basic_variables(input, &mut template)?;
        encode_basic_constraints(input, &mut template)?;
        encode_room_conflicts(input, &mut template)?;

        // Encode once in priority order so cached faculty-day links belong to
        // the first prefix that needs them, including shared faculty requests.
        let mut levels = Vec::new();
        for priority in 0..=criteria.max_priority() {
            let constraints = criteria.criteria_at_priority(priority);
            let mut groups = Vec::new();
            for criterion in constraints {
                let start = template.hallpass_order.len();
                encode_criterion(input, &mut template, criterion, criteria)?;
                groups.push(template.hallpass_order[start..].to_vec());
            }

            // A criterion's multiple hallpasses stay adjacent under clustering.
            // Shuffling uses a private per-priority RNG and does not perturb search RNGs.
            let mut hallpasses = match order {
                TotalizerOrder::Cluster => {
                    let domains: Vec<Vec<usize>> =
                        constraints.iter().map(|c| participating_sections(input, c)).collect();
                    cluster_order(&domains).into_iter().flat_map(|i| groups[i].iter().copied()).collect()
                }
                _ => groups.into_iter().flatten().collect::<Vec<_>>(),
            };
            if order == TotalizerOrder::Shuffle {
                fastrand::Rng::with_seed(seed.wrapping_add(u64::from(priority))).shuffle(&mut hallpasses);
            }
            levels.push(PreparedLevel { clause_end: template.clauses.len(), last_var: template.last_var, hallpasses });
        }
        Ok(Self { template, levels })
    }

    pub fn hallpass_count(&self, priority: u8) -> usize {
        self.levels.get(priority as usize).map_or(0, |level| level.hallpasses.len())
    }

    pub fn instance(&self, limits: &Score, priority: u8) -> Encoding {
        // Copy prepared clauses without rerunning any criterion encoder. Later
        // priority variables must not collide with this probe's counter variables.
        let level = &self.levels[priority as usize];
        let mut encoding = self.template.clone();
        encoding.clauses.truncate(level.clause_end);
        encoding.last_var = level.last_var;
        encoding.hallpasses.retain(|&p, _| p <= priority);
        encoding.problems.retain(|_, (p, _)| *p <= priority);
        encoding.hallpass_order.retain(|&var| var <= level.last_var);
        encoding.faculty_day_vars.retain(|_, var| *var <= level.last_var);

        // Counting constraints are the only clauses constructed for each probe.
        for p in 0..=priority {
            let hallpasses = &self.levels[p as usize].hallpasses;
            let k = limits.levels[p as usize] as usize;
            if k == 0 {
                for &h in hallpasses {
                    encoding.add_clause(vec![-h]);
                }
            } else if k == 1 && hallpasses.len() <= 30 {
                encoding.pairwise_at_most_one(hallpasses);
            } else {
                encoding.ordered_totalizer_at_most_k(hallpasses, k, None);
            }
        }
        encoding
    }
}

// Section identities connect curriculum rules directly and faculty rules through
// every participating section. Deduplication prevents shared teaching double counts.
fn participating_sections(input: &Input, criterion: &SatCriterion) -> Vec<usize> {
    use SatCriterion::*;
    let mut sections = match criterion {
        Conflict { sections, .. } => sections.to_vec(),
        AntiConflict { single, group, .. } => std::iter::once(*single).chain(group.iter().copied()).collect(),
        RoomPreference { section, .. } | TimeSlotPreference { section, .. } => vec![*section],
        TimePatternMatch { sections, .. } => sections.clone(),
        FacultySameDayOff { faculty, .. } => {
            faculty.iter().flat_map(|&f| input.faculty[f].sections.iter().copied()).collect()
        }
        FacultyDaysOff { faculty, .. }
        | FacultyEvenlySpread { faculty, .. }
        | FacultyNoRoomSwitch { faculty, .. }
        | FacultyTooManyRooms { faculty, .. }
        | FacultyGapTooLong { faculty, .. }
        | FacultyGapTooShort { faculty, .. }
        | FacultyClusterTooLong { faculty, .. }
        | FacultyClusterTooShort { faculty, .. } => input.faculty[*faculty].sections.clone(),
    };
    sections.sort_unstable();
    sections.dedup();
    sections
}

// Edge weight is the number of shared sections. Walk the strongest remaining
// edge, preferring highly connected criteria and then original criterion order.
fn cluster_order(domains: &[Vec<usize>]) -> Vec<usize> {
    let mut by_section: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (criterion, sections) in domains.iter().enumerate() {
        for &section in sections {
            by_section.entry(section).or_default().push(criterion);
        }
    }
    let mut edges = vec![BTreeMap::<usize, usize>::new(); domains.len()];
    for group in by_section.values() {
        for (i, &a) in group.iter().enumerate() {
            for &b in &group[i + 1..] {
                *edges[a].entry(b).or_default() += 1;
                *edges[b].entry(a).or_default() += 1;
            }
        }
    }

    // Restart at the strongest unvisited vertex when a connected walk ends.
    let degree: Vec<usize> = edges.iter().map(|neighbors| neighbors.values().sum()).collect();
    let mut visited = vec![false; domains.len()];
    let mut result = Vec::with_capacity(domains.len());
    let mut previous: Option<usize> = None;
    while result.len() < domains.len() {
        let neighbor = previous.and_then(|p| {
            edges[p]
                .iter()
                .filter(|(i, _)| !visited[**i])
                .max_by_key(|(i, weight)| (**weight, degree[**i], Reverse(**i)))
                .map(|(&i, _)| i)
        });
        let next = neighbor.unwrap_or_else(|| {
            (0..domains.len()).filter(|&i| !visited[i]).max_by_key(|&i| (degree[i], Reverse(i))).unwrap()
        });
        visited[next] = true;
        result.push(next);
        previous = Some(next);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Days;

    #[test]
    fn clustering_keeps_disjoint_curriculum_components_together() {
        // Original order interleaves two triangles and an isolated criterion.
        let domains = vec![vec![0, 1], vec![3, 4], vec![1, 2], vec![4, 5], vec![0, 2], vec![3, 5], vec![6]];
        let order = cluster_order(&domains);
        assert_eq!(order, vec![0, 2, 4, 1, 3, 5, 6]);
        assert!(cluster_order(&[]).is_empty());
    }

    fn roomless_input() -> Input {
        let mut input = crate::shared_day_off_tests::input();
        input.criteria.clear();
        for section in &mut input.sections {
            section.rooms.clear();
        }
        input
    }

    fn pin_times(encoding: &mut Encoding, times: &[usize]) {
        for (section, &time) in times.iter().enumerate() {
            encoding.add_clause(vec![encoding.section_time_vars[&(section, time)]]);
        }
    }

    #[test]
    fn all_orders_preserve_budgets_and_exclude_future_priorities() {
        let input = roomless_input();
        let mut criteria = SatCriteria::new();
        for sections in [[0, 1], [1, 2], [0, 2]] {
            criteria.add_criterion(SatCriterion::Conflict { sections, priority: 1 });
        }
        criteria.add_criterion(SatCriterion::Conflict { sections: [0, 3], priority: 2 });
        for order in [TotalizerOrder::Current, TotalizerOrder::Shuffle, TotalizerOrder::Cluster] {
            let prepared = PreparedCnf::new(&input, &criteria, order, 17).unwrap();
            let original_clauses = prepared.template.clauses.clone();
            let repeated = PreparedCnf::new(&input, &criteria, order, 17).unwrap();
            assert_eq!(prepared.levels[1].hallpasses, repeated.levels[1].hallpasses);
            for pattern in 0..27 {
                let times = [pattern % 3, pattern / 3 % 3, pattern / 9, 0];
                let violations = usize::from(times[0] == times[1])
                    + usize::from(times[1] == times[2])
                    + usize::from(times[0] == times[2]);
                for allowed in 0..=3 {
                    let mut limits = Score::new();
                    limits.levels[1] = allowed;
                    let mut early = prepared.instance(&limits, 1);
                    assert!(early.problems.values().all(|(p, _)| *p <= 1));
                    pin_times(&mut early, &times);
                    assert_eq!(early.solve().unwrap().is_some(), violations <= allowed as usize);

                    // The future conflict only becomes hard when its priority is selected.
                    let mut later = prepared.instance(&limits, 2);
                    pin_times(&mut later, &times);
                    assert_eq!(
                        later.solve().unwrap().is_some(),
                        violations <= allowed as usize && times[0] != times[3]
                    );
                }
            }
            // Probe counters must never mutate the prepared clause prefix.
            assert_eq!(prepared.template.clauses, original_clauses);
        }
    }

    #[test]
    fn prepared_prefixes_keep_shared_faculty_day_links() {
        let input = roomless_input();
        let mut criteria = SatCriteria::new();
        criteria.add_criterion(SatCriterion::FacultyDaysOff {
            faculty: 0,
            days_to_check: Days::parse("MT").unwrap(),
            desired_days_off: 1,
            priority: 1,
        });
        criteria.add_criterion(SatCriterion::FacultySameDayOff {
            faculty: [0, 1],
            days_to_check: Days::parse("MT").unwrap(),
            priority: 2,
        });
        let prepared = PreparedCnf::new(&input, &criteria, TotalizerOrder::Current, 0).unwrap();
        assert_eq!(prepared.template.faculty_day_vars.len(), 4);
        for (times, individual, shared) in
            [([0, 2, 0, 2], true, true), ([0, 2, 1, 2], true, false), ([0, 1, 0, 2], false, false)]
        {
            let mut early = prepared.instance(&Score::new(), 1);
            assert_eq!(early.faculty_day_vars.len(), 2);
            pin_times(&mut early, &times);
            assert_eq!(early.solve().unwrap().is_some(), individual);
            let mut later = prepared.instance(&Score::new(), 2);
            pin_times(&mut later, &times);
            assert_eq!(later.solve().unwrap().is_some(), individual && shared);
        }
    }

    #[test]
    fn ordered_totalizer_handles_counts_near_one_hundred() {
        for order in [TotalizerOrder::Current, TotalizerOrder::Shuffle, TotalizerOrder::Cluster] {
            for true_count in [97, 98, 99] {
                let mut encoding = Encoding::new();
                let mut inputs: Vec<_> = (0..160).map(|_| encoding.new_var()).collect();
                for (i, &var) in inputs.iter().enumerate() {
                    encoding.add_clause(vec![if i < true_count { var } else { -var }]);
                }
                match order {
                    TotalizerOrder::Current => (),
                    TotalizerOrder::Shuffle => fastrand::Rng::with_seed(9).shuffle(&mut inputs),
                    TotalizerOrder::Cluster => inputs.reverse(),
                }
                encoding.ordered_totalizer_at_most_k(&inputs, 98, None);
                assert_eq!(encoding.solve().unwrap().is_some(), true_count <= 98);
            }
        }
    }
}
