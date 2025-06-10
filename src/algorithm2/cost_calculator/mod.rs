pub mod create_before;
pub mod create_after;
pub mod insert_in;
pub mod end_cases;

pub use create_before::find_cost_creating_before;
pub use create_after::find_cost_creating_after;
pub use insert_in::find_cost_inserting_in_batch;
pub use end_cases::make_end_decision;
