//! Module 01: Ownership & Borrowing Foundations
//! 
//! This crate serves as the executable reference implementation for Module 01.
//! It demonstrates the core mechanics of Rust's ownership model, move semantics,
//! reference borrowing rules, and zero-cost slice abstractions.

fn main() {
    println!("=== Module 01: Ownership & Borrowing Reference ===\n");

    demonstrate_move_semantics();
    demonstrate_borrowing_rules();
    demonstrate_slice_references();
    demonstrate_raii_scoping();
    
    println!("\n=== All Module 01 demonstrations executed successfully ===");
}

/// 1. Move Semantics vs Copy Semantics
/// Demonstrates how heap-allocated types (String, Vec) transfer ownership (move),
/// whereas stack-only primitives (i32, bool) implement the Copy trait and duplicate.
fn demonstrate_move_semantics() {
    println!("--- 1. Move Semantics & Copy Trait ---");

    // Primitive types implement Copy: bits are duplicated entirely on the stack
    let x = 42;
    let y = x; // Copy occurs
    println!("Stack Copy: x = {}, y = {} (both valid)", x, y);

    // Heap-allocated types transfer ownership: stack metadata (ptr, len, cap) moves
    let s1 = String::from("Rust Ownership");
    let s2 = s1; // Move occurs: s1 is invalidated by the compiler to prevent double-free
    
    // println!("{}", s1); // COMPILER ERROR [E0382]: borrow of moved value `s1`
    println!("Heap Move:  s2 = '{}' (s1 pointer invalidated)", s2);

    // Explicit cloning duplicates both stack metadata and heap buffer
    let s3 = s2.clone();
    println!("Heap Clone: s2 = '{}', s3 = '{}' (two independent heap allocations)\n", s2, s3);
}

/// 2. Borrowing Rules & Reference Aliasing
/// Demonstrates:
/// - Any number of immutable references (&T) may coexist (shared read access).
/// - Exactly one mutable reference (&mut T) may exist at a time (exclusive write access).
/// - Immutable and mutable references cannot coexist in the same scope.
fn demonstrate_borrowing_rules() {
    println!("--- 2. Borrowing & Reference Aliasing ---");

    let mut buffer = String::from("System Buffer");

    // Multiple immutable borrows allowed simultaneously
    let r1 = &buffer;
    let r2 = &buffer;
    println!("Shared Borrows: r1 = '{}', r2 = '{}'", r1, r2);
    // r1 and r2 scopes end here (Non-Lexical Lifetimes - NLL)

    // Exclusive mutable borrow allowed once read references are no longer used
    let r_mut = &mut buffer;
    r_mut.push_str(" [MODIFIED]");
    println!("Exclusive Borrow: r_mut = '{}'\n", r_mut);
}

/// 3. Zero-Allocation Slices (&str, &[T])
/// Demonstrates how slices provide views into contiguous memory without ownership transfer
/// or additional heap allocations.
fn demonstrate_slice_references() {
    println!("--- 3. Zero-Allocation Slices ---");

    let telemetry = String::from("SYS_OK:CPU_LOAD_12%:MEM_OK");
    
    // &str slices borrow a substring directly from the existing heap buffer
    let status = &telemetry[0..6];
    let metrics = &telemetry[7..];
    
    println!("Full Buffer: '{}'", telemetry);
    println!("Slice View 1 (Status):  '{}'", status);
    println!("Slice View 2 (Metrics): '{}'\n", metrics);
}

/// 4. RAII and Scope Unwinding
/// Demonstrates deterministic deallocation when variables exit lexical scope.
fn demonstrate_raii_scoping() {
    println!("--- 4. RAII & Scope Unwinding ---");

    struct ScopeTracker(&'static str);

    impl Drop for ScopeTracker {
        fn drop(&mut self) {
            println!("  [RAII Drop] Deallocating resource: {}", self.0);
        }
    }

    println!("Entering inner block scope...");
    {
        let _res1 = ScopeTracker("Network Socket");
        let _res2 = ScopeTracker("File Descriptor");
        println!("  Inner block executing with active resources...");
    } // _res2 and _res1 are dropped in reverse order of allocation here
    println!("Exited inner block scope. Resources freed deterministically.\n");
}