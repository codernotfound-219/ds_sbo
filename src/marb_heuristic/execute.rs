use std::error::Error;

use super::structure::{MarbBatch, EligibleBatch};
use super::calculators::{
    get_new_attribute_ratio,
    get_eligible_batches,
};
use crate::structures::Job;

pub fn get_formed_batches(mut list: Vec<Job>) -> Result<Vec<MarbBatch>, Box<dyn Error>> {
    if list.is_empty() {
        return Err("Empty SLUJ passed to solver".into());
    }

    let mut formed_batches: Vec<MarbBatch> = Vec::new();
    let mut batch = MarbBatch::new(1);

    Job::sort_due_date_by_code(&mut list);
    batch.insert(list.pop().unwrap());
    formed_batches.push(batch);

    loop {
        if list.is_empty() {
            break;
        }
        execute_action(&mut formed_batches, list.pop().unwrap());
    }

    Ok(formed_batches)
}

fn execute_action(formed_batches: &mut Vec<MarbBatch>, job: Job) {
    let eligible_batches: Vec<EligibleBatch> = get_eligible_batches(formed_batches, job.size);

    let mut min_ar = f64::MAX;
    let mut min_ar_index: usize = usize::MAX;

    for batch in eligible_batches {
        let new_ar = get_new_attribute_ratio(&formed_batches[batch.index], &job);
        if new_ar < batch.attribute_ratio && min_ar > new_ar {
            min_ar = new_ar;
            min_ar_index = batch.index;
        }
    }

    if min_ar == f64::MAX {
        let mut batch = MarbBatch::new(formed_batches.len() + 1);
        batch.insert(job);
        formed_batches.push(batch);
        return;
    }

    formed_batches[min_ar_index].insert(job);
}
