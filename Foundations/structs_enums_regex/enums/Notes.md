// ===================== QUESTIONS I HAD & ANSWERS I FOUND =====================

// Q1: Is enum-with-data just a shorthand for struct + enum?
// A: No.
//    - Struct + enum = two independent fields (can disagree)
//    - Enum-with-data = either/or (variants are mutually exclusive)


// Q2: In the loose version, can an IP be both V4 and V6?
// A: No.
//    - It is NOT both at the same time
//    - It HAS both fields, which can contradict each other


// Q3: In the enum version, can I still mess things up?
// A: Yes, but only at the content level.
//    - You can put wrong data inside (e.g., "::1" as a String for V4)
//    - You cannot mess up the structure anymore


// Q4: Why is enum-with-data still better than struct + enum?
// A: Because it prevents structural bugs:
//    - No mismatched tags and data
//    - Match statements must handle all cases
//    - Fewer silent logic errors


// Q5: How do I completely prevent mistakes?
// A: Strengthen the type itself.

enum Ip {
    V4([u8; 4]),
    V6([u16; 8]),
}

// Now:
// - Wrong shapes do not compile
// - Wrong lengths do not compile
// - Only valid IP forms are representable


// Q6: What is the core Rust principle behind all this?
// A: Make invalid states unrepresentable.
//
// Move correctness from runtime checks
// into the type system whenever possible.
