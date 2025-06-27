# ds_sbo_rust

A Rust library and binary implementing batching algorithms to schedule jobs for minimal lateness.  
It provides an algorithm module (`greedy_dp`), a core scheduling model, and test utilities.

## Project Structure

```
ds_sbo_rust/
├── Cargo.toml
├── README.md           ← this file
├── src/
│   ├── main.rs         ← example binary: prints job list
│   ├── lib.rs          ← library entry point
│   ├── core/           ← scheduling model (Job, Batch, BatchSchedule, handlers)
│   ├── greedy_dp/      ← cost-driven batching (helper, cost_calculator, solve)
│   └── resources/      ← job definitions for problem instances
└── tests/              ← unit tests for each decision and cost function
```

## Modules

### core  
Defines the basic data structures:
- `Job` – release/due dates, processing time, size  
- `Batch` – groups jobs, computes release, processing, due metrics  
- `BatchSchedule` – sequence of batches with rolling completion times  
- `handlers` – logs and generic decision enums  

### greedy_dp  
A cost-based approach:
- `helper::locate_eligible_batch` – find candidate batch by due date  
- `cost_calculator` – compute slack/lateness for inserting, creating before/after, end cases  
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
