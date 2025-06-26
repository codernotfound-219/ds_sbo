use std::cmp::Ordering;

use crate::structures::{BatchSchedule, Job};
use crate::greedy_dp::{LogHistory, ActiveLog, Decision};
use crate::greedy_dp::deviation_calculator::common::{CompletionUpdate, decisions, compute_current_deviation};
use super::possibilities::{get_active_logs_for_lp_job, find_best_possibility};

// NOTE: Handles displacement caused due to insertion of current job
pub fn handle_displacement_due_to_cur_job(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    current_prev_completion: u32,
    _completion_update: CompletionUpdate,
) -> LogHistory {
    let mut job_list: Vec<Job> = schedule.batches[batch_index].jobs.clone();
    let lp_job = job_list.pop().expect("Batch should not be empty");

    match lp_job.cmp(job) {
        Ordering::Less | Ordering::Equal => {
            LogHistory::new(i32::MIN, vec![Decision::NotPossible])
        }
        Ordering::Greater => {
            // Calculate deviation for inserting job in current batch (after removing lp_job)
            let insertion_result = compute_current_deviation(&job_list, job, current_prev_completion);
            
            // Get ALL possible placements for the displaced job
            let recursive_possibilities = get_active_logs_for_lp_job(
                batch_index + 1,
                schedule,
                &lp_job,
                insertion_result.completion,
                Some((batch_index, insertion_result.completion)),
            );
            
            // Find the best possibility
            let (_best_deviation, best_logs) = find_best_possibility(recursive_possibilities);
            
            // Combine current insertion with the best recursive possibility
            let mut final_logs = vec![ActiveLog::new(
                insertion_result.deviation,
                decisions::insert_at(batch_index, job.code),
            )];
            final_logs.extend(best_logs);
            
            // Process into LogHistory
            process_active_logs(final_logs)
        }
    }
}

// NOTE: Handles displacement scenario for when lp_job displaces other jobs
pub fn handle_displacement_due_to_lp_job(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    prev_completion: u32,
    _completion_update: CompletionUpdate,
) -> Vec<Vec<ActiveLog>> {
    let mut job_list: Vec<Job> = schedule.batches[batch_index].jobs.clone();
    let lp_job = job_list.pop().expect("Batch should not be empty");

    match lp_job.cmp(job) {
        Ordering::Less | Ordering::Equal => {
            // Cannot displace - return empty possibilities
            Vec::new()
        }
        Ordering::Greater => {
            // Can displace - calculate deviation for current insertion
            let insertion_result = compute_current_deviation(&job_list, job, prev_completion);
            
            // Get all possibilities for the displaced job
            let recursive_possibilities = get_active_logs_for_lp_job(
                batch_index + 1,
                schedule,
                &lp_job,
                insertion_result.completion,
                Some((batch_index, insertion_result.completion)),
            );
            
            // Combine current insertion with each recursive possibility
            let mut combined_possibilities = Vec::new();
            for recursive_logs in recursive_possibilities {
                let mut combined_logs = vec![ActiveLog::new(
                    insertion_result.deviation,
                    decisions::insert_at(batch_index, job.code),
                )];
                combined_logs.extend(recursive_logs);
                combined_possibilities.push(combined_logs);
            }
            
            combined_possibilities
        }
    }
}

// NOTE: Process a vector of ActiveLogs into a single LogHistory
fn process_active_logs(logs: Vec<ActiveLog>) -> LogHistory {
    let invalid_logs: Vec<&ActiveLog> = logs.iter().filter(|log| log.deviation == i32::MIN).collect();
    if !invalid_logs.is_empty() {
        return LogHistory::new(i32::MIN, vec![Decision::NotPossible]);
    }

    let aggregate: i32 = logs.iter()
        .filter(|log| log.deviation < 0)
        .map(|log| log.deviation)
        .sum();
    let min_deviation = logs.iter()
        .map(|log| log.deviation)
        .min()
        .unwrap_or(0);

    let actions: Vec<Decision> = logs.iter()
        .map(|log| log.action)
        .collect();

    LogHistory::new(
        if aggregate != 0 { aggregate } else { min_deviation }, 
        actions
    )
}
