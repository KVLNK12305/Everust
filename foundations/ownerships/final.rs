fn main(){
    let s = String::from("hello");
    let s = takes_ownership(s);
    /*
    now the function ownes s so print statement can t access s as 2 onwes can  not be present at the same time so u can either borrow it or clone it
     */
    println!("{}",s);
}

fn takes_ownership(some_string: String) -> String{
    println!("{}",some_string);
    some_string

}


/*

So basically i got 3 options 
1. clone it 
2. borrow it and use it from the reference perspective
3. give ownership after job done 
 */







/*
Here are **three tiny snippets** showing the core idea. Pick the one you want to remember.

---

## ❌ **1. Moves ownership (your error)**

```rust
let s = String::from("hello");
takes(s);        // ownership moves
println!("{}", s); // ❌ error: s was moved
```

---

## ✔️ **2. Borrow instead (no move)**

```rust
let s = String::from("hello");
takes(&s);       // borrow
println!("{}", s); // ✔️ still usable
```

---

## ✔️ **3. Clone if you need two owners**

```rust
let s = String::from("hello");
takes(s.clone()); // new copy
println!("{}", s); // ✔️ still usable
```

---

Tell me if you want an even smaller “one-liner” version.

 */