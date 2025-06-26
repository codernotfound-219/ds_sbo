use super::{calculate_deviation_for_direct_insertion, handle_displacement_due_to_cur_job};
use crate::greedy_dp::deviation_calculator::{
    batch_effects::calculate_cascading_completion,
    common::{decisions, CompletionUpdate},
};
use crate::{
    structures::{BatchSchedule, Job},
    greedy_dp::LogHistory,
};

// NOTE: Main public interface for job insertion
pub fn get_insertion_deviations(
    schedule: &BatchSchedule,
    job: &Job,
) -> Vec<LogHistory> {
    insertion_helper(0, schedule, job, 0, None)
}

// NOTE: Helper function that handles the core insertion logic
pub(crate) fn insertion_helper(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    completion_update: CompletionUpdate,
) -> Vec<LogHistory> {
    let mut results: Vec<LogHistory> = Vec::new();

    // // Handle case where we go beyond all existing batches: SEEMS LIKE NOT NEEDED
    // if batch_index >= schedule.batches.len() {
    //     return vec![create_end_beyond_batches(job, prev_completion)];
    // }

    // NOTE: Try inserting at each position from batch_index (jcurrent) onwards
    for index in batch_index..schedule.batches.len() {
        let current_prev_completion = calculate_prev_completion_for_index(
            index,
            batch_index,
            prev_completion,
            schedule,
            completion_update,
        );

        let log_history = insert_cur_job_at_batch(
            job,
            index,
            schedule,
            current_prev_completion,
            completion_update,
        );

        results.push(log_history);
    }

    results
}

// SEEMS NOT NEEDED
// /// Handle creation when going beyond all existing batches
// fn create_end_beyond_batches(job: &Job, prev_completion: u32) -> LogHistory {
//     let release_date = job.release_date.max(prev_completion);
//     let completion = release_date + job.processing_time;
//     let net_deviation = job.due_date as i32 - completion as i32;
//
//     LogHistory::new(net_deviation, vec![decisions::create_end(job.code)])
// }

// NOTE: Calculate the previous completion time upon virtual insertion
fn calculate_prev_completion_for_index(
    index: usize,
    batch_index: usize,
    prev_completion: u32,
    schedule: &BatchSchedule,
    completion_update: CompletionUpdate,
) -> u32 {
    if index == 0 {
        0
    } else if index == batch_index {
        prev_completion
    } else {
        calculate_cascading_completion(index - 1, schedule, completion_update)
    }
}

// NOTE: try inserting current job at given index
fn insert_cur_job_at_batch(
    job: &Job,
    index: usize,
    schedule: &BatchSchedule,
    current_prev_completion: u32,
    completion_update: CompletionUpdate,
) -> LogHistory {
    use crate::greedy_dp::deviation_calculator::common::size_check;

    if !size_check(&schedule.batches[index], job) {
        // Handle displacement scenario
        handle_displacement_due_to_cur_job(
            job,
            index,
            schedule,
            current_prev_completion,
            completion_update,
        )
    } else {
        // Size check passed - can insert directly
        let net_deviation =
            calculate_deviation_for_direct_insertion(index, schedule, job, current_prev_completion);

        LogHistory::new(net_deviation, vec![decisions::insert_at(index, job.code)])
    }
}
