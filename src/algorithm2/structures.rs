#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Decision {
    CreateBefore(i32),
    InsertAtPosition(i32),
    CreateAfter(i32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndDecision {
    InsertAtLast(i32),
    CreateAfter(i32),
}
