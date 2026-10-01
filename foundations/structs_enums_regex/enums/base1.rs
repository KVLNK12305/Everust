// ===================== MY LEARNINGS (Rust enums vs structs) =====================

// 1) LOOSE DESIGN: struct + enum
// - The struct always HAS all fields at the same time
// - Fields can disagree
// - Compiler cannot enforce correctness
// - Invalid states are representable

enum Version {
    V4,
    V6,
}

struct IpLoose {
    version: Version, // tag
    addr: String,     // data (not tied to the tag)
}

let bad = IpLoose {
    version: Version::V4,
    addr: "::1".to_string(), // ❌ logically wrong, but compiles
};

// -------------------------------------------------------------------------------

// 2) BETTER DESIGN: enum with associated data
// - Value IS one of the variants
// - Either V4 OR V6 (never both)
// - Shape is correct
// - Content can still be wrong if too generic

enum IpBetter {
    V4(String),
    V6(String),
}

let still_possible = IpBetter::V4("::1".to_string()); // ⚠ logically wrong, but shape is safe

// -------------------------------------------------------------------------------

// 3) STRONG DESIGN: make invalid states unrepresentable
// - Data is enforced by the type system
// - Compiler prevents misuse
// - Bugs are caught at compile time

enum IpStrong {
    V4([u8; 4]),    // exactly 4 bytes
    V6([u16; 8]),   // exactly 8 segments
}

let home = IpStrong::V4([127, 0, 0, 1]);
let loopback = IpStrong::V6([0, 0, 0, 0, 0, 0, 0, 1]);

// IpStrong::V4("127.0.0.1"); // ❌ does not compile
// IpStrong::V4([127, 0, 1]); // ❌ does not compile

// -------------------------------------------------------------------------------

// CORE TAKEAWAYS:
//
// - Structs model "HAS fields"
// - Enums model "IS one of"
// - Loose types require discipline
// - Strong types remove whole classes of bugs
// - Good Rust design pushes correctness into the type system
//
// If invalid states are possible, the type is too weak.




