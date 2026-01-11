/*An enum represents one value that can be exactly one of several possibilities.*/

enum Direction {
    North,
    South,
    East,
    West,
}
/*
What does this mean?
    Direction is a type
    A value of that type must be exactly one variant
    There is no “invalid” direction
*/

    /*
    At runtime:
        d is not a string
        d is not an integer
        d is a tagged value chosen from a closed set of possibilities
    This “closed set” property is crucial.
     */


// Bad design:

fn move_player(dir: &str) {
    match dir {
        "North" => println!("Moving North"),
        "South" => println!("Moving South"),
        "East" => println!("Moving East"),
        "West" => println!("Moving West"),
        _ => println!("Invalid direction!"),
    }
    // problem: what if we misspell a direction?
    // problem: what if we pass in an invalid string?
    // if str is not a proper  direction, compiler can't help us
 }


fn move_npc(dir: Direction){
    match dir {
        Direction::North => println!("NPC moving North"),
        Direction::South => println!("NPC moving South"),
        Direction::East => println!("NPC moving East"),
        Direction::West => println!("NPC moving West"),
    }
    // safe: dir must be one of the valid variants
    // no invalid states possible
    // compiler enforces correctness
}

enum Msg {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
    ChangeColor { r: i32, g: i32, b: i32 },
}

// defines behavior (OK at top level)
fn process_message(msg: Msg) {
    match msg {
        Msg::Quit => println!("Quit message"),
        Msg::Write(text) => println!("Writing message: {}", text),
        Msg::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Msg::ChangeColor { r, g, b } => {
            println!("Changing color to RGB({}, {}, {})", r, g, b)
        }
    }
}

fn main() {
    // executable code lives here
    let m1 = Msg::Write("hello".to_string());
    let m2 = Msg::Move { x: 10, y: 20 };

    process_message(m1);
    process_message(m2);
}


// These two lines are illegal at module scope:

// let m1 = Msg::Write("hello".to_string());
// let m2 = Msg::Move { x: 10, y: 20 };

/*
Reason:

let is a statement, and statements are only allowed inside functions or blocks.

Rust files are modules, not scripts.
 */