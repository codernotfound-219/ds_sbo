use super::structure::{MarbBatch, EligibleBatch};
use crate::resources::BATCH_CAPACITY;
use crate::structures::Job;

pub fn get_eligible_batches(formed_batches: &[MarbBatch], size: u32) -> Vec<EligibleBatch> {
    let mut result: Vec<EligibleBatch> = Vec::new();
    for (i, batch) in formed_batches.iter().enumerate() {
        if BATCH_CAPACITY - batch.size >= size {
            result.push(EligibleBatch {
                index: i,
                attribute_ratio: get_attribute_ratio(batch),
            });
        }
    }

    result
}

pub fn get_attribute_ratio(batch: &MarbBatch) -> f64 {
    batch.processing_time as f64 / batch.jobs.len() as f64
}

pub fn get_new_attribute_ratio(batch: &MarbBatch, job: &Job) -> f64 {
    let processing_time = batch.processing_time.max(job.processing_time) as f64;
    processing_time / (batch.jobs.len() as f64 + 1.0)
}

