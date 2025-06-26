use crate::core::{BatchSchedule, Job};
use crate::greedy_dp::{Decision, ActiveLog};
use crate::greedy_dp::deviation_calculator::utils::{calculate_deviation, size_check};
use super::processor::displacement::handle_displacement_scenario;

pub fn insert_in_size_ok(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = job
        .release_date
        .max(batch.release_date)
        .max(prev_completion);
    let processing = job.processing_time.max(batch.processing_time);
    let completion = release_date + processing;
    let due = job.due_date.min(batch.min_due_time);
    let deviation = due as i32 - completion as i32;

    calculate_deviation(batch_index + 1, schedule, completion, deviation)
}

// Helper function to handle direct insertion (size check passes)
pub fn handle_direct_insertion(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    prev_completion: u32,
) -> ActiveLog {
    let net_deviation = insert_in_size_ok(batch_index, schedule, job, prev_completion);
    ActiveLog::new(
        net_deviation,
        Decision::InsertIn {
            batch_index,
            job_code: job.code,
        },
    )
}

// Helper function to try insertion at specific batch index
pub fn try_insertion_at_batch(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    prev_completion: u32,
    updated_completion_at_index: Option<(usize, u32)>,
) -> Vec<Vec<ActiveLog>> {
    let mut possibilities = Vec::new();

    // Check if size allows direct insertion (no displacement needed)
    if size_check(&schedule.batches[batch_index], job) {
        let direct_insertion = handle_direct_insertion(job, batch_index, schedule, prev_completion);
        possibilities.push(vec![direct_insertion]);
    } else {
        // Size check failed - handle displacement
        let displacement_possibilities = handle_displacement_scenario(
            job, 
            batch_index, 
            schedule, 
            prev_completion, 
            updated_completion_at_index
        );
        possibilities.extend(displacement_possibilities);
    }

    possibilities
}

