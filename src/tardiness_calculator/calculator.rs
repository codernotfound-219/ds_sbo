use crate::structures::BatchSchedule;

pub fn get_tardiness(schedule: &BatchSchedule) -> i32 {
    let mut aggregate = 0;
    for batch in schedule.batches.iter() {
        for job in batch.jobs.iter() {
            let completion = job.due_date as i32 - batch.completion_time as i32;

            if completion < 0 {
                if let Some(penalty) = job.penalty {
                    aggregate += completion.abs() * penalty as i32;
                } else {
                    aggregate += completion.abs();
                }
            }
        }
    }

    aggregate
}
