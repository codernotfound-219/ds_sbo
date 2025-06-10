use crate::core::{Job, BatchSchedule, Batch};
use super::helper::size_check;

pub fn create_after(schedule: &mut BatchSchedule, batch_index: usize, job: Job) {
    let batch = &schedule.batches[batch_index];
    let mut new_batch = Batch::new(batch.code+1);
    new_batch.insert_begin(job);
    schedule.insert_at_position(batch_index+1, new_batch);
}

pub fn create_before(schedule: &mut BatchSchedule, batch_index: usize, job: Job) {
    let batch = &schedule.batches[batch_index];
    let mut new_batch = Batch::new(batch.code);
    new_batch.insert_begin(job);
    schedule.insert_at_position(batch_index, new_batch);
}

pub fn insert_at_position(schedule: &mut BatchSchedule, batch_index: usize, job: Job) {
    if size_check(20, &schedule.batches[batch_index], &job) {
        schedule.batches[batch_index].insert_begin(job);
        return;
    }

    let last_job = match schedule.batches[batch_index].pop_job() {
        Some(j) => j,
        None => panic!("Tried to pop from an empty batch!")
    };
    schedule.batches[batch_index].insert_begin(job);

    if batch_index+1 == schedule.batches.len() {
        let mut batch = Batch::new(batch_index+2);
        batch.insert_begin(last_job);
        schedule.insert_end(batch);
        return;
    }

    insert_at_position(schedule, batch_index+1, last_job);
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
