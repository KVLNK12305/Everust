use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Enter your brute value");
    let secret_number= rand::rng().random_range(1..101); // ✅ valid in rand 0.9.1
    // by default u get i32 (signed 32 bit) as input

    loop{
        let mut guess = String::new();
        // a mutable string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input"); 
        // the input function which is referencing to the mutable guess and prepared with error handling

        /*now the thing is given input is a string but, our secret_number is an integer sooo we use the same guess and parse it  
        
        Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
        */
        let guess: u32 = match guess.trim().parse() {
            // The guess in the expression refers to the original guess variable that contained the input as a string. The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do before we can convert the string to a u32
            Ok(num) => num,
            // error handling kuda jaragabaduthundhi
            Err(_) => {
                println!("Please enter a valid number.");
                return;
            }
        };

        println!("You guessed: {}", guess);
// idhi oka switch statement lantidhi but inka powerful dani kante, but ikkada deeni basics maatrame chesanu
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!");
                break;
            }
        }
    }
}
