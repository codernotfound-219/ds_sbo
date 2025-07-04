use super::structure::MarbBatch;
use crate::structures::Job;

pub fn find_job(list: &[Job], job: &Job) -> Option<usize> {
    for (i, each_job) in list.iter().enumerate() {
        if each_job == job {
            return Some(i);
        }
    }

    None
}

pub fn compute_min_due_date(list: &[MarbBatch]) -> usize {
    let mut min_due_date = u32::MAX;
    let mut priority_index = 0;

    for (index, batch) in list.iter().enumerate() {
        let mut cur_due_date = u32::MAX;
        for job in batch.jobs.iter() {
            cur_due_date = cur_due_date.min(job.due_date);
        }

        if min_due_date > cur_due_date {
            min_due_date = cur_due_date;
            priority_index = index;
        }
    }

    priority_index
}

