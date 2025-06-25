#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Decision {
    InsertIn { batch_index: usize, job_code: u32 },
    CreateAt { batch_index: usize, job_code: u32 },
    CreateEnd { job_code: u32 },
    NotPossible,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Log {
    deviation: i32,
    action: Decision,
}

impl Log {
    pub fn new(deviation: i32, action: Decision) -> Self {
        Log {
            deviation, 
            action,
        }
    }
}
