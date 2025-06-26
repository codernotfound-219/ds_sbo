use crate::greedy_dp::ActiveLog;

// NOTE: Type alias for completion time updates during cascading calculations
pub type CompletionUpdate = Option<(usize, u32)>;

// NOTE: Type alias for a set of insertion possibilities
pub type PossibilitySet = Vec<Vec<ActiveLog>>;

// NOTE: Result of a insertion operation
#[derive(Debug, Clone)]
pub struct InsertionResult {
    pub deviation: i32,
    pub completion: u32,
}

// NOTE: Common decision types (improves type safety)
pub mod decisions {
    use crate::greedy_dp::Decision;
    
    pub fn insert_at(batch_index: usize, job_code: u32) -> Decision {
        Decision::InsertIn { batch_index, job_code }
    }
    
    pub fn create_at(batch_index: usize, job_code: u32) -> Decision {
        Decision::CreateAt { batch_index, job_code }
    }
    
    pub fn create_end(job_code: u32) -> Decision {
        Decision::CreateEnd { job_code }
    }
}
