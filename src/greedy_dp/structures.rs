use super::cost_calculator::InsertAction;

#[derive(Clone, Debug)]
pub enum Decision {
    InsertBefore(i32, Vec<InsertAction>),
    CreateBefore(i32),
    InsertAtPosition(i32, Vec<InsertAction>),
    CreateAfter(i32),
    InsertAfter(i32, Vec<InsertAction>),
}


#[derive(Clone, Debug)]
pub enum EndDecision {
    InsertAtLast(i32, Vec<InsertAction>),
    CreateAfter(i32),
}
