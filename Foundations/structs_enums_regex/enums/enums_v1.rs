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

fn main(){
    let dir = Direction::East;
    /*
    At runtime:

        d is not a string

        d is not an integer

        d is a tagged value chosen from a closed set of possibilities

    This “closed set” property is crucial.
     */

}


Bad design:

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