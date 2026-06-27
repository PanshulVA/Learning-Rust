# CS4xx: Comparative Programming Languages
## Assignment 2: The Rust Paradigm Translation Guide

**Student Name:** [Your Name]  
**GitHub ID:** [@YourHandle]  
**Date:** June 24, 2026  
**Instructor:** [Tutor/Instructor Name]  

---

### 📋 Assignment Objectives
- [x] Create a comprehensive guide for transitioning from C/C++/Python to Rust.
- [x] Map existing mental models (GC, manual memory management, OOP) to Rust's unique paradigms.
- [x] Cover core concepts: Ownership, Borrowing, Structs, Enums, Error Handling, Traits, and **Lifetimes**.
- [x] Include a polyglot translation matrix.
- [x] Demonstrate understanding of Rust's safety guarantees in Concurrency.

---

## 1. High-Level Paradigm Shifts

Learning Rust isn't about learning syntax; it's about learning the **Borrow Checker**. Since I already understand pointers and stack vs. heap memory from C/C++, and high-level abstractions from Python, here is the translation map:

* **Memory Management**: Python uses a Garbage Collector (runtime overhead). C requires `malloc/free` (unsafe). C++ uses RAII and smart pointers. Rust enforces RAII strictly at *compile time* through **Ownership**. There is no GC, and manual `delete` is impossible.
* **Default Immutability**: Variables in C/C++/Python are mutable by default. In Rust, they are **immutable** by default to prevent unintended side effects.
* **No `null` or `nullptr`**: Rust eliminates the "billion-dollar mistake" entirely. It uses an `Option` Enum to handle the explicit absence of a value.
* **No Exceptions (`try/catch`)**: Instead of throwing invisible exceptions that jump up the call stack (C++/Python), Rust returns `Result` enums. You *must* explicitly handle success and failure paths.
* **Composition over Inheritance**: Rust has no classes or inheritance hierarchies. It uses `struct` for data layout and `trait` for shared interfaces (similar to C++ Concepts or Java Interfaces).

## 2. Variables and Mutability

Variables are statically typed, but Rust has strong type inference (giving it a Python-like feel).

```rust
// IMMUTABLE by default (Unlike C/C++/Python)
let x = 5; 
// x = 6; // ERROR: cannot assign twice to immutable variable

// MUTABLE (Explicit opt-in)
let mut y = 10;
y = 15; // OK

// SHADOWING (Allows reusing variable names, common in Rust)
let z = 5;
let z = z + 1; // Creates a new 'z', shadowing the old one

// CONSTANTS (Evaluated at compile time, requires explicit type)
const MAX_POINTS: u32 = 100_000;
```

## 3. Control Flow

Control flow is similar to C/C++, but `if` and `match` are **expressions** (they return values), similar to Python's `a if cond else b`.

```rust
let condition = true;

// No parentheses needed around conditions. 
// Can be used as an expression (like C++ ternary ?:)
let number = if condition { 5 } else { 6 }; 

// Loops
loop { break; } // Infinite loop
while condition { break; }
for i in 0..5 { } // Ranges: 0 to 4 (like Python's range(5))
```

## 4. The Big Filter: Ownership

This replaces Python's GC and C's `free()`. It manages heap data.

**The 3 Rules of Ownership:**
1. Each value has a variable that’s called its **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes out of scope, the value will be **dropped** (memory freed).

```rust
{
    let s1 = String::from("hello"); // s1 owns the String on the heap
    let s2 = s1; // OWNERSHIP MOVED to s2. 
                 // Under the hood, the pointer, length, and capacity are copied.

    // println!("{}", s1); // COMPILE ERROR! s1's value was moved.
} // s2 goes out of scope, memory is automatically freed.
```
*C++ Analogy: This is essentially `std::unique_ptr` enforced as a fundamental language rule.*

## 5. Borrowing and References (`&`)

Moving ownership everywhere is tedious. Instead, we can **borrow** values using references.

**The Borrowing Rules (The Reader-Writer Lock):**
At any given time, you can have *either*:
* **Multiple immutable references** (`&T`) -> Read-only access.
* **Exactly ONE mutable reference** (`&mut T`) -> Read-Write access.

