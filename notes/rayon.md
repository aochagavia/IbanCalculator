# Count

# List

First attempt: atomic

Problem, need to add a lock so results are printed in-order. Rust can of course
not guarantee a sequential order. `println` has its own lock.

Second attempt: Mutex

# Search

Problem, early return.

