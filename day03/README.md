# Day 3 - Basic Data Types in Rust

## Summary
Today, I focused on learning about **basic data types** in Rust. Understanding these foundational building blocks is crucial for writing safe and efficient Rust programs.

## What I Learned
1. **Integers** (`i32`, `u32`, etc.):
   - Example: `let age: i32 = 25;`
   - Rust has several integer types, each with a specific size (`i8`, `i16`, `i32`, `i64`, etc.) and signedness (`i` for signed, `u` for unsigned).

2. **Floats** (`f32`, `f64`):
   - Example: `let price: f64 = 10.99;`
   - Used for decimal values. `f64` is the default for floating-point numbers.

3. **Booleans** (`bool`):
   - Example: `let is_student: bool = true;`
   - Represents either `true` or `false`.

4. **Characters** (`char`):
   - Example: `let grade: char = 'A';`
   - A character type that represents a single Unicode scalar value.

5. **Strings** (`String` and `&str`):
   - Example: `let name = String::from("Mart");`
   - `&str` is a string slice (immutable reference), whereas `String` is a growable, heap-allocated data type.

6. **Tuples**:
   - Example: `let person: (&str, i32) = ("Alice", 30);`
   - Tuples can group different types together and are useful for returning multiple values.

7. **Arrays**:
   - Example: `let numbers: [i32; 3] = [1, 2, 3];`
   - Fixed-size collections where all elements are of the same type.

8. **Vectors**:
   - Example: `let mut numbers = vec![1, 2, 3];`
   - Growable lists that are similar to arrays but can change in size.

## Code for Today
Check out the code examples in [`day03/main.rs`](./main.rs).

## Reflections
Learning about data types highlighted Rust's emphasis on **safety and performance**. I especially enjoyed how Rust's type system makes sure the code is more predictable and less prone to errors.

## Resources
- [The Rust Programming Language Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Video on Rust Data Types](https://www.youtube.com/watch?v=o9mCVvEuKGg&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=5)

