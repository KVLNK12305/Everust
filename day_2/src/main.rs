use std::io;
fn main(){
    println!("Enter your brute value");
    let mut guess = Integer::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read it");
    println!("You guessed: {}", guess);
}
