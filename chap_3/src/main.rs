use std::{io};

// type definitions in functions
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn if_else() {
    // control flow

    /*
    definition of control flow

    The ability to run some code depending on whether a condition is true and to run some
    code repeatedly while a condition is true are basic building blocks in most programming languages

    */
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read the number");

    //let tcn: u32 = number.parse().expect("Failed to parse number from string");

    /*If you type 5 and press Enter, number actually contains "5\n". Parsing "5\n" to tcn fails. */
    // tcn - typecasted number
    let tcn: u32 = number.trim().parse().expect("Failed to parse number from string");

    if tcn%2==0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    let j = 6;

    if j % 4 == 0 {
        println!("number is divisible by 4");
    } else if j % 3 == 0 {
        println!("number is divisible by 3");
    } else if j % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

}

fn loopings(){
    // loopings

    /*
    definition of loopings

    The ability to run some code repeatedly while a condition is true are basic building blocks in most programming languages
    */

    let mut i = 0;

    // loop
    loop{
        println!("i is {}", i);
        i += 1;
        if i == 5 {
            break;
        }
    }

    // while
    let mut j = 0;
    while j < 5 {
        println!("j is {}", j);
        j += 1;
    }

    // for  k in 0 to 5
    for k in 0..5 {
        println!("k is {}", k);
    }

    //usage of break statement
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

}


fn main() {

    let mut option: String = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: u32 = option.trim().parse().expect("Failed to parse number");

    if option == 0 {
        print_labeled_measurement(5, 'm');
    } else if option == 1 {
        if_else();
    } else if option == 2 {
        loopings();
    } else {
        println!("Invalid option");
    }
}