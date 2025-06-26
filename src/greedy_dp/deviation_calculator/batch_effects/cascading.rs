use super::super::common::CompletionUpdate;
use crate::structures::BatchSchedule;

// NOTE: Calculate cascading completion times occuring due to virtual insertion of displaced jobs
pub fn calculate_cascading_completion(
    target_index: usize,
    schedule: &BatchSchedule,
    updated_completion_at_index: CompletionUpdate,
) -> u32 {
    if let Some((updated_index, updated_completion)) = updated_completion_at_index {
        if updated_index <= target_index {
            // We need to calculate the cascading effect
            let mut current_completion = updated_completion;

            // Calculate completion times for all batches between updated_index and target_index
            for i in (updated_index + 1)..=target_index {
                let batch = &schedule.batches[i];
                let release_date = batch.release_date.max(current_completion);
                current_completion = release_date + batch.processing_time;
            }

            return current_completion;
        }
    }

    // No update affects this batch, use original completion time
    schedule.batches[target_index].completion_time
}

// NOTE: Calculate completion time of the last batch, considering any updates due to displaced jobs
pub fn calculate_last_batch_completion(
    schedule: &BatchSchedule,
    updated_completion_at_index: CompletionUpdate,
) -> u32 {
    if schedule.batches.is_empty() {
        return 0;
    }

    let last_index = schedule.batches.len() - 1;
    calculate_cascading_completion(last_index, schedule, updated_completion_at_index)
}
