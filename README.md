# ds_sbo_rust

A Rust library and binary implementing batching algorithms to schedule jobs for minimal lateness.  
It provides an algorithm module (`greedy_dp`), a core scheduling model, and test utilities.

## Project Structure

```
ds_sbo_rust/
├── Cargo.toml
├── README.md           ← this file
├── src/
│   ├── main.rs         ← main binary: prints schedule list
│   ├── lib.rs          ← library entry point
│   ├── core/           ← scheduling model (Job, Batch, BatchSchedule)
│   ├── greedy_dp/      ← deviation-driven batching procedure
│   └── resources/      ← job definitions for problem instances
└── tests/              ← unit tests for each decision and deviation functions
```

## Modules

### core  
Defines the basic data structures:
- `Job` – release/due dates, processing time, size  
- `Batch` – groups jobs, computes release, processing, due metrics  
- `BatchSchedule` – sequence of batches with rolling completion times  

### greedy_dp  
A cost-based approach:
- `deviation_calculator/` – finds deviation for all possible cases of insertion / creation
- `execute` – executes the action corresponding to maximum deviation
- `solve` – (stub) orchestrates repeated decisions across the schedule  

### resources  
Factory functions (`job1()`, …, `job10()`) for two problem instances: `problem1.rs` and `problem2.rs`.

## Getting Started

Build the library and run tests:

```bash
cd ds_sbo_rust
cargo build
cargo test
```

Run the example binary:

```bash
cargo run
```

## Usage

As a library, import and call the solver:

```rust
use ds_sbo_rust::core::Job;
use ds_sbo_rust::greedy_dp::solve;

let mut jobs = vec![
    Job::new(1, 0, 10, 3, 2),
    Job::new(2, 1, 12, 4, 1),
    // ...
];
let schedule = solve(&mut jobs);
for batch in schedule.get_batches() {
    println!("{}", batch);
}
```