```rust
let mut s = String::from("hello");

let r1 = &s; // Immutable borrow
let r2 = &s; // Immutable borrow
// let r3 = &mut s; // ERROR! Cannot borrow as mutable while immutable borrows exist.

println!("{} {}", r1, r2); // r1 and r2's scope implicitly ends here (Non-Lexical Lifetimes).

let r3 = &mut s; // NOW it's OK because r1 and r2 are done.
r3.push_str(", world");
```
*Contrast:* C++ allows multiple mutable pointers to the same data, causing data races. Rust makes data races a compile-time error.

## 6. Lifetimes (The Borrow Checker's Assistant)

In C++, returning a reference to a local variable causes a dangling pointer (segfault). Rust's compiler stops this using **Lifetimes**—annotations that tell the compiler how long references live.

```rust
// 'a specifies that the return value lives exactly as long as 
// the shortest-lived argument passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
*Most of the time, Rust infers lifetimes automatically (Lifetime Elision).*

## 7. Structs, Collections, and Generics

Rust separates data (`struct`) from logic (`impl`). 

```rust
// A generic struct (Like C++ templates)
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    // &self is like C++ 'this' or Python 'self'.
    fn width(&self) -> &T {
        &self.width
    }
}
```

**Common Collections mappings:**
* Python `list` / C++ `std::vector` -> Rust `Vec<T>`
* Python `dict` / C++ `std::unordered_map` -> Rust `HashMap<K, V>`
* Python `str` / C++ `std::string` -> Rust `String` (heap allocated) vs `&str` (string slice / view)

## 8. Enums and Pattern Matching

Rust Enums are Algebraic Data Types (like C++ `std::variant`).

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

// match is an exhaustive switch statement
match home {
    IpAddr::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
    IpAddr::V6(s) => println!("IPv6: {}", s),
}
```

### The `Option` Enum (No `NULL`)
```rust
enum Option<T> { Some(T), None }
```
You *must* handle the `None` case, usually with `match` or `if let`.

## 9. Error Handling

Recoverable errors use the `Result<T, E>` enum.

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

**The `?` Operator:** Propagates errors up the stack, acting like an explicit `throw`.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    // If open fails, ? returns the Err immediately. Otherwise, extracts the file.
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

## 10. Traits (Polymorphism)

Traits are interfaces defining shared behavior.

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
}

// "Implement the Summary interface for the Tweet struct"
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
* Python relies on "Duck Typing" (if it walks like a duck...). 
* C++ uses Virtual Base Classes.
* Rust achieves this safely at compile-time using Traits and Monomorphization (zero-cost abstractions).

## 11. Concurrency (Fearless)

Because of Ownership and Borrowing, Rust guarantees thread safety. If a thread owns data, no other thread can mutate it. If a thread shares data, it must be locked (e.g., `Mutex`).

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Arc = Atomic Reference Counted (like C++ std::shared_ptr)
// Mutex ensures single-thread access
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        // Must lock to access data. Mutex frees automatically on scope exit!
        let mut num = counter_clone.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}
```
*If you forget to lock the Mutex, Rust will refuse to compile.*

---

## 📚 Appendix: Translation Matrix

| Concept | C | C++ | Python | Rust | 
| :--- | :--- | :--- | :--- | :--- | 
| **Build System** | Make | CMake / Conan | pip / poetry | `cargo` (built-in) | 
| **Null/None** | `NULL` | `nullptr` | `None` | `Option::None` | 
| **Exceptions** | Error codes | `throw` / `catch` | `raise` / `except` | `Result::Err` and `?` | 
| **Memory Mgt** | `malloc` / `free` | `new`/`delete`/RAII | Garbage Collector | Ownership / Borrow Checker | 
| **Polymorphism** | Function pointers | Virtual Inheritance | Duck Typing | Traits | 
| **Strings** | `char*` | `std::string` | `str` | `String` (owned) / `&str` (borrowed) | 
| **Thread Safety**| Manual | `std::mutex` / Manual | GIL (Global Int. Lock) | `Send` and `Sync` Traits |

---
**Honor Code Declaration:** *I certify that this assignment represents my own understanding of the Rust programming language as translated from my prior knowledge of C, C++, and Python.*