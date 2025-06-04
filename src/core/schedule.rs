use crate::core::batch::Batch;

#[derive(Debug)]
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
        let mut prev_completion = if index == 0 {
            0
        } else {
            self.batches[index - 1].completion_time
        };

        for batch in &mut self.batches[index..] {
            batch.release_date = batch.release_date.max(prev_completion);
            batch.completion_time = batch.release_date + batch.processing_time;

            prev_completion = batch.completion_time;
        }
    }
}
