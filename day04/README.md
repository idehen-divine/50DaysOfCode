# Day 4 - Functions in Rust

## Summary
Today, I learned about **functions** in Rust. Functions are a great way to reuse code and make it more organized and easier to read. Rust encourages breaking code into smaller, reusable functions, which makes programs modular and easier to maintain.

## What I Learned
1. **Defining Functions**:
   - Functions in Rust are defined using the `fn` keyword.
   - Functions help organize code by creating reusable blocks of logic.

   Example:
   ```rust
   fn make_tea() {
       println!("Boil water");
       println!("Add tea bag");
       println!("Pour water into a cup");
       println!("Let it steep");
   }
   ```

2. **Calling Functions**:
   - Functions are called by using their name followed by parentheses.
   - The `main` function is where Rust begins execution, and other functions can be called from inside it.

   Example:
   ```rust
   fn main() {
       make_tea(); // Call the make_tea function
   }
   ```

## Code for Today
Check out the code examples in [`day04/main.rs`](./main.rs).

## Reflections
Learning about functions made me appreciate how Rust encourages writing clean, reusable code. Functions are a core concept in programming that helps maintain structure, especially as projects grow in size.

## Resources
- [The Rust Programming Language Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Video on Functions in Rust](https://www.youtube.com/watch?v=YnwiRiXu5gA&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN&index=15)

