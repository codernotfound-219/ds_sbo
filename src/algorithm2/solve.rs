use crate::core::{Job, Batch, BatchSchedule};
use super::structures::{Decision, EndDecision};

pub fn solve(list: &mut Vec<Job>) -> BatchSchedule {
    let mut schedule = BatchSchedule::new();
    let mut batch = Batch::new(1);

    Job::sort_release_date(list);
    batch.insert_begin(list.pop().unwrap());
    schedule.insert_begin(batch);

    while !list.is_empty() {
        let current = list.pop().expect("ERROR: expected a job in the sorted list");

        if let Some(batch_index) = locate_eligible_batch(&schedule, current.due_date) {
            // TODO: compare 3 testcases
            // let result = comparison(schedule, batch_index, current);
        } else {
            let result = should_create_or_insert_last(&schedule, &current);

            match result {
                EndDecision::CreateAfter(lateness) => {create_end(&mut schedule, current);},
                EndDecision::InsertAtLast(lateness) => {insert_last(&mut schedule, current);},
            }
        }
    }

    schedule
}

pub fn create_end(schedule: &mut BatchSchedule, job: Job) {
    let batch_code = schedule.batches.len() + 1;
    let mut batch = Batch::new(batch_code);
    batch.insert_begin(job);

    schedule.insert_end(batch);
}

pub fn insert_last(schedule: &mut BatchSchedule, job: Job) {
    let batch_index = schedule.batches.len()-1;
    schedule.batches[batch_index].insert_end(job);
}

pub fn should_create_or_insert_last(schedule: &BatchSchedule, job: &Job) -> EndDecision {
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
