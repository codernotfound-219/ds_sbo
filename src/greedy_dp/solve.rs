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

pub fn recurse(k: usize, schedule: &BatchSchedule, job: &Job) -> Vec<DecisionHistory> {
    let mut decision_set: Vec<DecisionHistory> = Vec::new();

    if k > schedule.batches.len() {
        let release_date = job.release_date.max(schedule.batches[schedule.batches.len() - 1].completion_time);
        let completion = release_date + job.processing_time;
        let deviation = job.due_date as i32 - completion as i32;

        decision_set.push( DecisionHistory::new(
            deviation,
            Decision::CreateEnd { job_code: job.code },
        ));

        return decision_set;
    }
    decision_set
}
