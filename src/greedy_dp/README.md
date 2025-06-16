# Greedy-Based Job Scheduling Algorithm for Single Burn-In Oven

**Objective:** Minimize tardiness in single-machine scheduling

## Algorithm Overview

This algorithm schedules jobs for a single burn-in oven by organizing them into batches to minimize overall tardiness. The algorithm uses a greedy approach to make locally optimal decisions at each step.

## Terminology:

- **Set V**: Collection of unprocessed jobs, sorted by release date
- **Set W**: Collection of processed job batches
- **Cost Function**: Measures the benefit of inserting a job at a specific position
- **Greedy Strategy**: Always choose the insertion option with maximum immediate benefit

## Algorithm Steps

### Step 1: Initialize Job Queue
Sort all unprocessed jobs by their **release date** (earliest first) and store them in set **V** (unprocessed jobs).

### Step 2: Create Initial Batch
Move the first job from set **V** into set **W** (processed batches). This creates the first batch.

### Step 3: Process Remaining Jobs
For each remaining job in set **V**:

1. **Pop the next job** from set **V** and call it `new_job`

2. **Find candidate batch**: Locate a batch in set **W** whose `due_date` is greater than or equal to the `due_date` of `new_job`. Call this batch `cur_batch`.

3. **Calculate insertion cost** using the formula:
   ```
   cost = batch.due_date - batch.completion_time
   ```

4. **Evaluate 5 possible operations** and calculate their respective costs:
   - **Option I**: Insert `new_job` at the end of the previous batch (`prev_batch`)
   - **Option II**: Create a new batch before `cur_batch` and place `new_job` inside
   - **Option III**: Insert `new_job` directly into `cur_batch`
   - **Option IV**: Create a new batch after `cur_batch` and place `new_job` inside
   - **Option V**: Insert `new_job` at the beginning of the next batch (`next_batch`)

5. **Select optimal operation**: Choose and execute the operation that yields the **maximum cost** (greedy selection criterion)

### Step 4: Termination
Repeat **Step 3** until set **V** is empty (all jobs have been processed and assigned to batches).


## Algorithm Complexity
- **Time Complexity**: O(n²) where n is the number of jobs
- **Space Complexity**: O(n) for storing jobs and batches
