# Day 2 - Variables and Constants

## Summary
On Day 2, I explored **variables** and **constants** in Rust, learning how to declare and use them effectively. Understanding these concepts is fundamental for writing Rust programs.

## What I Learned
1. **Variables**:Sure! Here are the `README.md` files for Day 1 and Day 2, separated accordingly.


   - Variables are immutable by default, meaning once assigned a value, it cannot be changed.
   - Used `mut` keyword to make a variable mutable.
   - Example: 
   ```rust
   let age = 30; // Immutable
   let mut height = 180; // Mutable
   height = 185; // Now height can be changed
   ```

2. **Constants**:
   - Constants are always immutable and must have a type explicitly defined.
   - Declared using `const` keyword.
   - Example: 
   ```rust
   const PI: f64 = 3.14159; // Constant
   ```

## Code for Today
Check out the code examples in [`day02/main.rs`](./main.rs).

## Reflections
Learning about variables and constants highlighted the importance of immutability in Rust, which promotes safety and reduces bugs. I'm starting to appreciate the language's unique approach!

## Resources
- [The Rust Programming Language Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Video on Rust Variables and Mutability](https://www.youtube.com/watch?v=J3fv1-1SgI4&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=4)

---

**Next Goal**: Dive into **basic data types** to understand how to work with different kinds of data in Rust!