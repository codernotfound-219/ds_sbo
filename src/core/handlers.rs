#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decision {
    CMPNextBatch,
    CreateBatchBefore,
    CreateBatchAfter,
    InsertAtPosition(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct DecisionLog {
    pub past_decision: Decision,
    pub lateness: u32,
}

