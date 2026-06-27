# 🔥 RUST QUICK REFRESHER & GOTCHAS 🔥

*(Read this when coming back from C++/Python)*

## 1. THE BASICS

* **Entry Point:** `fn main() {}` (Just like `int main()` in C/C++)

* **Macros:** Anything with a `!` (like `println!`) is a macro, not a standard function. It generates code at compile time.

* **Comments:** `//` works exactly the same as C/C++.

## 2. VARIABLES: MUTABILITY vs. SHADOWING

* **Immutable by Default:** `let x = 5;` cannot be changed.

* **Mutability (`mut`):** `let mut x = 5;` -> Mutable, BUT can only be reassigned to the SAME data type (e.g., int to int).

* **Shadowing (`let` again):** `let x = 5; let x = "Hello";` -> The second `let` shadows (hides) the first. Allows you to reuse names and change data types entirely.

* **Block Encapsulation:** If you shadow a variable inside `{ }`, it temporarily hides the outer variable. When the block ends, the original outer variable comes back untouched!

## 3. SEMICOLONS MATTER (Expressions vs Statements)

*In C++, missing a semicolon is a syntax error. In Rust, it changes the behavior!*

* **WITH semicolon:** It's a statement. Returns nothing `()`.

* **WITHOUT semicolon:** It's an expression. It returns the value.
  *Example:* `fn add(a: i32, b: i32) -> i32 { a + b }` (Notice no semicolon! It implicitly returns `a+b`).

## 4. STRINGS: String vs &str

* **`&str` (String Slice):** Fixed size, immutable, often hardcoded literals (e.g., `"hello"`). It is just a borrowed view. (Like `const char*` in C++).

* **`String`:** Heap-allocated, dynamic, mutable. It is OWNED data. (Like `std::string` in C++).

## 5. ERROR HANDLING (No try/catch)

* Rust uses `Result<T, E>` (Ok or Err) and `Option<T>` (Some or None).

* **Avoid `.unwrap()`:** It crashes the program if there's an error. Use it only for quick prototyping.

* **The `?` Operator:** Put `?` at the end of a function call that might fail. If it fails, it immediately returns the error up the call stack (just like throwing an exception).

## 6. OWNERSHIP & BORROWING (The Borrow Checker)

* **1 Rule to rule them all:** Data has exactly ONE owner. When the owner goes out of scope, memory frees automatically (No `free()` or `delete`).

* To pass data without losing ownership, you **BORROW** it using `&`.

* **The Reader-Writer Lock Rule:**

  * \-> You can have **INFINITE** immutable references (`&T`) at once (Read-only).

  * \-> **OR** you can have **EXACTLY ONE** mutable reference (`&mut T`) (Read/Write).

  * \-> **NEVER BOTH AT THE SAME TIME.**

## 7. OOP TRANSLATION

* **Data** = `struct` (No classes).

* **Methods** = `impl` block.

* **Interfaces/Polymorphism** = `trait` (Instead of inheritance or Duck Typing).