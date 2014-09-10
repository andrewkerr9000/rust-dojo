First things first
=====

You will need a recent nightly build (not 0.11) or stuff won't build. WELCOME TO RUST!

Threads
===

In the olden days Rust used green (userspace) threads running over multiple OS threads, similar to Go. Native (OS) threads were introduced later, and then became the default. Now green threads are being pushed out of the standard library, as they have various drawbacks and in the Rust memory model seem to have no advantages.

AIO
===

There is no non blocking or async IO on Rust. There may not be before 1.0. It is hard to get high-performance cross-platform abstractions over Windows and Linux/OSX AIO functionality.

Concurrency
=====

* Task
* Future
* try
* message passing
* shared memory
** arc
** mutex


https://www.youtube.com/watch?v=oAZ7F7bqT-o
http://people.mozilla.org/~acrichton/rust-talk-2014-08-27/
