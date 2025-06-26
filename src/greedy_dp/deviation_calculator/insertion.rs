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

// Modified function with debug prints
fn get_all_possible_active_logs_for_displaced_job(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    updated_completion_at_index: Option<(usize, u32)>,
) -> Vec<Vec<ActiveLog>> {
    let mut all_possibilities: Vec<Vec<ActiveLog>> = Vec::new();

    println!("=== Getting possibilities for job {} starting at batch {} ===", job.code, batch_index);

    // Handle case where we go beyond all existing batches
    if batch_index >= schedule.batches.len() {
        let release_date = job.release_date.max(prev_completion);
        let completion = release_date + job.processing_time;
        let net_deviation = job.due_date as i32 - completion as i32;

        println!("Job {} beyond all batches, creating at end with deviation {}", job.code, net_deviation);
        all_possibilities.push(vec![ActiveLog::new(
            net_deviation,
            Decision::CreateEnd { job_code: job.code },
        )]);
        return all_possibilities;
    }

    // Try inserting at each position from batch_index onwards
    for index in batch_index..schedule.batches.len() {
        println!("Trying job {} at batch index {}", job.code, index);
        
        let current_prev_completion = if index == 0 {
            0
        } else if index == batch_index {
            prev_completion
        } else {
            calculate_cascading_completion(index - 1, schedule, updated_completion_at_index)
        };

        println!("Current prev completion for batch {}: {}", index, current_prev_completion);

        // Check if size allows direct insertion (no displacement needed)
        let size_check_result = size_check(&schedule.batches[index], job);
        println!("Size check for job {} at batch {}: {}", job.code, index, size_check_result);
        
        if size_check_result {
            // Size check passed - can insert directly
            let net_deviation = insert_in_size_ok(index, schedule, job, current_prev_completion);
            println!("Direct insertion of job {} at batch {} gives deviation: {}", job.code, index, net_deviation);
            all_possibilities.push(vec![ActiveLog::new(
                net_deviation,
                Decision::InsertIn {
                    batch_index: index,
                    job_code: job.code,
                },
            )]);
        } else {
            // Size check failed - need to displace
            println!("Size check failed, attempting displacement");
            let mut job_list: Vec<Job> = schedule.batches[index].jobs.clone();
            let lp_job = job_list.pop().expect("Batch should not be empty");

            println!("Comparing job {} (processing: {}) with lp_job {} (processing: {})", 
                job.code, job.processing_time, lp_job.code, lp_job.processing_time);

            match lp_job.cmp(job) {
                Ordering::Less | Ordering::Equal => {
                    println!("Job {} cannot displace lp_job {} - skipping", job.code, lp_job.code);
                    continue;
                }
                Ordering::Greater => {
                    println!("Job {} can displace lp_job {}", job.code, lp_job.code);
                    // Can displace - calculate deviation for current insertion
                    let (current_deviation, completion) =
                        compute_current_deviation(&job_list, job, current_prev_completion);
                    
                    println!("Current deviation for job {} displacing from batch {}: {}", job.code, index, current_deviation);
                    
                    // Get all possibilities for the displaced job
                    let recursive_possibilities = get_all_possible_active_logs_for_displaced_job(
                        index + 1,
                        schedule,
                        &lp_job,
                        completion,
                        Some((index, completion)),
                    );
                    
                    println!("Got {} recursive possibilities for displaced job {}", recursive_possibilities.len(), lp_job.code);
                    
                    // Combine current insertion with each recursive possibility
                    for (i, recursive_logs) in recursive_possibilities.iter().enumerate() {
                        let mut combined_logs = vec![ActiveLog::new(
                            current_deviation,
                            Decision::InsertIn {
                                batch_index: index,
                                job_code: job.code,
                            },
                        )];
                        combined_logs.extend(recursive_logs.clone());
                        println!("Possibility {}: Current job {} at batch {}, then: {:?}", 
                            i, job.code, index, recursive_logs);
                        all_possibilities.push(combined_logs);
                    }
                }
            }
        }
    }

    // Calculate the correct completion time for creating at the end
    let last_batch_completion = if schedule.batches.is_empty() {
        0
    } else {
        // Get the completion time of the last batch, considering any updates
        let last_index = schedule.batches.len() - 1;
        calculate_cascading_completion(last_index, schedule, updated_completion_at_index)
    };

    // Always add the option to create new batch at the end
    let release_date = job.release_date.max(last_batch_completion);
    let completion = release_date + job.processing_time;
    let net_deviation = job.due_date as i32 - completion as i32;

    println!("Adding create-at-end option for job {} with deviation {}", job.code, net_deviation);
    all_possibilities.push(vec![ActiveLog::new(
        net_deviation,
        Decision::CreateEnd { job_code: job.code },
    )]);

    println!("Total {} possibilities for job {}", all_possibilities.len(), job.code);
    for (i, possibility) in all_possibilities.iter().enumerate() {
        println!("Possibility {}: {:?}", i, possibility);
    }
    println!("=== End possibilities for job {} ===", job.code);

    all_possibilities
}

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
            let mut job_list: Vec<Job> = schedule.batches[index].jobs.clone();
            let lp_job = job_list.pop().expect("Batch should not be empty");

            match lp_job.cmp(job) {
                Ordering::Less | Ordering::Equal => {
                    set_final.push(LogHistory::new(i32::MIN, vec![Decision::NotPossible]));
                }
                Ordering::Greater => {
                    println!("=== Processing displacement for job {} at batch {} ===", job.code, index);
                    
                    // Calculate deviation for inserting job in current batch (after removing lp_job)
                    // This should include cascading effects on subsequent batches in the modified schedule
                    let (current_deviation_raw, completion) =
                        compute_current_deviation(&job_list, job, current_prev_completion);
                    
                    println!("Current deviation for main job {} at batch {}: {}", 
                        job.code, index, current_deviation_raw);
                    
                    // Get ALL possible placements for the displaced job
                    let all_recursive_possibilities = get_all_possible_active_logs_for_displaced_job(
                        index + 1,
                        schedule,
                        &lp_job,
                        completion,
                        Some((index, completion)),
                    );
                    
                    // Find the best possibility (maximum deviation)
                    let mut best_deviation = i32::MIN;
                    let mut best_logs: Vec<ActiveLog> = Vec::new();
                    
                    println!("Evaluating {} possibilities for displaced job {}", all_recursive_possibilities.len(), lp_job.code);
                    
                    for (i, possibility) in all_recursive_possibilities.iter().enumerate() {
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
                        
                        println!("Possibility {}: aggregate={}, min={}, final={}, actions={:?}", 
                            i, aggregate, min_deviation, possibility_deviation, 
                            possibility.iter().map(|log| log.action).collect::<Vec<_>>());
                        
                        if possibility_deviation > best_deviation {
                            best_deviation = possibility_deviation;
                            best_logs = possibility.clone();
                            println!("  ^ This is now the best possibility");
                        }
                    }
                    
                    println!("Selected best possibility with deviation: {}", best_deviation);
                    
                    // Combine current insertion with the best recursive possibility
                    let mut final_logs = vec![ActiveLog::new(
                        current_deviation_raw,  // Use the cascading version
                        Decision::InsertIn {
                            batch_index: index,
                            job_code: job.code,
                        },
                    )];
                    final_logs.extend(best_logs);
                    
                    // Process into LogHistory
                    let log_history = process_active_log(final_logs);
                    println!("Final LogHistory: {:?}", log_history);
                    set_final.push(log_history);
                }
            }
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
