pub struct Lateness {
    pub lateness: i32,
    pub job_code: u32,
}

impl Lateness {
    pub fn new(late: i32, code: u32) -> Self {
        Lateness {
            lateness: late,
            job_code: code,
        }
    }
}
