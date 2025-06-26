use crate::greedy_dp::deviation_calculator::insertion::helpers::insert_in_size_ok;
use crate::greedy_dp::deviation_calculator::insertion::processor::displacement::handle_main_displacement;
use crate::greedy_dp::deviation_calculator::utils::size_check;
use crate::greedy_dp::deviation_calculator::insertion::calculators::calculate_cascading_completion;
use crate::{
    core::{BatchSchedule, Job},
    greedy_dp::{LogHistory, Decision},
};

pub fn insert_in(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
) -> Vec<LogHistory> {
    insert_in_helper(batch_index, schedule, job, prev_completion, None)
}

// Main insertion helper function - now much cleaner
pub fn insert_in_helper(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    updated_completion_at_index: Option<(usize, u32)>,
) -> Vec<LogHistory> {
    let mut set_final: Vec<LogHistory> = Vec::new();

    // Handle case where we go beyond all existing batches
    if batch_index >= schedule.batches.len() {
        let release_date = job.release_date.max(prev_completion);
        let completion = release_date + job.processing_time;
        let net_deviation = job.due_date as i32 - completion as i32;

        set_final.push(LogHistory::new(
            net_deviation,
            vec![Decision::CreateEnd { job_code: job.code }],
        ));
        return set_final;
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

        if !size_check(&schedule.batches[index], job) {
            // Handle displacement scenario
            let log_history = handle_main_displacement(
                job,
                index,
                schedule,
                current_prev_completion,
                updated_completion_at_index,
            );
            set_final.push(log_history);
        } else {
            // Size check passed - can insert directly
            let net_deviation = insert_in_size_ok(index, schedule, job, current_prev_completion);
            set_final.push(LogHistory::new(
                net_deviation,
                vec![Decision::InsertIn {
                    batch_index: index,
                    job_code: job.code,
                }],
            ));
        }
    }

    set_final
}
