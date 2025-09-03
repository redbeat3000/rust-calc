# Rust Calc 🦀➗

A simple command-line calculator written in Rust.  
It supports standard arithmetic operations, parentheses, exponents, and percentages with an interactive REPL.

---

## ✨ Features
- Arithmetic operations: `+`, `-`, `*`, `/`, `^`, `%`
- Parentheses for grouping: `( ... )`
- Unary minus (negative numbers)
- Percent operator (e.g., `50% = 0.5`)
- Correct operator precedence and associativity (via Shunting Yard algorithm)
- Built-in REPL commands:
  - `help` – show usage
  - `clear` – clear the screen
  - `quit` / `exit` – close the program

---

## 🚀 Usage

Run with Cargo:
```bash
cargo run
```
Or compile directly:
```
rustc src/main.rs -o calc
./calc
```
💻 Example
Rust Calculator REPL
Type expressions, e.g.: 2 + 3 * (4 - 1) ^ 2
Operators: + - * / ^ % (percent converts number to fraction, e.g. 50% -> 0.5)
Commands: quit, exit, help, clear
```
> 2 + 3 * 4
14
> (5 - 2) ^ 3
27
> -7 + 10
3
> 50%
0.5
```
📂 Project Structure
calculator_rs/
├── Cargo.toml       # Project configuration
└── src/
    └── main.rs      # Calculator source code
🛠️ Build & Run Tests
```
cargo build
cargo test

```
