pub fn should_create_or_insert_last(schedule: &BatchSchedule, job: &Job) -> EndDecision {
    let last_batch = &schedule.batches[schedule.batches.len()-1];
    let cost_of_creating_after = job.due_date as i32 - (last_batch.completion_time as i32 + job.processing_time as i32);

    if !(size_check(20, last_batch, job)) {
        return EndDecision::CreateAfter(cost_of_creating_after);
    }

    let release_date = last_batch.release_date.max(job.release_date) as i32;
    let processing_time = last_batch.processing_time.max(job.processing_time) as i32;
    let cost_of_inserting_last = last_batch.min_due_time as i32 - (release_date + processing_time);

    let options = [
        (cost_of_creating_after, EndDecision::CreateAfter(cost_of_creating_after)),
        (cost_of_inserting_last, EndDecision::InsertAtLast(cost_of_inserting_last)),
    ];

    let (_, decision) = options
        .iter()
        .max_by_key(|&(lateness, _)| lateness)
        .expect("At least one options is to be present");
    
    *decision
}

pub fn locate_eligible_batch(schedule: &BatchSchedule, due: u32) -> Option<usize> {
    let min_index = schedule.batches.iter()
        .enumerate()
        .min_by_key(|(_, batch)| batch.min_due_time)
        .map(|(index, _)| index)?;

    schedule.batches.iter()
        .enumerate()
        .skip(min_index)
        .find(|(_, batch)| batch.min_due_time >= due)
        .map(|(index, _)| index)
}

pub fn size_check(size: u32, batch: &Batch, job: &Job) -> bool {
    batch.size + job.size <= size
}
