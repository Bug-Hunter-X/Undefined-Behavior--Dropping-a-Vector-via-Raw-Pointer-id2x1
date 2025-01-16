# Rust Undefined Behavior Example

This repository demonstrates a common pitfall in Rust involving unsafe code and raw pointers.  Improper handling of raw pointers can lead to undefined behavior, crashes, and memory corruption. The `bug.rs` file shows the erroneous code, while `bugSolution.rs` offers a safer alternative.

**Key Learning Points:**
* Always be mindful when working with unsafe code in Rust.
* Raw pointers bypass Rust's memory safety mechanisms. Misuse can lead to significant problems. 
* Prefer using safe abstractions provided by Rust's standard library wherever possible.
* Understanding memory management in Rust is crucial for writing robust and reliable code.