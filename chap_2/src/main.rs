use std::io;
fn main(){
    println!("Enter your brute value");
    let mut guess = String::new();
    // this :: is an associate function which on left is class and the right is the fn

    // read_line puts what the user types into the string we’re passing it, but it also returns a value—in this case, an io::Result
    io::stdin().read_line(&mut guess).expect("Failed to read it");
    // process is standard inp is provided, line is read if read, printed in the next line else prints failed to read
    println!("You guessed: {}", guess);
}
