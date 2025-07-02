use super::{Decision, LogHistory};
use crate::structures::{Batch, BatchSchedule, Job};

pub fn execute_action(loghistory: &LogHistory, schedule: &mut BatchSchedule, job: Job) {
    if loghistory.actions.is_empty() {
        panic!("No actions provided");
    }

    let mut current_job = job;
    let mut prev_batch_index = schedule.batches.len().saturating_sub(1);

    // println!("=======================================");
    // println!("Placing Job: J{}", job.code);

    for (i, action) in loghistory.actions.iter().enumerate() {
        if i > 0 {
            current_job = pop_job_from_batch(schedule, prev_batch_index);
        }

        match action {
            Decision::InsertIn { batch_index, job_code } => {
                validate_job_code(*job_code, &current_job);
                
                // println!("Inserting Job: J{} in Batch{}", job_code, batch_index+1);
                prev_batch_index = *batch_index;
                schedule.batches[*batch_index].insert(current_job);
            }
            Decision::CreateAt { batch_index, job_code } => {
                validate_job_code(*job_code, &current_job);

                // println!("Inserting Job: J{} in New Batch{}", job_code, batch_index+1);
                let mut batch = Batch::new(batch_index + 1);
                batch.insert(current_job);
                schedule.insert_at_position(*batch_index, batch);
                // println!("=======================================");
                return;
            }
            Decision::CreateEnd { job_code } => {
                validate_job_code(*job_code, &current_job);

                // println!("Inserting Job: J{} in New End Batch", job_code);
                let mut batch = Batch::new(schedule.batches.len() + 1);
                batch.insert(current_job);
                schedule.insert_end(batch);
                // println!("=======================================");
                return;
            }
            Decision::NotPossible => {
                panic!("NotPossible decision should not be executed");
            }
        }

        schedule.update_parameters();
    }
}

fn validate_job_code(expected_code: u32, job: &Job) {
    if expected_code != job.code {
        panic!("Job code mismatch: expected {}, got {}", expected_code, job.code);
    }
}

fn pop_job_from_batch(schedule: &mut BatchSchedule, batch_index: usize) -> Job {
    schedule.batches[batch_index]
        .pop_job()
        .expect("Could not pop job from batch")
}
