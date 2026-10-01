Got it! Here’s your complete learnings summary **formatted in Markdown** — ready to save as a `.md` file:

````markdown
# 📝 Rust Learnings — Beginner to Tables Game

## 1. Random Number Generation

- **`rand` crate** is used for randomness.
- Add to `Cargo.toml`:
  ```toml
  rand = "0.9"
````

* Import Rng trait:

  ```rust
  use rand::Rng;
  ```

* Create RNG (rand 0.9 modern API):

  ```rust
  let mut rng = rand::rng();
  ```

* Generate numbers in a range:

  ```rust
  let num = rng.random_range(1..=10); // inclusive
  ```

* RNG must be **mutable** because generating a number changes its internal state.

* **ThreadRng** is a thread-local RNG, not a thread.

---

## 2. Reading User Input

* Use `io::stdin().read_line(&mut input)` to read input.
* Must handle the Result:

  * `.unwrap()` → panic on error with default message
  * `.expect("msg")` → panic on error with custom message
* Common idiom with shadowing:

  ```rust
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("retry again");
  let input = input.trim(); // &str
  ```
* **Parsing strings to integers** returns Result:

  ```rust
  let num: i32 = input.parse().expect("Please enter a number");
  ```

---

## 3. Handling User Input Safely

* Use `match` to handle parse results without crashing:

  ```rust
  match input.parse::<i32>() {
      Ok(value) if value == correct => println!("Correct!"),
      Ok(_) => println!("Wrong!"),
      Err(_) => println!("Please enter a valid number."),
  }
  ```
* Can check for commands like `"stop"` before parsing:

  ```rust
  if input.eq_ignore_ascii_case("stop") {
      break;
  }
  ```

---

## 4. `match` vs `if` Statements

* Use `match` when handling multiple outcomes or `Result`s.
* Use `if/else` for simple conditions.
* Example from guessing game:

  ```rust
  match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!"),
  }
  ```

---

## 5. Variables and Shadowing

* Rust allows **shadowing**:

  ```rust
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let input = input.trim(); // shadows previous input
  ```
* Shadowing is useful for converting types (String → &str) without creating new names.

---

## 6. Let Statements vs Expressions

* `let` is a **statement**, not an expression — cannot assign it to a variable:

  ```rust
  let x = (let y = 6); // ❌ invalid
  ```
* Correct approaches:

  ```rust
  let y = 6;
  let x = y; // ✅ separate statements
  ```

  Or using a block:

  ```rust
  let x = {
      let y = 6;
      y + 1 // block evaluates to last expression
  };
  ```

---

## 7. Multiplication Tables Game Lessons

* Loop structure:

  1. Generate random numbers
  2. Ask user for input
  3. Check for `"stop"`
  4. Parse input safely
  5. Compare answer using `match`
* Example of a working loop:

  ```rust
  loop {
      let x = rng.random_range(1..=10);
      let y = rng.random_range(1..=10);
      let ans = x * y;

      println!("What is {} x {}? (type 'stop' to quit)", x, y);

      let mut typed_ans = String::new();
      io::stdin().read_line(&mut typed_ans).expect("retry again");
      let typed_ans = typed_ans.trim();

      if typed_ans.eq_ignore_ascii_case("stop") {
          println!("Goodbye!");
          break;
      }

      match typed_ans.parse::<i32>() {
          Ok(num) if num == ans => println!("Correct!\n"),
          Ok(_) => println!("Wrong! The answer was {}\n", ans),
          Err(_) => println!("Please enter a valid number.\n"),
      }
  }
  ```

---

## 8. Key Takeaways

* Rust enforces **explicit error handling** (`Result`) — `.unwrap()` or `match`.
* RNGs are mutable objects; random generation changes state.
* Shadowing is idiomatic for converting types or trimming strings.
* `match` is powerful for handling multiple outcomes or parsing errors.
* Blocks `{}` can be used as expressions to evaluate multiple steps and return a value.
* Your first small game teaches **loops, input/output, parsing, random numbers, conditional logic** — all essential Rust skills before diving into ownership.

```

---
