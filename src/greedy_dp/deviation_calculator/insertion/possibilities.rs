use super::{
    calculations::calculate_deviation_for_direct_insertion,
    displacement::handle_displacement_due_to_lp_job,
};
use crate::structures::{BatchSchedule, Job};
use crate::greedy_dp::deviation_calculator::{
    batch_effects::{calculate_cascading_completion, calculate_last_batch_completion},
    common::{decisions, size_check, CompletionUpdate, PossibilitySet},
};
use crate::greedy_dp::ActiveLog;

// NOTE: Get all possible ActiveLogs for a displaced job
pub fn get_active_logs_for_lp_job(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    completion_update: CompletionUpdate,
) -> PossibilitySet {
    let mut all_possibilities: PossibilitySet = Vec::new();

    // Handle case where we go beyond all existing batches
    if batch_index >= schedule.batches.len() {
        let end_option = create_end_for_cur_job(job, prev_completion);
        all_possibilities.push(vec![end_option]);
        return all_possibilities;
    }

    // Try inserting at each position from batch_index onwards
    for index in batch_index..schedule.batches.len() {
        let current_prev_completion = calculate_prev_completion_for_lp_job(
            index,
            batch_index,
            prev_completion,
            schedule,
            completion_update,
        );

        let batch_possibilities = insert_displaced_job_at_batch(
            job,
            index,
            schedule,
            current_prev_completion,
            completion_update,
        );

        all_possibilities.extend(batch_possibilities);
    }

    //NOTE: add the option to create new batch at the end
    let end_option = create_end_option_for_displaced_job(job, schedule, completion_update);
    all_possibilities.push(vec![end_option]);

    all_possibilities
}

// NOTE: Try insertion at the specified batch index for displaced jobs
fn insert_displaced_job_at_batch(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    prev_completion: u32,
    completion_update: CompletionUpdate,
) -> PossibilitySet {
    let mut possibilities = Vec::new();

    // Check if size allows direct insertion (no displacement needed)
    if size_check(&schedule.batches[batch_index], job) {
        let direct_insertion =
            create_direct_insertion_log(job, batch_index, schedule, prev_completion);
        possibilities.push(vec![direct_insertion]);
    } else {
        // Size check failed - handle displacement
        let displacement_possibilities = handle_displacement_due_to_lp_job(
            job,
            batch_index,
            schedule,
            prev_completion,
            completion_update,
        );
        possibilities.extend(displacement_possibilities);
    }

    possibilities
}

// NOTE: Create an ActiveLog for direct insertion if size_check passed
fn create_direct_insertion_log(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    prev_completion: u32,
) -> ActiveLog {
    let net_deviation =
        calculate_deviation_for_direct_insertion(batch_index, schedule, job, prev_completion);
    ActiveLog::new(net_deviation, decisions::insert_at(batch_index, job.code))
}

// NOTE: Create end option for current job
fn create_end_for_cur_job(job: &Job, prev_completion: u32) -> ActiveLog {
    let release_date = job.release_date.max(prev_completion);
    let completion = release_date + job.processing_time;
    let net_deviation = job.due_date as i32 - completion as i32;

    ActiveLog::new(net_deviation, decisions::create_end(job.code))
}

// NOTE: Create end option for displaced job (lp_job)
fn create_end_option_for_displaced_job(
    job: &Job,
    schedule: &BatchSchedule,
    completion_update: CompletionUpdate,
) -> ActiveLog {
    let last_batch_completion = calculate_last_batch_completion(schedule, completion_update);
    let release_date = job.release_date.max(last_batch_completion);
    let completion = release_date + job.processing_time;
    let net_deviation = job.due_date as i32 - completion as i32;

    ActiveLog::new(net_deviation, decisions::create_end(job.code))
}

// NOTE: Calculate previous completion time for displaced job insertion
fn calculate_prev_completion_for_lp_job(
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

// NOTE: Return the maximum deviation + the actions that led to it.
pub fn find_best_possibility(possibilities: PossibilitySet) -> (i32, Vec<ActiveLog>) {
    let mut best_deviation = i32::MIN;
    let mut best_logs: Vec<ActiveLog> = Vec::new();

    for possibility in possibilities {
        // Calculate aggregate/minimum for this possibility
        let aggregate: i32 = possibility
            .iter()
            .filter(|log| log.deviation < 0)
            .map(|log| log.deviation)
            .sum();
        let min_deviation = possibility
            .iter()
            .map(|log| log.deviation)
            .min()
            .unwrap_or(0);

        let possibility_deviation = if aggregate != 0 {
            aggregate
        } else {
            min_deviation
        };

        if possibility_deviation > best_deviation {
            best_deviation = possibility_deviation;
            best_logs = possibility;
        }
    }

    (best_deviation, best_logs)
}
