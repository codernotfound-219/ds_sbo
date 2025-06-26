# Deviation Calculator Module

This module handles the calculation of deviations for job insertions and batch creations in the scheduling algorithm.

## Structure

### Core Modules

#### `common/`
Contains shared utilities and types used across all deviation calculator modules.

- **`types.rs`**: Common type definitions and utility types
  - `CompletionUpdate`: Type alias for completion time updates
  - `PossibilitySet`: Type alias for insertion possibilities
  - `InsertionResult`: Result struct for basic insertion operations
  - `decisions`: Helper functions for creating Decision types

- **`utils.rs`**: Shared utility functions
  - `size_check()`: Check if a job can fit in a batch
  - `calculate_deviation()`: Calculate cascading deviations for a batch
  - `compute_current_deviation()`: Calculates deviation due to insertion of cur_job after popping lp_job

#### `batch_effects/`
Handles cascading effects when batch completion times change.

- **`cascading.rs`**: Cascading completion time calculations
  - `calculate_cascading_completion()`: Handle ripple effects of batch modifications
  - `calculate_last_batch_completion()`: Get the completion time of the last batch after virtual insertion of cur_job

#### `insertion/`
Main insertion logic with clean separation of concerns.

- **`core.rs`**: Main insertion interface and orchestration
  - `insert_in()`: Public API for job insertion
  - `insert_in_helper()`: Core insertion logic

- **`displacement.rs`**: Displacement handling logic
  - `handle_displacement_due_to_cur_job()`: Main displacement processing
  - `handle_displacement_due_to_lp_job()`: Scenario-specific displacement handling

- **`possibilities.rs`**: Possibility generation and selection
  - `get_active_logs_for_lp_job()`: Generate all placement options
  - `find_best_possibility()`: Select optimal placement from options

- **`calculations.rs`**: Deviation calculations
  - `calculate_deviation_for_direct_insertion()`: Direct insertion calculations

#### `creation/`
Handles creation of new batches.

- **`creation.rs`**: Batch creation logic
  - `create_in()`: Create batch at specific position
  - `create_end()`: Create batch at the end
