#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Decision {
    InsertIn { batch_index: usize, job_code: u32 },
    CreateAt { batch_index: usize, job_code: u32 },
    CreateEnd { job_code: u32 },
    NotPossible,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ActiveLog {
    pub deviation: i32,
    pub action: Decision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogHistory {
    pub deviation: i32,
    pub actions: Vec<Decision>,
}

impl ActiveLog {
    pub fn new(deviation: i32, action: Decision) -> Self {
        ActiveLog {
            deviation, 
            action,
        }
    }
}

impl LogHistory {
    pub fn new(deviation: i32, actions: Vec<Decision>) -> Self {
        LogHistory {
            deviation,
            actions,
        }
    }
}
