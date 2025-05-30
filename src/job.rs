#[derive(Debug)]
pub struct Job {
    pub code: u32,
    pub release_date: u32,
    pub due_date: u32,
    pub processing_time: u32,
    pub size: u32,
}

impl Job {
    pub fn new(code: u32, rel: u32, due: u32, pt: u32, size: u32) -> Job {
        Job {
            code,
            release_date: rel,
            due_date: due,
            processing_time: pt,
            size,
        }
    }
}
