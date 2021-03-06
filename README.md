Get Started
===========

Download and install a recent Rust
I used a nightly from 4th July, shortly after 0.11. Other versions may or may not compile. Fun!

Syntax highlighting is available in all major editors and Emacs.
https://github.com/rust-lang/rust/wiki/Doc-packages,-editors,-and-other-tools#editors
https://github.com/rust-lang/rust/tree/master/src/etc

Clone this repository. Preferably a branch without the answers.

Docs, Examples
==========

* http://doc.rust-lang.org/index.html
* http://rustbyexample.com/

Rust for X
==========

* http://www.rustforrubyists.com/
* http://featherweightmusings.blogspot.co.uk/2014/04/rust-for-c-programmers-part-1-hello.html
* https://github.com/rust-lang/rust/wiki/Rust-for-CXX-programmers
* http://science.raphael.poss.name/rust-for-functional-programmers.html

Community
=========

http://www.reddit.com/r/rust


Some basic syntax
=================

Compile tests with
rustc --test src/main.rs
This will produce an executable called rust-dojo. Run it to run the tests.

Complete src/functions/mod.rs to make the tests pass! See comments in src/functions/tests.rs

Vague plan
==========

* Basic syntax, pattern matching, options
* Ownership, aliasing, lifetimes
* Containers
* Types: Structs, traits
* Concurrency
* Unsafe, FFI
* Attempt to use one of the half-finished libraries or frameworks. This will be fun.