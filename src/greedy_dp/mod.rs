pub mod solve;
pub mod structures;
pub mod deviation_calculator;
pub mod helper;

pub use structures::Decision;
pub use structures::DecisionHistory;

// TESTS
pub use deviation_calculator::create_in;
pub use deviation_calculator::create_end;
