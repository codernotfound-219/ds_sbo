use std::error::Error;

use crate::structures::{BatchSchedule, Job};
use super::{execute_action, get_creation_deviations, get_insertion_deviations, LogHistory};

pub fn solver_helper(schedule: &mut BatchSchedule, job: Job) -> Result<(), Box<dyn Error>> {
    let insertion_list = get_insertion_deviations(schedule, &job);
    let creation_list = get_creation_deviations(schedule, &job);
    let best_action = get_action(&insertion_list, &creation_list);
    execute_action(best_action, schedule, job)
}

pub fn get_action<'a>(insertion_list: &'a [LogHistory], creation_list: &'a [LogHistory]) -> &'a LogHistory {
    let max_insertion_action = insertion_list
        .iter()
        .max_by_key(|log_history| log_history.deviation)
        .expect("Should have found maximum deviation in list of insertion deviations");

    let best_action = creation_list
        .iter()
        .chain(insertion_list.iter())
        .max_by_key(|log_history| log_history.deviation)
        .expect("Should have found maximum deviation in list of deviations");


    if max_insertion_action.deviation >= 0 {
        max_insertion_action
    } else {
        best_action
    }
}
