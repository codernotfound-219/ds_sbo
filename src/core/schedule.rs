use crate::core::batch::Batch;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct BatchSchedule {
    pub batches: Vec<Batch>,
}

impl BatchSchedule {
    pub fn new() -> Self {
        BatchSchedule {
            batches: Vec::new(),
        }
    }

    pub fn get_batches(&self) -> &Vec<Batch> {
        &self.batches
    }

    pub fn is_empty(&self) -> bool {
        self.batches.is_empty()
    }

    pub fn insert_begin(&mut self, batch: Batch) {
        self.batches.insert(0, batch);
        self.update_parameters(0);
    }

    pub fn insert_end(&mut self, batch: Batch) {
        self.batches.push(batch);
        let batch_index = self.batches.len() - 1;
        self.update_parameters(batch_index);
    }

    pub fn insert_at_position(&mut self, index: usize, batch: Batch) {
        self.batches.insert(index, batch);
        self.update_parameters(index);
    }

    fn update_parameters(&mut self, index: usize) {
        let (mut prev_completion, mut prev_code) = if index == 0 {
            (0, 0)
        } else {
            (self.batches[index - 1].completion_time, self.batches[index-1].code)
        };


        for batch in &mut self.batches[index..] {
            batch.release_date = batch.release_date.max(prev_completion);
            batch.completion_time = batch.release_date + batch.processing_time;
            batch.code = prev_code + 1;

            prev_completion = batch.completion_time;
            prev_code += 1;
        }
    }
}

impl Default for BatchSchedule {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BatchSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BatchSchedule:")?;
        for batch in &self.batches {
            writeln!(f, "    Batch_Code: {}", batch.code)?;
            write!(f, "    Batch_Jobs: ")?;
            for (i, job) in batch.jobs.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", job.code)?;
            }
            writeln!(f)?;
            writeln!(f, "    Batch_release: {}", batch.release_date)?;
            writeln!(f, "    Batch_completion: {}", batch.completion_time)?;
            writeln!(f, "    Batch_min_due_date: {}", batch.min_due_time)?;
            writeln!(f, "    Batch_size: {}", batch.size)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
