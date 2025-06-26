use crate::core::{BatchSchedule, Job};
use crate::greedy_dp::{ActiveLog, LogHistory, Decision};

// NOTE:
// this function calculates deviation due to insertion of job in current batch after popping lp_job
pub fn compute_current_deviation(list: &[Job], job: &Job, prev_completion: u32) -> (i32, u32) {
    let release_date = prev_completion
        .max(
            list.iter()
                .map(|cur_job| cur_job.release_date)
                .max()
                .unwrap(),
        )
        .max(job.release_date);
    let processing = job.processing_time.max(
        list.iter()
            .map(|cur_job| cur_job.processing_time)
            .max()
            .unwrap(),
    );
    let completion = processing + release_date;
    let due = job
        .due_date
        .min(list.iter().map(|cur_job| cur_job.due_date).min().unwrap());

    (due as i32 - completion as i32, completion)
}

// NOTE:
// This function calculates the completion time of previous batch when
// the tester job was put in one of the former batches.
pub fn calculate_cascading_completion(
    target_index: usize,
    schedule: &BatchSchedule,
    updated_completion_at_index: Option<(usize, u32)>,
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

// NOTE:
// this function finds the best possibility from the given list of ActiveLogs
pub fn find_best_possibility(possibilities: Vec<Vec<ActiveLog>>) -> (i32, Vec<ActiveLog>) {
    let mut best_deviation = i32::MIN;
    let mut best_logs: Vec<ActiveLog> = Vec::new();
    
    for possibility in possibilities {
        // Calculate aggregate/minimum for this possibility
        let aggregate: i32 = possibility.iter()
            .filter(|log| log.deviation < 0)
            .map(|log| log.deviation)
            .sum();
        let min_deviation = possibility.iter()
            .map(|log| log.deviation)
            .min()
            .unwrap_or(0);
        
        let possibility_deviation = if aggregate != 0 { aggregate } else { min_deviation };
        
        if possibility_deviation > best_deviation {
            best_deviation = possibility_deviation;
            best_logs = possibility;
        }
    }
    
    (best_deviation, best_logs)
}

// NOTE:
// this function flattens the activeLogs into one LogHistory
pub fn process_active_log(set_m: Vec<ActiveLog>) -> LogHistory {
    let invalid_logs: Vec<&ActiveLog> = set_m.iter().filter(|log| log.deviation == i32::MIN).collect();
    if !invalid_logs.is_empty() {
        return LogHistory::new(i32::MIN, vec![Decision::NotPossible]);
    }

    let aggregate: i32 = set_m.iter().filter(|log| log.deviation < 0).map(|log| log.deviation).sum();
    let min_deviation = set_m.iter().map(|log| log.deviation).min().unwrap();

    let actions: Vec<Decision> = set_m
        .iter()
        .map(|log| log.action)
        .collect();

    LogHistory::new(if aggregate != 0 { aggregate } else { min_deviation }, actions)
}
