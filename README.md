# Rust Starter Project

A comprehensive Rust learning project that covers fundamental concepts through practical examples. This project serves as a hands-on introduction to Rust programming language concepts.

## 🚀 Project Overview

This project contains multiple binary examples demonstrating core Rust concepts:

- **Variables and Mutability** - Understanding variable declaration, mutability, and type inference
- **Functions** - Function definitions, parameters, and return values
- **Compound Types** - Working with tuples and arrays
- **Scaler Types** - Exploring different data types in Rust
- **Print Operations** - Various ways to print and format output

## 📁 Project Structure

```
rust_starter/
├── Cargo.toml
├── Cargo.lock
└── src/
    └── bin/
        ├── variables.rs    # Variable declarations and mutability
        ├── functions.rs    # Function definitions and calls
        ├── compound.rs     # Tuples and arrays
        ├── scaler.rs       # Scaler data types
        └── print.rs        # Print operations and formatting
```

## 🛠️ Prerequisites

- Rust installed on your system
- Cargo (comes with Rust installation)

To install Rust, visit [https://rustup.rs/](https://rustup.rs/) or run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🏃‍♂️ Running the Examples

Each example can be run individually using Cargo. Here's how to run each binary:

### Variables Example
```bash
cargo run --bin variables
```
**What you'll learn:**
- Immutable vs mutable variables
- Type annotations
- Constants
- Variable shadowing
- Type inference

### Functions Example
```bash
cargo run --bin functions
```
**What you'll learn:**
- Function definitions and calls
- Parameters and arguments
- Return values
- Expression vs statements
- Block expressions

### Compound Types Example
```bash
cargo run --bin compound
```
**What you'll learn:**
- Tuple creation and destructuring
- Array manipulation
- Index access
- User input handling
- Error handling with `expect()`

### Scaler Types Example
```bash
cargo run --bin scaler
```
**What you'll learn:**
- Integer types (signed and unsigned)
- Floating-point types
- Boolean type
- Character type
- Type conversions

### Print Operations Example
```bash
cargo run --bin print
```
**What you'll learn:**
- `println!` macro usage
- String formatting
- Debug formatting with `{:?}`
- Variable interpolation

## 📚 Learning Resources

This project is based on the Rust Book and various tutorials. The examples include:

- **YouTube Tutorial**: [Rust Programming Tutorial](https://www.youtube.com/watch?v=b4GbXzB7PYo)
- **Official Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

## 🔧 Dependencies

The project uses minimal dependencies:
- `io = "0.0.2"` - For input/output operations

## 🎯 Key Concepts Covered

### Variables and Mutability
- Understanding the difference between `let` and `let mut`
- Type annotations and type inference
- Constants with `const`
- Variable shadowing

### Functions
- Function syntax and parameters
- Return values and expressions
- The difference between expressions and statements
- Block expressions

### Compound Types
- Tuples: fixed-size collections of different types
- Arrays: fixed-size collections of the same type
- Pattern matching and destructuring
- Index access and bounds checking

### Data Types
- Scaler types (integers, floats, booleans, characters)
- Type safety and conversions
- Memory representation

## 🚨 Important Notes

- All examples include detailed comments explaining Rust concepts
- Some examples demonstrate compile-time errors (commented out)
- The project uses Rust 2024 edition
- Examples are designed to be educational and demonstrate best practices

## 🤝 Contributing

Feel free to add more examples or improve existing ones! This is a learning project, so contributions that help others learn Rust are welcome.

## 📄 License

This project is open source and available under the MIT License.

---

**Happy Rusting! 🦀** 