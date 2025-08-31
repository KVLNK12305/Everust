````markdown
# Everust 🦀

My Rust learning journey — documenting progress, experiments, and discoveries as I master the Rust programming language.

## About This Repository 📚

This repo is my **daily Rust diary** — code snippets, core insights, and "aha!" moments as I move through *The Rust Programming Language (a.k.a The Book)*.  
I’m logging everything here: wins, roadblocks, and the cool quirks that make Rust unique.

---

# Chapter 1 - Getting Started 🚀

*"Every adventure begins with a single compile."*  

The first chapter was all about **setting up my tools** and writing the traditional "Hello, World!" — the moment every programmer knows they’re alive.

### 1.1 Installation ⚙️ — Gearing Up

- Rust is installed through **`rustup`**, the toolchain manager.  
- Bonus: `cargo` comes bundled — think of it as Rust’s Swiss Army knife (compiler, package manager, build system, all-in-one).  

Quick install:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
````

Verify:

```bash
rustc --version
cargo --version
```

✅ Boom. Rust is ready.

---

### 1.2 Hello, World! ✨ — The Ritual

The classic warm-up program:

```rust
fn main() {
    println!("Hello, world!");
}
```

* `fn main()` → the entry point.
* `println!` → **macro magic** (note the `!`).
* Run it all in one go with:

```bash
cargo run
```

And just like that → your first Rust output greets the terminal.

---

# Chapter 2 - Programming a Guessing Game 🎲

*"Learning by building beats reading about building."*

This chapter builds a fun little **number guessing game**, and along the way introduces crates, error handling, loops, and pattern matching. A perfect mini-tour of Rust’s core features.

---

### 2.1 Crates 📦 — Sharing the Toolbox

Rust code lives in **crates** (think: projects or libraries).
Want extra powers? Add dependencies in `Cargo.toml`.

Example: bring in random numbers with `rand`:

```toml
[dependencies]
rand = "0.8"
```

---

### 2.2 Input & Random Numbers 🎲

```rust
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

💡 Key takeaways:

* Variables are immutable by default → `mut` gives flexibility.
* Error handling with `.expect("message")` is built-in.
* Randomness via `rand::Rng`.

---

### 2.3 Making Guesses 🔍

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

Rust’s `match` = a smarter, stricter `switch` statement.

* Exhaustive: you must cover every possibility.
* Elegant: pattern matching feels natural.

---

### 2.4 Loops & Error Handling ♻️

Want to keep guessing until you’re right? Use a `loop`.
Also, Rust pushes you to **handle bad input safely**:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

* If parsing fails → `continue` skips and asks again.
* No silent crashes, no unsafe behavior.

---

### 2.5 The Full Game 🎉

What you get at the end:

* User input
* Random number generation
* Pattern matching
* Looping until success

It’s tiny but mighty — the first real taste of Rust’s **safety + expressiveness**.

---

# Chapter 3 - Common Programming Concepts 🧩

*"Back to basics, but the Rust way."*

Rust doesn’t reinvent programming fundamentals — but it makes them **stricter, safer, and clearer**.

---

### 3.1 Variables and Mutability 🔄

* Immutable by default → safer by design.
* Add `mut` if you really need to change a value.
* Use `const` for true constants (evaluated at compile time).

```rust
let mut x = 5;
println!("x = {}", x);
x = 6;
```

---

### 3.2 Data Types 📊

Rust is **statically typed** — every value has a known type.

* **Scalar types**: integers, floats, booleans, chars.
* **Compound types**: tuples, arrays.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (a, b, c) = tup;

let arr = [1, 2, 3, 4, 5];
```

---

### 3.3 Functions ⚡

Functions are first-class citizens.
Return values come from the last expression (no `;`).

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

---

### 3.4 Control Flow 🔀

Rust control flow feels familiar but sharp:

**if expressions** (yes, they return values!):

```rust
let result = if number < 5 {
    "small"
} else {
    "big"
};
```

**Loops** — three flavors:

* `loop` → infinite until you `break`.
* `while` → run while condition is true.
* `for` → iterate over collections.

```rust
for element in arr {
    println!("{}", element);
}
```

---

# ✅ Covered up to Chapter 3

That’s the foundation:

* ✅ Setup & Hello World
* ✅ First project (guessing game)
* ✅ Core concepts (variables, types, functions, control flow)

From **Chapter 4 onward (Ownership, References, Borrowing)** — I’ll continue diving deeper into what makes Rust truly unique.

*"Stay tuned, the real magic starts in Chapter 4."* ✨

```

---

Would you like me to also **add emojis/icons for each concept (variables, functions, loops, etc.)** to make it *even more visually engaging* for GitHub readers?
```
