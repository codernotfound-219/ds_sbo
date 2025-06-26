use std::cmp::Ordering;

use crate::core::{BatchSchedule, Job};
use crate::greedy_dp::{LogHistory, ActiveLog, Decision};
use super::utils::{process_active_log, compute_current_deviation, find_best_possibility};
use super::helpers::get_all_possible_active_logs_for_displaced_job;

// Helper function to handle displacement in main insertion logic
pub fn handle_main_displacement(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    current_prev_completion: u32,
    _updated_completion_at_index: Option<(usize, u32)>,
) -> LogHistory {
    let mut job_list: Vec<Job> = schedule.batches[batch_index].jobs.clone();
    let lp_job = job_list.pop().expect("Batch should not be empty");

    match lp_job.cmp(job) {
        Ordering::Less | Ordering::Equal => {
            LogHistory::new(i32::MIN, vec![Decision::NotPossible])
        }
        Ordering::Greater => {
            // Calculate deviation for inserting job in current batch (after removing lp_job)
            let (current_deviation_raw, completion) =
                compute_current_deviation(&job_list, job, current_prev_completion);
            
            // Get ALL possible placements for the displaced job
            let all_recursive_possibilities = get_all_possible_active_logs_for_displaced_job(
                batch_index + 1,
                schedule,
                &lp_job,
                completion,
                Some((batch_index, completion)),
            );
            
            // Find the best possibility
            let (_best_deviation, best_logs) = find_best_possibility(all_recursive_possibilities);
            
            // Combine current insertion with the best recursive possibility
            let mut final_logs = vec![ActiveLog::new(
                current_deviation_raw,
                Decision::InsertIn {
                    batch_index,
                    job_code: job.code,
                },
            )];
            final_logs.extend(best_logs);
            
            // Process into LogHistory
            process_active_log(final_logs)
        }
    }
}

// Helper function to handle displacement scenario
pub fn handle_displacement_scenario(
    job: &Job,
    batch_index: usize,
    schedule: &BatchSchedule,
    prev_completion: u32,
    _updated_completion_at_index: Option<(usize, u32)>,
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
            let (current_deviation, completion) =
                compute_current_deviation(&job_list, job, prev_completion);
            
            // Get all possibilities for the displaced job
            let recursive_possibilities = get_all_possible_active_logs_for_displaced_job(
                batch_index + 1,
                schedule,
                &lp_job,
                completion,
                Some((batch_index, completion)),
            );
            
            // Combine current insertion with each recursive possibility
            let mut combined_possibilities = Vec::new();
            for recursive_logs in recursive_possibilities {
                let mut combined_logs = vec![ActiveLog::new(
                    current_deviation,
                    Decision::InsertIn {
                        batch_index,
                        job_code: job.code,
                    },
                )];
                combined_logs.extend(recursive_logs);
                combined_possibilities.push(combined_logs);
            }
            
            combined_possibilities
        }
    }
}

