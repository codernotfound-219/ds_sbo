GREEDY-BASED JOB SCHEDULING ALGORITHM FOR SINGLE BURN-IN OVEN

OBJECTIVE: Minimize tardiness in single-machine scheduling

================================================================================

ALGORITHM OVERVIEW

This algorithm schedules jobs for a single burn-in oven by organizing them into 
batches to minimize overall tardiness. The algorithm uses a greedy approach to 
make locally optimal decisions at each step.

================================================================================

TERMINOLOGY:

• Set V: Collection of unprocessed jobs, sorted by release date
• Set W: Collection of processed job batches
• Cost Function: Measures the benefit of inserting a job at a specific position
• Greedy Strategy: Always choose the insertion option with maximum immediate benefit

================================================================================

ALGORITHM STEPS

STEP 1: Initialize Job Queue
Sort all unprocessed jobs by their release date (earliest first) and store them 
in set V (unprocessed jobs).

STEP 2: Create Initial Batch
Move the first job from set V into set W (processed batches). This creates the 
first batch.

STEP 3: Process Remaining Jobs
For each remaining job in set V:

   1. Pop the next job from set V and call it 'new_job'

   2. Find candidate batch: Locate a batch in set W whose due_date is greater 
      than or equal to the due_date of new_job. Call this batch 'cur_batch'.

   3. Calculate insertion cost using the formula:
      cost = batch.due_date - batch.completion_time

   4. Evaluate 5 possible operations and calculate their respective costs:
      • Option I: Insert new_job at the end of the previous batch (prev_batch)
      • Option II: Create a new batch before cur_batch and place new_job inside
      • Option III: Insert new_job directly into cur_batch
      • Option IV: Create a new batch after cur_batch and place new_job inside
      • Option V: Insert new_job at the beginning of the next batch (next_batch)

   5. Select optimal operation: Choose and execute the operation that yields the 
      maximum cost (greedy selection criterion)

STEP 4: Termination
Repeat Step 3 until set V is empty (all jobs have been processed and assigned 
to batches).

================================================================================

LIMITATIONS AND CONSIDERATIONS

1. Greedy Nature: The algorithm makes locally optimal decisions that may not lead 
   to the globally optimal solution, as it cannot anticipate the impact of future 
   job assignments.

================================================================================