# Spin Lock

The `SpinLock` backend focuses on an easy to implement API.

Interesting:
* Even if we are sure that we got the spin lock right, the compiler is unable
to check it for us. Therefore, we need to use unsafe in order to get our program
to compile (see commit XXX).
* Only tested the lock with run_list, since the other functions didn't need one
* The caller must provide a closure that is executed between acquiring and releasing
the lock. This trades flexibility for an easier implementation.

# Advanced Spin Lock

The main difference with respect to `SpinLock` is that it hands out a reference
to the data, instead of requiring the user to pass a closure. This is more flexible.

Interesting:
* Based mainly on the `Mutex` type from Rust's standard library (open source FTW!)
* Main problem was a logical one: spinning endlessly (deadlock)
* More difficult for the programmer to ensure that all edge cases are covered.
* TTaS implemented used to acquire the lock.

# General considerations

The fact that we are using unsafe means that there could be hidden errors not
catched by the compiler. Potentially, our program could cause undefined behavior.

In case of `SpinLock`, undefined behavior would happen whenever we fail to satisfy
the guarantee that the critical region can only be accessed by one thread at a time.

In case of `AdvancedSpinLock`, the critical region begins after taking the lock and
ends when the `SpinLockGuard` is destroyed. Therefore, we must ensure that at all times,
there is at most one `SpinLockGuard`. Failure to do so would mean that the data is mutably
aliased.

Note that the lifetime system ensures that the `AdvancedSpinLock` lives at least as long
as any `SpinLockGuard`. This means that we can safely use the references stored in a
`SpinLockGuard` without fear for dangling pointers.
