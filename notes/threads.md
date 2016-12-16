# Threads

What are threads? They are the units that the operating system can schedule processing time for your processes.

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

The problem was that the error message was unclear.
It said that the thread must outlive lifetime `'static`.
Solving this error was hard because we needed to // discover that the variabl all variables
//It needed to indicate that the shared variables could not be guarenteed to outlive -> ...

Error message:
```
C:\git\CPD\IbanCalculator>cargo build --release
   Compiling iban_calculator v0.1.0 (file:///C:/git/CPD/IbanCalculator)
error[E0477]: the type `[closure@src\modes\threads.rs:30:40: 38:14 settings:&settings::Settings, delta:u32, i:u32]` does not fulfill the required lifetime
  --> src\modes\threads.rs:30:26
30 |             threads.push(thread::spawn(move || {
   |                          ^^^^^^^^^^^^^
   = note: type must outlive the static lifetime
```

Error code: (commit hash 6ea8aa8a6a13571cab100ec4032307bea16f1e7a)
```rust
threads.push(thread::spawn(move || {
    let mut count = 0;
    for x in settings.top + delta * i..settings.top + delta * (1+i) {
        if util::m_proef(x, settings.modulo) {
            count += 1;
        }
    }
    count
}));
```

Correct code: (commit hash 3b43b9b55bc71fd898fca52ffb8a279d384eefa4)
```rust
let range = settings.top + delta * i..settings.top + delta * (1+i);
let modulo = settings.modulo;
threads.push(thread::spawn(move || {
    let mut count = 0;
    for x in range {
        if util::m_proef(x, modulo) {
            count += 1;
        }
    }
    count
}));
````

In order to make the code compile we need to copy the data required by the different threads.
These copies of the data are moved into the scope of the thread.

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

Error code: (commit hash ea108c0386f401de356d744fbf4edd0cd4ccbf5c)
```rust
let (send, recv) = mpsc::channel();
for range in split_ranges(settings.bottom, settings.top, settings.threads) {
    // Spin up another thread
    let modulo = settings.modulo;
    let send = send.clone();
    threads.push(thread::spawn(move || {
        for x in range {
            if util::m_proef(x, modulo) {
                send.send(x).unwrap();
            }
        };
        drop(send);
    }));
}
let mut counter = 1;
for x in recv {
   println!("{} {}", counter, x);
   counter += 1;
}
```

Correct code: (commit hash e94b9eb91bd7112b5fd72b334e9304f48540ea46)
```rust
let (send, recv) = mpsc::channel();
for range in split_ranges(settings.bottom, settings.top, settings.threads) {
    // Spin up another thread
    let modulo = settings.modulo;
    let send = send.clone();
    threads.push(thread::spawn(move || {
        for x in range {
            if util::m_proef(x, modulo) {
                send.send(x).unwrap();
            }
        };
        drop(send);
    }));
}
drop(send);
let mut counter = 1;
for x in recv {
   println!("{} {}", counter, x);
   counter += 1;
}
```

# Search

Search needs the `hash` variable on all different threads.
To accomplish memory safety accross thread boundaries,
with the certainty that the memory gets freed after it is no longer referenced,
we chose to encapsulate the hash in a atomic reference counter (`Arc`).

At first we just tried to access the hash from the different threads.
This gave errors about hash being moved into the closure.
After we placed a atomic reference counter around the hash this problem was solved.

Error message:
```
error[E0382]: capture of moved value: `hash`
   --> src\backend\threads.rs:103:73
101 |             threads.push(thread::spawn(move || {
    |                                        ------- value moved (into closure) here
102 |                 for x in range {
103 |                     if util::m_proef(x, modulo) && util::valid_hash(x, &hash) {
    |                                                                         ^^^^ value captured here after move
    = note: move occurs because `hash` has type `Box<[u8; 20]>`, which does not implement the `Copy` trait
```

Error code: (commit hash 1b672ffe34cf60f68ee68a753faa3ebd1551c622)
```
for range in split_ranges(settings.bottom, settings.top, settings.threads) {
    // Spin up another thread
    let modulo = settings.modulo;
    let send = send.clone();
    threads.push(thread::spawn(move || {
        for x in range {
            if util::m_proef(x, modulo) && util::valid_hash(x, &hash) {
                send.send(x).unwrap();
            }
        };
        drop(send);
    }));
}
```

Correct code: (commit hash 0917d8a9d0c5d9294ac6981a5e9515a74a743c73)
```
let hash = Arc::new(hash);
for range in split_ranges(settings.bottom, settings.top, settings.threads) {
    // Spin up another thread
    let modulo = settings.modulo;
    let send = send.clone();
    let hash = hash.clone();
    threads.push(thread::spawn(move || {
        for x in range {
            if util::m_proef(x, modulo) && util::valid_hash(x, &hash) {
                send.send(x).unwrap();
            }
        };
        drop(send);
    }));
}
```
