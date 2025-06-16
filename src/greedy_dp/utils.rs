use crate::core::{Job, BatchSchedule, Batch};
use super::{cost_calculator::InsertAction, helper::size_check};

pub fn insert_before(schedule: &mut BatchSchedule, job: Job, actions: &[InsertAction]) {
    match actions[0] {
        InsertAction::InsertInBatch { batch_index, job_code } => {
            if job.code != job_code {
                panic!("Job code didnt match for InsertAction::InsertInBatch, batch_index: {}, job_code: {}", batch_index, job_code);
            }
            
            schedule.batches[batch_index].insert_end(job);
        },
        InsertAction::PopAndCreateNewBatch { batch_index: _, job_code: _ } => {
            panic!("First action is not insertion of cur_job")
        },
        InsertAction::PopAndInsertInNextBatch { batch_index: _, job_code: _} => {
            panic!("First action is not insertion of cur_job")
        },
    } 

    for action in actions.iter().skip(1) {
        match action {
            InsertAction::InsertInBatch { batch_index: _, job_code: _} => {
                panic!("Insert in Batch occurs twice in actions!");
            },
            InsertAction::PopAndCreateNewBatch { batch_index, job_code } => {
                if let Some(popped_job) = schedule.batches[*batch_index].pop_job() {
                    if popped_job.code != *job_code {
                        panic!("Job code didnt match for InsertAction::PopAndCreateNewBatch, batch_index: {}, job_code: {}", batch_index, job_code);
                    }

                    let mut batch = Batch::new(batch_index+2);
                    batch.insert_begin(popped_job);
                    schedule.insert_at_position(batch_index+1, batch);
                } else {
                    panic!("Popped from an empty batch in fn: insert_before");
                }
            },
            InsertAction::PopAndInsertInNextBatch { batch_index, job_code } => {
                if let Some(popped_job) = schedule.batches[*batch_index].pop_job() {
                    if popped_job.code != *job_code {
                        panic!("Job code didnt match for InsertAction::PopAndInsertInNextBatch, batch_index: {}, job_code: {}", batch_index, job_code);
                    }

                    schedule.batches[batch_index+1].insert_begin(popped_job);
                } else {
                    panic!("Popped from an empty batch in fn: insert_before");
                }
            },
        }
    }
}

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
