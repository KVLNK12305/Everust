use std::io;
struct User {
    active: bool,
    uname: String,
    email: String,
    sign_in_count: i32,
}

fn main() {
    let user1 = User {
        active: true,
        uname: String::from("Kushal"),
        email: String::from("old@example.com"),
        sign_in_count: 1,
    };
    println!("Choose a case from 1 to 4 to understand all possiblilities");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("not a number");
    let case:i32 = input.trim().parse().expect("not a number");


    match case {
        1 => {
            // ❌ Case 1: Move without overriding Strings (will not compile)
            // let user2 = User {
            //     UName: String::from("NewName"),
            //     ..user1 // email moves → user1.email no longer usable
            // };
            println!("Case 1 would move a String and break user1");
        }

        2 => {
            // ❌ Case 2: Positional struct construction (invalid)
            // let user2 = User(true, String::from("New"), String::from("new@example.com"), 1);
            println!("Case 2 is invalid: named-field struct cannot use positional args");
        }

        3 => {
            // ⚠ Case 3: Clone Strings but never use some fields (warning)
            let user2 = User {
                uname: user1.uname.clone(),
                email: user1.email.clone(),
                ..user1 // Copy fields reused
            };
            println!("Case 3: user2 email = {}", user2.email);
            // active and sign_in_count never read → compiler warns
        }

        4 => {
            // ✅ Case 4: Clone Strings and use all fields
            let user2 = User {
                uname: user1.uname.clone(),
                email: user1.email.clone(),
                ..user1
            };
            println!(
                "Case 4:\nuser1: {}, {}, {}, {}\nuser2: {}, {}, {}, {}",
                user1.uname, user1.email, user1.active, user1.sign_in_count,
                user2.uname, user2.email, user2.active, user2.sign_in_count
            );
        }

        _ => println!("Invalid case"),
    }
}





// User(true, username, email, 1);  // ❌ Not allowed for named-field structs

/*
Here, the arguments do use position, because function parameters are positional.
But when you construct the struct inside, you still assign them by name:
 */