# rust-practice-2025

## Introduction

This repository is a personal learning journey through the Rust programming language. The primary goal is to build proficiency in Rust fundamentals, syntax, and best practices. A secondary objective is to explore integrating Rust with Python, enabling high-performance extensions and interoperability between the two languages.

The repository is organized by day/topic, with each directory containing practical examples and exercises that progressively build understanding of Rust concepts.

## Repository Structure

- `day1/` - Hello World and basic project setup
- `day2/` - Data types (primitive and compound)
- `day3/` - Functions
- _(Additional days will be added as learning progresses)_

## Learning Goals

- Master Rust fundamentals: ownership, borrowing, lifetimes
- Understand Rust's type system and memory safety guarantees
- Build proficiency with Rust's tooling (Cargo, rustc, rustfmt, clippy)
- Explore systems programming concepts
- Learn Rust-Python interoperability using PyO3 or similar tools

## Rustlings: Interactive Learning

[Rustlings](https://github.com/rust-lang/rustlings/) is a collection of small, focused exercises designed to teach Rust through hands-on practice. It's an official Rust project that helps you learn by reading, writing, and fixing Rust code. Small exercises to get you used to reading and writing Rust code - Recommended in parallel to reading the official Rust book. 📚️

### What is Rustlings?

Rustlings provides bite-sized exercises that start simple and gradually increase in complexity. Each exercise is a small Rust program with intentional errors or missing code that you need to fix. The exercises cover core Rust concepts including:

- Variables and mutability
- Functions and control flow
- Data types and structures
- Ownership, borrowing, and lifetimes
- Error handling
- Traits and generics
- Testing and much more

### How to Use Rustlings

1. **Installation**:
   ```bash
   curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
   ```
   
   Or with Cargo:
   ```bash
   cargo install rustlings
   ```

2. **Getting Started**:
   ```bash
   rustlings init
   cd rustlings
   rustlings watch
   ```

3. **Working Through Exercises**:
   - Run `rustlings watch` - it will automatically check your solutions as you save files
   - Open exercises in your editor (located in the `exercises/` directory)
   - Read the instructions and hints in each file
   - Fix the code to make the tests pass
   - Use `rustlings hint <exercise_name>` if you get stuck
   - Progress through exercises sequentially or jump to specific topics

4. **Key Commands**:
   - `rustlings watch` - Auto-check solutions as you work
   - `rustlings verify` - Check all exercises at once
   - `rustlings run <exercise_name>` - Run a specific exercise
   - `rustlings hint <exercise_name>` - Get a hint for an exercise
   - `rustlings list` - Show all exercises and their status

### Why Rustlings?

- **Learn by doing**: Active practice reinforces concepts better than passive reading
- **Immediate feedback**: The watch mode provides instant feedback on your solutions
- **Structured progression**: Exercises build on each other logically
- **Complements The Book**: Works perfectly alongside reading The Rust Programming Language
- **Low commitment**: Each exercise takes just a few minutes, perfect for quick learning sessions

## References

### Official Rust Resources

- **[The Rust Programming Language Book](https://doc.rust-lang.org/book/)** - The official book for learning Rust, also known as "The Book"
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - A collection of runnable examples illustrating various Rust concepts
- **[Rustlings](https://github.com/rust-lang/rustlings/)** - Small exercises to get you used to reading and writing Rust code
- **[Rust Standard Library Documentation](https://doc.rust-lang.org/std/)** - Comprehensive documentation for the standard library

### Additional Learning Resources

- **[The Cargo Book](https://doc.rust-lang.org/cargo/)** - Learn about Rust's package manager and build system
- **[Rust Language Cheat Sheet](https://cheats.rs/)** - Quick reference for Rust syntax and concepts
- **[Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)** - A collection of simple examples demonstrating good practices
- **[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)** - Guide to async/await in Rust

### Rust-Python Integration

- **[PyO3](https://pyo3.rs/)** - Rust bindings for Python, enabling Rust extensions for Python
- **[maturin](https://www.maturin.rs/)** - Build and publish Rust-based Python packages
- **[rust-cpython](https://github.com/dgrunwald/rust-cpython)** - Alternative Rust bindings for Python

### Community and Tools

- **[Rust Playground](https://play.rust-lang.org/)** - Try Rust in your browser
- **[This Week in Rust](https://this-week-in-rust.org/)** - Weekly newsletter about Rust updates
- **[Rust Users Forum](https://users.rust-lang.org/)** - Community discussion forum
- **[r/rust on Reddit](https://www.reddit.com/r/rust/)** - Rust community on Reddit

## Progress Tracking

| Day | Topic | Status |
|-----|-------|--------|
| Day 1 | Hello World, Project Setup | ✅ Complete |
| Day 2 | Data Types | ✅ Complete |
| Day 3 | Functions | ✅ Complete |

---

*Last updated: November 23, 2025*