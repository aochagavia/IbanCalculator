# Rayon

What is Rayon?

# Count

The count task is a perfect match for rayon. By combining `into_par_iter`,
`filter` and `count` we get concurrency for free. No room for errors here.

```rust
let modulo = settings.modulo;
let range = (settings.bottom .. settings.top).into_par_iter();
range.filter(|&x| util::m_proef(x, modulo)).count() as u32
```

# List

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

The search task is a perfect match for rayon. By combining `into_par_iter` and
`find_any` we again get concurrency for free. No room for errors here.

How does find_any work?

```rust
(settings.bottom .. settings.top)
    .into_par_iter()
    .find_any(|&x| util::m_proef(x, settings.modulo)
                && util::valid_hash(x, &hash))
```