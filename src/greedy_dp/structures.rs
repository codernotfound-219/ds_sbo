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

impl DecisionHistory {
    pub fn new(deviation: i32, decision: Decision) -> Self {
        DecisionHistory {
            deviation,
            decision,
            past_actions: Vec::new(),
        }
    }
}
