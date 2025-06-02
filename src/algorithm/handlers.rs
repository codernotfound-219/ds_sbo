#[derive(Debug, PartialEq)]
pub enum Priority {
    Head,
    Current,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Pass,      // Both jobs can meet their deadlines
    Fail(u32), // At least one job will be late (value = max lateness)
}

#[derive(Debug)]
pub struct CMP {
    pub priority: Priority,
    pub status: Status,
}
