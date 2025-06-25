use std::cmp::Ordering;

use super::utils::{calculate_deviation, size_check};
use super::{create_end, create_in};
use crate::core::{BatchSchedule, Job};

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

    calculate_deviation(batch_index+1, schedule, completion, deviation)
}

pub fn insert_in(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
) -> Vec<i32> {
    insert_in_helper(batch_index, schedule, job, prev_completion, None)
}

pub fn insert_in_helper(
    batch_index: usize,
    schedule: &BatchSchedule,
    job: &Job,
    prev_completion: u32,
    updated_completion_at_index: Option<(usize, u32)>,
) -> Vec<i32> {
    let mut set_m: Vec<i32> = Vec::new();

    // Handle case where we go beyond all existing batches
    if batch_index >= schedule.batches.len() {
        let release_date = job.release_date.max(prev_completion);
        let completion = release_date + job.processing_time;
        let net_deviation = job.due_date as i32 - completion as i32;

        set_m.push(net_deviation);
        return set_m;
    }

    // Try inserting at each position from batch_index onwards
    for index in batch_index..schedule.batches.len() {
        let current_prev_completion = if index == 0 {
            0
        } else if index == batch_index {
            prev_completion
        } else {
            calculate_cascading_completion(index-1, schedule, updated_completion_at_index)
        };

        if !size_check(&schedule.batches[index], job) {
            let mut job_list: Vec<Job> = schedule.batches[index].jobs.clone();
            let lp_job = job_list.pop().expect("Batch should not be empty");

            match lp_job.cmp(job) {
                Ordering::Less | Ordering::Equal => { 
                    set_m.push(i32::MIN); 
                },
                Ordering::Greater => {
                    // Calculate deviation for inserting job in current batch (after removing lp_job)
                    let (current_deviation, completion) = compute_current_deviation(&job_list, job, current_prev_completion); // prev_completion remains 0
                    set_m.push(current_deviation);

                    // Recursively handle the displaced job
                    let recursive_results = insert_in_helper(index + 1, schedule, &lp_job, completion, Some((index, completion)));
                    set_m.extend(recursive_results);
                }
            }
        } else {
            // Size check passed - can insert directly
            let net_deviation = insert_in_size_ok(index, schedule, job, current_prev_completion);
            set_m.push(net_deviation) 
        }
        
        // Add result for creating new batch at this position (for every index)
        // set_m.push(create_in(index, schedule, job));
    }

    // Add option to create new batch at the end (only once, after all variations)
    // set_m.push(create_end(schedule, job));


    set_m
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
