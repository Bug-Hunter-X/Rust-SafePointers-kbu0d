# Unsafe Pointer Manipulation in Rust

This repository demonstrates a potential issue when directly manipulating memory using raw pointers in Rust.  The example shows how modifying a vector's elements through a raw pointer after certain operations on the vector can cause unexpected and undefined behavior.  The solution provides a safer alternative.

## Bug

The `bug.rs` file contains code that modifies a vector's element via its raw pointer.  This approach is unsafe because Rust's ownership and borrowing system is bypassed, leading to potential data corruption or panics if the vector's internal layout changes.

## Solution

The `bugSolution.rs` file presents a safer approach, leveraging Rust's standard library functions to modify the vector's elements without directly using raw pointers. This ensures memory safety and prevents unexpected behavior.