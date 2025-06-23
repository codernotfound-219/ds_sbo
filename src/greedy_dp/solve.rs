use super::{deviation_calculator, Decision, DecisionHistory};
use crate::core::{Batch, BatchSchedule, Job};

pub fn solve(list: &mut Vec<Job>) -> BatchSchedule {
    if list.is_empty() {
        panic!("Empty SLUJ passed to solver");
    }

    let mut schedule = BatchSchedule::new();
    let mut batch = Batch::new(1);

    Job::sort_release_date(list);
    batch.insert(list.pop().unwrap());
    schedule.insert_begin(batch);

    schedule
}

pub fn recurse(k: usize, schedule: &BatchSchedule, job: &Job) -> Vec<DecisionHistory> {
    let mut decision_set: Vec<DecisionHistory> = Vec::new();

    if k >= schedule.batches.len() {
        let release_date = job
            .release_date
            .max(schedule.batches[schedule.batches.len() - 1].completion_time);
        let completion = release_date + job.processing_time;
        let deviation = job.due_date as i32 - completion as i32;

        decision_set.push(DecisionHistory::new(
            deviation,
            Decision::CreateEnd { job_code: job.code },
        ));

        return decision_set;
    }

    for index in k..schedule.batches.len() {
        let batch = &schedule.batches[index];
        // TODO:  find deviation
        // 1. insert in 
        // 2. create in
        
        let deviation_create_in = deviation_calculator::create_in(index, schedule, job);
    }

    decision_set
}

fn insert_in(batch_index: usize, schedule: &BatchSchedule, job: &Job) -> i32 {
    let batch = &schedule.batches[batch_index];
    let release_date = job.release_date.max(batch.release_date);
    let processing = job.processing_time.max(batch.processing_time);
    let completion = release_date + processing;
    let due = job.due_date.min(batch.min_due_time);

    let deviation = due as i32 - completion as i32;
}
