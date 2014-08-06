Ownership, aliasing, lifetimes
==============================

Rust's USP is a combination of ML level safety and C level runtime overhead.

This is enabled by determining the lifetime of resources (heap memory, file handles etc) statically, which is made possible by controlling ownership and aliasing of variables.

This dojo:
======

Each task is in a separate sub directory. Existing code will not compile. Get it to compile. There are no tests.

Outline
=====
* Take a ref for a func that needs it
* Vars cannot be accessed while mutably borrowed
* Box recursive type
* Moving ownership of box
* Passing into a function moves ownership
* Destructors

Pointers not in this exercise
=========
* Reference counted Rc<T>
* Atomic reference counted Arc<T>
* Garbage collected Gc<T>
* Raw