#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Decision {
    InsertIn { batch_index: u32, job_code: u32 },
    CreateAt { batch_index: u32, job_code: u32 },
    CreateEnd { job_code: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecisionHistory {
    pub deviation: i32,
    pub decision: Decision,
    pub past_actions: Vec<Decision>,
}
