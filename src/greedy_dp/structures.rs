#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Decision {
    InsertIn { batch_index: u32, job_code: u32 },
    CreateAt { batch_index: u32, job_code: u32 },
    CreateEnd { job_code: u32 },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InsertAction {
    InsertInBatch { batch_code: usize, job_code: u32 },
    PopAndCreateBatch { from_batch: usize, job_code: u32, at_pos: usize },
    PopAndInsertInBatch { from_batch: usize, job_code: u32, to_batch: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecisionHistory {
    deviation: i32,
    actions: Option<Vec<InsertAction>>,
}

impl DecisionHistory {
    pub fn new(deviation: i32) -> Self {
        DecisionHistory {
            deviation,
            actions: None,
        }
    }
}
