use super::error::Result;
use super::input::*;
use super::score::*;

// SAT Criterion types, mirroring the Python ConstraintType union
#[derive(Debug, Clone)]
pub enum SatCriterion {
    // A soft conflict constraint: two sections cannot be scheduled at overlapping times
    Conflict {
        sections: [usize; 2], // Indices of the two sections
        priority: u8,
    },

    // An anti-conflict: one section must be scheduled at the same time as one from a group
    AntiConflict {
        single: usize,     // Index of the single section
        group: Vec<usize>, // Indices of the group sections
        priority: u8,
    },

    // A preference to avoid a specific room for a section
    RoomPreference {
        section: usize,
        room: usize,
        priority: u8,
    },

    // A preference to avoid a specific time slot for a section
    TimeSlotPreference {
        section: usize,
        time_slot: usize,
        priority: u8,
    },

    // A preference for a faculty member to have a specific number of days off
    FacultyDaysOff {
        faculty: usize,
        days_to_check: Days,
        desired_days_off: usize,
        priority: u8,
    },

    // A preference for a faculty member's classes to be evenly spread across days
    FacultyEvenlySpread {
        faculty: usize,
        days_to_check: Days,
        priority: u8,
    },

    // A preference for a faculty member to not have to switch rooms between back-to-back classes
    FacultyNoRoomSwitch {
        faculty: usize,
        days_to_check: Days,
        max_gap_within_cluster: Duration,
        priority: u8,
    },

    // A preference for a faculty member to not have classes in too many different rooms
    FacultyTooManyRooms {
        faculty: usize,
        desired_max_rooms: usize,
        priority: u8,
    },

    // A gap between class clusters that is too long in a faculty member's schedule
    FacultyGapTooLong {
        faculty: usize,
        days_to_check: Days,
        duration: Duration,
        max_gap_within_cluster: Duration,
        priority: u8,
    },

    // A gap between class clusters that is too short in a faculty member's schedule
    FacultyGapTooShort {
        faculty: usize,
        days_to_check: Days,
        duration: Duration,
        max_gap_within_cluster: Duration,
        priority: u8,
    },

    // A cluster of classes that is too long in a faculty member's schedule
    FacultyClusterTooLong {
        faculty: usize,
        days_to_check: Days,
        duration: Duration,
        max_gap_within_cluster: Duration,
        priority: u8,
    },

    // A cluster of classes that is too short in a faculty member's schedule
    FacultyClusterTooShort {
        faculty: usize,
        days_to_check: Days,
        duration: Duration,
        max_gap_within_cluster: Duration,
        priority: u8,
    },

    // A constraint that all sections in the group should have the same time pattern
    TimePatternMatch {
        sections: Vec<usize>,
        priority: u8,
    },
}

impl SatCriterion {
    // Gets the priority of this criterion
    pub fn priority(&self) -> u8 {
        match self {
            SatCriterion::Conflict { priority, .. } => *priority,
            SatCriterion::AntiConflict { priority, .. } => *priority,
            SatCriterion::RoomPreference { priority, .. } => *priority,
            SatCriterion::TimeSlotPreference { priority, .. } => *priority,
            SatCriterion::FacultyDaysOff { priority, .. } => *priority,
            SatCriterion::FacultyEvenlySpread { priority, .. } => *priority,
            SatCriterion::FacultyNoRoomSwitch { priority, .. } => *priority,
            SatCriterion::FacultyTooManyRooms { priority, .. } => *priority,
            SatCriterion::FacultyGapTooLong { priority, .. } => *priority,
            SatCriterion::FacultyGapTooShort { priority, .. } => *priority,
            SatCriterion::FacultyClusterTooLong { priority, .. } => *priority,
            SatCriterion::FacultyClusterTooShort { priority, .. } => *priority,
            SatCriterion::TimePatternMatch { priority, .. } => *priority,
        }
    }
}

// Container for all SAT criteria, organized by priority level
pub struct SatCriteria {
    // Criteria organized by priority level, where the index is the priority level
    criteria_by_priority: Vec<Vec<SatCriterion>>,
}

impl SatCriteria {
    // Create an empty SatCriteria
    pub fn new() -> Self {
        Self { criteria_by_priority: Vec::new() }
    }

    // Add a criterion to the appropriate priority level, expanding the vector if needed
    pub fn add_criterion(&mut self, criterion: SatCriterion) {
        let priority = criterion.priority() as usize;

        // Ensure we have enough priority levels
        if priority >= self.criteria_by_priority.len() {
            self.criteria_by_priority.resize_with(priority + 1, Vec::new);
        }

        // Add the criterion to the appropriate level
        self.criteria_by_priority[priority].push(criterion);
    }

