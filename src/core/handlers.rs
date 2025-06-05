#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decision {
    CMPNextBatch,
    CreateBatchBefore,
    CreateBatchAfter,
    InsertAtPosition(usize, usize), // (batch_index, index_to_insert)
}

#[derive(Debug, Clone, Copy)]
pub struct DecisionLog {
    pub past_decision: Decision,
    pub lateness: u32,
}

