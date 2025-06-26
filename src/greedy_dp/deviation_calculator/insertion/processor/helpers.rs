use crate::core::{Job, BatchSchedule};
use crate::greedy_dp::{Decision, ActiveLog};
use super::calculators::calculate_cascading_completion;
use crate::greedy_dp::deviation_calculator::insertion::helpers::try_insertion_at_batch;

// Helper function to create end option for displaced job
pub fn create_end_option_for_displaced_job(
    job: &Job,
    schedule: &BatchSchedule,
    updated_completion_at_index: Option<(usize, u32)>,
) -> ActiveLog {
    let last_batch_completion = if schedule.batches.is_empty() {
        0
    } else {
        let last_index = schedule.batches.len() - 1;
        calculate_cascading_completion(last_index, schedule, updated_completion_at_index)
    };

    let release_date = job.release_date.max(last_batch_completion);
    let completion = release_date + job.processing_time;
    let net_deviation = job.due_date as i32 - completion as i32;

    ActiveLog::new(net_deviation, Decision::CreateEnd { job_code: job.code })
}

// Refactored function to get all possible ActiveLogs for displaced job
pub fn get_all_possible_active_logs_for_displaced_job(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    updated_completion_at_index: Option<(usize, u32)>,
) -> Vec<Vec<ActiveLog>> {
    let mut all_possibilities: Vec<Vec<ActiveLog>> = Vec::new();

    // Handle case where we go beyond all existing batches
    if batch_index >= schedule.batches.len() {
        let release_date = job.release_date.max(prev_completion);
        let completion = release_date + job.processing_time;
        let net_deviation = job.due_date as i32 - completion as i32;

        all_possibilities.push(vec![ActiveLog::new(
            net_deviation,
            Decision::CreateEnd { job_code: job.code },
        )]);
        return all_possibilities;
    }

    // Try inserting at each position from batch_index onwards
    for index in batch_index..schedule.batches.len() {
        let current_prev_completion = if index == 0 {
            0
        } else if index == batch_index {
            prev_completion
        } else {
            calculate_cascading_completion(index - 1, schedule, updated_completion_at_index)
        };

        let batch_possibilities = try_insertion_at_batch(
            job,
            index,
            schedule,
            current_prev_completion,
            updated_completion_at_index,
        );
        
        all_possibilities.extend(batch_possibilities);
    }

    // Always add the option to create new batch at the end
    let end_option = create_end_option_for_displaced_job(job, schedule, updated_completion_at_index);
    all_possibilities.push(vec![end_option]);

    all_possibilities
}