    // Convert Input into SatCriteria
    pub fn from_input(input: &Input) -> Result<Self> {
        let mut criteria = Self::new();

        // Add hard conflicts as priority level 0 constraints
        for (section_i, section) in input.sections.iter().enumerate() {
            for &other_section_i in &section.hard_conflicts {
                // Only add each conflict once (when section_i < other_section_i)
                if section_i < other_section_i {
                    criteria.add_criterion(SatCriterion::Conflict {
                        sections: [section_i, other_section_i],
                        priority: LEVEL_FOR_HARD_CONFLICT,
                    });
                }
            }
        }

        // Process all criteria in the input and convert them to SatCriterion
        for criterion in &input.criteria {
            match criterion {
                Criterion::SoftConflict { priority, sections } => {
                    criteria.add_criterion(SatCriterion::Conflict { sections: *sections, priority: *priority });
                }

                Criterion::AntiConflict { priority, single, group } => {
                    criteria.add_criterion(SatCriterion::AntiConflict {
                        single: *single,
                        group: group.clone(),
                        priority: *priority,
                    });
                }

                Criterion::RoomPreference { section, rooms_with_priorities } => {
                    for &RoomWithPriority { room, priority } in rooms_with_priorities {
                        criteria.add_criterion(SatCriterion::RoomPreference { section: *section, room, priority });
                    }
                }

                Criterion::TimeSlotPreference { section, time_slots_with_priorities } => {
                    for &TimeSlotWithPriority { time_slot, priority } in time_slots_with_priorities {
                        criteria.add_criterion(SatCriterion::TimeSlotPreference {
                            section: *section,
                            time_slot,
                            priority,
                        });
                    }
                }

                Criterion::FacultyPreference {
                    faculty,
                    sections: _, // We'll get these from the Faculty struct
                    days_to_check,
                    days_off,
                    evenly_spread,
                    no_room_switch,
                    too_many_rooms,
                    max_gap_within_cluster,
                    distribution_intervals,
                } => {
                    // Faculty days off
                    if let Some((priority, desired)) = days_off {
                        criteria.add_criterion(SatCriterion::FacultyDaysOff {
                            faculty: *faculty,
                            days_to_check: *days_to_check,
                            desired_days_off: *desired,
                            priority: *priority,
                        });
                    }

                    // Faculty evenly spread
                    if let Some(priority) = evenly_spread {
                        criteria.add_criterion(SatCriterion::FacultyEvenlySpread {
                            faculty: *faculty,
                            days_to_check: *days_to_check,
                            priority: *priority,
                        });
                    }

                    // Faculty no room switch
                    if let Some(priority) = no_room_switch {
                        criteria.add_criterion(SatCriterion::FacultyNoRoomSwitch {
                            faculty: *faculty,
                            days_to_check: *days_to_check,
                            max_gap_within_cluster: *max_gap_within_cluster,
                            priority: *priority,
                        });
                    }

                    // Faculty too many rooms
                    if let Some((priority, desired)) = too_many_rooms {
                        criteria.add_criterion(SatCriterion::FacultyTooManyRooms {
                            faculty: *faculty,
                            desired_max_rooms: *desired,
                            priority: *priority,
                        });
                    }

                    // Process distribution intervals
                    for interval in distribution_intervals {
                        match interval {
                            DistributionInterval::GapTooLong { priority, duration } => {
                                criteria.add_criterion(SatCriterion::FacultyGapTooLong {
                                    faculty: *faculty,
                                    days_to_check: *days_to_check,
                                    duration: *duration,
                                    max_gap_within_cluster: *max_gap_within_cluster,
                                    priority: *priority,
                                });
                            }
                            DistributionInterval::GapTooShort { priority, duration } => {
                                criteria.add_criterion(SatCriterion::FacultyGapTooShort {
                                    faculty: *faculty,
                                    days_to_check: *days_to_check,
                                    duration: *duration,
                                    max_gap_within_cluster: *max_gap_within_cluster,
                                    priority: *priority,
                                });
                            }
                            DistributionInterval::ClusterTooLong { priority, duration } => {
                                criteria.add_criterion(SatCriterion::FacultyClusterTooLong {
                                    faculty: *faculty,
                                    days_to_check: *days_to_check,
                                    duration: *duration,
                                    max_gap_within_cluster: *max_gap_within_cluster,
                                    priority: *priority,
                                });
                            }
                            DistributionInterval::ClusterTooShort { priority, duration } => {
                                criteria.add_criterion(SatCriterion::FacultyClusterTooShort {
                                    faculty: *faculty,
                                    days_to_check: *days_to_check,
                                    duration: *duration,
                                    max_gap_within_cluster: *max_gap_within_cluster,
                                    priority: *priority,
                                });
                            }
                        }
                    }
                }

                Criterion::SectionsWithDifferentTimePatterns { priority, sections } => {
                    criteria.add_criterion(SatCriterion::TimePatternMatch {
                        sections: sections.clone(),
                        priority: *priority,
                    });
                }
            }
        }

        Ok(criteria)
    }

    // Get all criteria at a specific priority level
    pub fn criteria_at_priority(&self, priority: u8) -> &[SatCriterion] {
        let priority = priority as usize;
        if priority < self.criteria_by_priority.len() { &self.criteria_by_priority[priority] } else { &[] }
    }

    // Get the maximum priority level
    pub fn max_priority(&self) -> u8 {
        // The maximum priority is one less than the length, or 0 if empty
        (self.criteria_by_priority.len().saturating_sub(1)) as u8
    }

    // Iterate through all priority levels
    pub fn priorities(&self) -> impl Iterator<Item = u8> + '_ {
        (0..self.criteria_by_priority.len())
            .filter_map(move |p| if !self.criteria_by_priority[p].is_empty() { Some(p as u8) } else { None })
    }

    // Total number of criteria at all priority levels
    pub fn total_criteria_count(&self) -> usize {
        self.criteria_by_priority.iter().map(|v| v.len()).sum()
    }
}

impl Default for SatCriteria {
    fn default() -> Self {
        Self::new()
    }
}
