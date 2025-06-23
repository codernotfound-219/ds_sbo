use crate::core::{Job, BatchSchedule, Batch};
use super::cost_calculator::InsertAction;

pub fn employ_insert_action(schedule: &mut BatchSchedule, job: Job, actions: &[InsertAction]) {
    match actions[0] {
        InsertAction::InsertInBatch { batch_index, job_code } => {
            if job.code != job_code {
                panic!("Job code didnt match for InsertAction::InsertInBatch, batch_index: {}, job_code: {}", batch_index, job_code);
            }
            
            schedule.batches[batch_index].insert(job);
            schedule.update_parameters(batch_index);        // Update params as job was inserted
            // and not batch
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
                    schedule.update_parameters(*batch_index);        // Update params as job was popped

                    let mut batch = Batch::new(batch_index+2);
                    batch.insert(popped_job);
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
                    schedule.update_parameters(*batch_index);        // Update params as job was popped

                    schedule.batches[batch_index+1].insert(popped_job);
                    schedule.update_parameters(batch_index+1);              // update_params as job
                    // was inserted and not batch
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
    new_batch.insert(job);
    schedule.insert_at_position(batch_index+1, new_batch);
}

pub fn create_before(schedule: &mut BatchSchedule, batch_index: usize, job: Job) {
    let batch = &schedule.batches[batch_index];
    let mut new_batch = Batch::new(batch.code);
    new_batch.insert(job);
    schedule.insert_at_position(batch_index, new_batch);
}

pub fn create_end(schedule: &mut BatchSchedule, job: Job) {
    let batch_code = schedule.batches.len() + 1;
    let mut batch = Batch::new(batch_code);
    batch.insert(job);

    schedule.insert_end(batch);
}

pub fn insert_last(schedule: &mut BatchSchedule, job: Job) {
    let batch_index = schedule.batches.len()-1;
    schedule.batches[batch_index].insert(job);
}
