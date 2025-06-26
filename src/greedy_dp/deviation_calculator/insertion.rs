use std::cmp::Ordering;

use super::utils::{calculate_deviation, size_check};
use crate::{
    core::{BatchSchedule, Job},
    greedy_dp::{structures::LogHistory, ActiveLog, Decision},
};

pub fn insert_in(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
) -> Vec<LogHistory> {
    insert_in_helper(batch_index, schedule, job, prev_completion, None)
}

fn compute_current_deviation(list: &[Job], job: &Job, prev_completion: u32) -> (i32, u32) {
    let release_date = prev_completion
        .max(
            list.iter()
                .map(|cur_job| cur_job.release_date)
                .max()
                .unwrap(),
        )
        .max(job.release_date);
    let processing = job.processing_time.max(
        list.iter()
            .map(|cur_job| cur_job.processing_time)
            .max()
            .unwrap(),
    );
    let completion = processing + release_date;
    let due = job
        .due_date
        .min(list.iter().map(|cur_job| cur_job.due_date).min().unwrap());

    (due as i32 - completion as i32, completion)
}

fn insert_in_size_ok(
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

// Helper function to calculate cascading completion times
fn calculate_cascading_completion(
    target_index: usize,
    schedule: &BatchSchedule,
    updated_completion_at_index: Option<(usize, u32)>,
) -> u32 {
    if let Some((updated_index, updated_completion)) = updated_completion_at_index {
        if updated_index <= target_index {
            // We need to calculate the cascading effect
            let mut current_completion = updated_completion;

            // Calculate completion times for all batches between updated_index and target_index
            for i in (updated_index + 1)..=target_index {
                let batch = &schedule.batches[i];
                let release_date = batch.release_date.max(current_completion);
                current_completion = release_date + batch.processing_time;
            }

            return current_completion;
        }
    }

    // No update affects this batch, use original completion time
    schedule.batches[target_index].completion_time
}

// Helper function to create end option for displaced job
fn create_end_option_for_displaced_job(
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

// Helper function to handle direct insertion (size check passes)
fn handle_direct_insertion(
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

// Helper function to handle displacement scenario
fn handle_displacement_scenario(
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

// Helper function to try insertion at specific batch index
fn try_insertion_at_batch(
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

// Refactored function to get all possible ActiveLogs for displaced job
fn get_all_possible_active_logs_for_displaced_job(
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

// Helper function to find best possibility from multiple options
fn find_best_possibility(possibilities: Vec<Vec<ActiveLog>>) -> (i32, Vec<ActiveLog>) {
    let mut best_deviation = i32::MIN;
    let mut best_logs: Vec<ActiveLog> = Vec::new();
    
    for possibility in possibilities {
        // Calculate aggregate/minimum for this possibility
        let aggregate: i32 = possibility.iter()
            .filter(|log| log.deviation < 0)
            .map(|log| log.deviation)
            .sum();
        let min_deviation = possibility.iter()
            .map(|log| log.deviation)
            .min()
            .unwrap_or(0);
        
        let possibility_deviation = if aggregate != 0 { aggregate } else { min_deviation };
        
        if possibility_deviation > best_deviation {
            best_deviation = possibility_deviation;
            best_logs = possibility;
        }
    }
    
    (best_deviation, best_logs)
}

// Helper function to handle displacement in main insertion logic
fn handle_main_displacement(
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

fn process_active_log(set_m: Vec<ActiveLog>) -> LogHistory {
    let invalid_logs: Vec<&ActiveLog> = set_m.iter().filter(|log| log.deviation == i32::MIN).collect();
    if !invalid_logs.is_empty() {
        return LogHistory::new(i32::MIN, vec![Decision::NotPossible]);
    }

    let aggregate: i32 = set_m.iter().filter(|log| log.deviation < 0).map(|log| log.deviation).sum();
    let min_deviation = set_m.iter().map(|log| log.deviation).min().unwrap();

    let actions: Vec<Decision> = set_m
        .iter()
        .map(|log| log.action)
        .collect();

    LogHistory::new(if aggregate != 0 { aggregate } else { min_deviation }, actions)
}
