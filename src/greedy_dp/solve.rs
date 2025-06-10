use crate::core::{Job, Batch, BatchSchedule};
use super::{make_end_decision, locate_eligible_batch, EndDecision};

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
            // let result = make_decision(&schedule, batch_index, &current);
        } else {
            let result = make_end_decision(&schedule, &current);

            match result {
                EndDecision::CreateAfter(_) => {create_end(&mut schedule, current);},
                EndDecision::InsertAtLast(_) => {insert_last(&mut schedule, current);},
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

