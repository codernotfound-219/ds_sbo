#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decision {
    CreateBatchBefore(usize),
    CreateBatchAfter(usize),
    InsertAtPosition(usize, usize), // (batch_index, index_to_insert)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveVariant {
    CanInsert,
    MoveFailed(i32),
}

#[derive(Debug, Clone, Copy)]
pub struct DecisionLog {
    pub past_decision: Decision,
    pub lateness: u32,
}

