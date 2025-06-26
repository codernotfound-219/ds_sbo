use crate::structures::{Batch, Job, BatchSchedule};
use super::Decision;

pub fn execute_actions(actions: Vec<Decision>, schedule: &mut BatchSchedule, job: Job) {
    match actions[0] { 
        Decision::InsertIn { batch_index, job_code } => {
            if job_code != job.code { panic!("Incorrect job code detected during execution"); }
            schedule.batches[batch_index].insert(job);
            schedule.update_parameters(batch_index);
        },
        Decision::CreateAt { batch_index, job_code } => {
            if job_code != job.code { panic!("Incorrect job code detected during execution"); }
            let mut batch = Batch::new(batch_index+1);
            batch.insert(job);
            schedule.insert_at_position(batch_index, batch);
        },
        Decision::CreateEnd { job_code } => {
            if job_code != job.code { panic!("Incorrect job code detected during execution"); }
            let mut batch = Batch::new(schedule.batches.len()+1);
            batch.insert(job);
            schedule.insert_end(batch);
        },
        Decision::NotPossible => { panic!("i32::MIN was sent in as deviation for execution"); },
    }

    // for action in 1..actions.len() {
    //
    // }
}

// TODO:
// how do you find the location to which popped job is from?
