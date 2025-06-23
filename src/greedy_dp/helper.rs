use crate::core::{Batch, Job};
use crate::resources::BATCH_CAPACITY;

pub fn size_check(batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= BATCH_CAPACITY
}

