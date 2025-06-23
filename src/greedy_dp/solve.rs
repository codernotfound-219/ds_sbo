use crate::core::{Job, Batch, BatchSchedule};
use super::{Decision, DecisionHistory};

pub fn solve(list: &mut Vec<Job>) -> BatchSchedule {
    if list.is_empty() { panic!("Empty SLUJ passed to solver"); }

    let mut schedule = BatchSchedule::new();
    let mut batch = Batch::new(1);

    Job::sort_release_date(list);
    batch.insert(list.pop().unwrap());
    schedule.insert_begin(batch);

    schedule
}

pub fn recurse(k: u32, schedule: &BatchSchedule, job: &Job) -> Vec<DecisionHistory> {
    let decision_set: Vec<DecisionHistory> = Vec::new();
    decision_set
}
