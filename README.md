# ds_sbo_rust

A Rust library and binary implementing batch scheduling algorithms to minimize total tardiness.  
It provides two scheduling algorithms (`greedy_dp` and `marb_heuristic`), core data structures, and comprehensive test utilities.

## Research Context

This project implements two distinct approaches to batch scheduling:

- **Greedy Dynamic Programming (`greedy_dp`)**: The main research focus and novel contribution of this work, implementing a deviation-based approach for optimal batch scheduling.

- **MARB Heuristic (`marb_heuristic`)**: Implementation based on the Modified Attribute Ratio Based heuristic from Pujara Dhaval's Ph.D. thesis "Development of efficient scheduling algorithms for dynamic real time scheduling of burn-in oven in semiconductor manufacturing" (September 2023).

## Project Structure

```text
ds_sbo_rust/
├── Cargo.toml
├── README.md                    ← this file
├── logs/                        ← algorithm execution logs
├── src/
│   ├── main.rs                  ← main binary: configurable solver runner
│   ├── lib.rs                   ← library entry point with CLI configuration
│   ├── structures/              ← core scheduling model (Job, Batch, BatchSchedule)
│   ├── greedy_dp/               ← greedy dynamic programming algorithm
│   │   ├── deviation_calculator/ ← insertion and creation deviation calculations
│   │   │   ├── batch_effects/   ← cascading completion time calculations
│   │   │   ├── common/          ← shared utilities and types
│   │   │   ├── creation/        ← new batch creation logic
│   │   │   └── insertion/       ← job insertion logic
│   │   ├── execute.rs           ← executes actions with maximum deviation
│   │   ├── helper.rs            ← helper functions for decision making
│   │   ├── solve.rs             ← main solver orchestration
│   │   └── structures.rs        ← algorithm-specific data structures
│   ├── marb_heuristic/          ← MARB (Modified Attribute Ratio Based) heuristic
│   │   ├── solve.rs             ← main MARB solver implementation
│   │   └── structure.rs         ← MARB-specific batch structure
│   ├── tardiness_calculator/    ← tardiness computation utilities
│   └── resources/               ← job definitions for problem instances
│       ├── problem1.rs          ← 10-job problem instance
│       ├── problem2.rs          ← 15-job problem instance
│       ├── problem3.rs          ← 25-job problem instance
│       └── variables.rs         ← global constants (BATCH_CAPACITY)
└── tests/                       ← comprehensive unit and integration tests
```

## Modules

### structures

Defines the basic data structures:

- `Job` – release/due dates, processing time, size  
- `Batch` – groups jobs, computes release, processing, due metrics  
- `BatchSchedule` – sequence of batches with rolling completion times  

### greedy_dp

A deviation-based approach that uses dynamic programming concepts:

- `deviation_calculator/` – comprehensive deviation calculations for insertion and creation scenarios
  - `insertion/` – handles job insertion into existing batches with displacement logic
  - `creation/` – manages new batch creation at various positions
  - `batch_effects/` – calculates cascading effects on batch completion times
  - `common/` – shared utilities and type definitions
- `execute` – executes the action corresponding to maximum deviation
- `helper` – utility functions for action selection and decision making
- `solve` – orchestrates the complete solving process
- `structures` – algorithm-specific data structures (Decision, ActiveLog, LogHistory)

### marb_heuristic

Modified Attribute Ratio Based heuristic algorithm:

- `solve` – main MARB solver with attribute ratio calculations
- `structure` – specialized MarbBatch structure for the heuristic

### tardiness_calculator

Utilities for computing total tardiness of schedules:

- `calculator` – main tardiness computation functions
- `structure` – data structures for tardiness calculations

### resources

Factory functions and problem definitions:

- `problem1.rs` – 10-job problem instance (`job1()` to `job10()`)
- `problem2.rs` – 15-job problem instance (`job1()` to `job15()`)
- `problem3.rs` – 25-job problem instance (`job1()` to `job25()`)
- `variables.rs` – global constants like `BATCH_CAPACITY`

## Getting Started

### Prerequisites

First, install Rust if you haven't already:

```bash
# Install Rust using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions, then restart your terminal or run:
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

For more installation options and details, visit [rustup.rs](https://rustup.rs/).

### Building and Running

Build the library and run tests:

```bash
cd ds_sbo_rust
cargo build
cargo test
```

Run the binary with specific solver and problem:

```bash
# Run MARB heuristic on problem 1
cargo run marb problem1

# Run Greedy DP on problem 3
cargo run gdp problem3

# Run with verbose output
cargo run marb problem2 -v
```

### Command Line Usage

The binary accepts the following arguments:

```text
ds_sbo_rust <SOLVER> <PROBLEM> [FLAG]
```

- `SOLVER`: `marb` or `gdp`
- `PROBLEM`: `problem1`, `problem2`, or `problem3`
- `FLAG` (optional): `-v` for verbose output (shows the complete schedule)
