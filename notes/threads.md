# Threads

What are threads? They are the units that the operating system can schedule processing time for your processesses.

# Split range

There is no partition defined on ranges that is generic in number of desired chunks.
To better reuse our code we implemented a function that returns an Iterator that returns Iterators.

```rust
fn split_ranges(low: u32, high: u32, chunks: u32) -> impl Iterator<Item=impl Iterator<Item=u32> + Debug> {
    let delta = (high - low) / chunks; // Size of the individual chunks
    (0..chunks).map(move |i| { // For each desired chunk
        if i == chunks - 1 {
            low + delta * i..high // Last chunk also gobbles up all remaining elements that couldn't be distributed equally
        } else {
            low + delta * i..low + delta * (1+i) // Chunks with the size of delta
        }
    })
}
```

# Count

In order to make the speed up the greatest it is important to equally distribute the work accross the threads.
The division of the work is handled by the `split_ranges` function.
Thread handles are stored in a `Vec` this is needed to later `join` these threads.
On joining the threads there results are added and the accumelator is returnded at the of the scope.

The individual threads execute the `m_proef` on each individual element in their range.
Adding `1` to their local count if the m_proef has a positive result.

problems -> ...


# List

The main problem with listing the numbers with the ever incrementing global counter.
To make the solutions more diversive we chose to use a `channel`.
With a channel the work can be continued while the main thread writes the answers to the console.
The main upside is that a channel works asynchronous, while writing to the console would need a lock.
This could result in a more scalable solution.

One problem was encountered that wasn't picked up as an error while implementing a channel based solution.
When main thread didn't drop it's `send channel` and entered the loop on the `recv channel`.
The loop on the recieving end would only close when all writeble channels are dropped.
This resulted in an endless loop.

The list task is not completely a good match for rayon, since we are required
to print counter in sequential order. That is, the output needs to be ...

First attempt: use an atomic integer (see commit XXX). Note: we forgot to commit
when needed, but

However, using an atomic integer does not produce correct results. The problem
with this approach is that `println!` has an own lock, to ensure writes to
`stdout` are synchronized. Therefore, while the atomic integer is sequentially
implemented, the calls to `println!` have a race condition in which the first
thread to get the lock will print, regardless of the count.

In order to solve this problem, we remove the atomic integer and use a mutually
exclusive lock (`Mutex`). This way we can ensure that both the increment and
the write to `stdout` are done as a single transaction.

As you can see, while Rust is able to forbid concurrent writes to the same
variables, it is unable to forbid race conditions in a general sense. The
same holds for deadlocks, for instance.

# Search

Search needs the hash on all different threads.
To accomplish memory safety accross thread boundaries,
with the certainty that the memory gets freed after it is no longer referenced,
we chose to encapsulate the hash in a atomic reference counter (`Arc`).

problems -> errors about hash when no Arc is used
