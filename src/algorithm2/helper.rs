use crate::core::{Batch, BatchSchedule, Job};
use super::EndDecision;

pub fn make_end_decision(schedule: &BatchSchedule, job: &Job) -> EndDecision {
    let last_batch = &schedule.batches[schedule.batches.len()-1];
    let cost_of_creating_after = job.due_date as i32 - (last_batch.completion_time as i32 + job.processing_time as i32);

    if !(size_check(20, last_batch, job)) {
        return EndDecision::CreateAfter(cost_of_creating_after);
    }

    let release_date = last_batch.release_date.max(job.release_date) as i32;
    let processing_time = last_batch.processing_time.max(job.processing_time) as i32;
    let cost_of_inserting_last = last_batch.min_due_time as i32 - (release_date + processing_time);

    let options = [
        (cost_of_creating_after, EndDecision::CreateAfter(cost_of_creating_after)),
        (cost_of_inserting_last, EndDecision::InsertAtLast(cost_of_inserting_last)),
    ];

    let (_, decision) = options
        .iter()
        .max_by_key(|&(lateness, _)| lateness)
        .expect("At least one options is to be present");
    
    *decision
}

pub fn locate_eligible_batch(schedule: &BatchSchedule, due: u32) -> Option<usize> {
    let min_index = schedule.batches.iter()
        .enumerate()
        .min_by_key(|(_, batch)| batch.min_due_time)
        .map(|(index, _)| index)?;

    schedule.batches.iter()
        .enumerate()
        .skip(min_index)
        .find(|(_, batch)| batch.min_due_time >= due)
        .map(|(index, _)| index)
}

pub fn size_check(size: u32, batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= size
}

pub fn find_cost_creating_before(schedule: &BatchSchedule, batch_index: usize, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = batch.release_date.max(job.release_date);
    let cost_creating_before = job.due_date as i32 - (release_date as i32 + job.processing_time as i32);

    let num = batch.release_date as i32 - job.release_date as i32;
    let to_add = if num < 0 {
        job.processing_time + (-num) as u32
    } else {
        job.processing_time
    };


    let min_cost = schedule.batches.iter()
        .enumerate()
        .skip(batch_index)
        .map(|(_, batch)| batch.min_due_time as i32 - (batch.completion_time as i32 + to_add as i32))
        .min()
        .unwrap_or(i32::MAX);

    min_cost.min(cost_creating_before)
}
