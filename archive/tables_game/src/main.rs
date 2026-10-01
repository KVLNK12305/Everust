use rand::Rng;
use std::io;
fn main() {
    println!("Welcome to my very first tables game");

    // let mut guess = String::new();

    // the new API (rand 0.9)
    let mut rng_thread = rand::rng();

    loop{
        let x = rng_thread.random_range(1..=10);
        let y = rng_thread.random_range(1..=10);
        let ans = x*y;

        println!("What is {} x {} ? (type 'stop' to quit)", x, y);

        let mut typed_ans = String::new();
        io::stdin().read_line(&mut typed_ans).expect("retry again");
        let typed_ans = typed_ans.trim();

        if typed_ans.eq_ignore_ascii_case("stop"){
            print!("tata");
            break;
        }

        match typed_ans.parse::<i32>(){
            Ok(num) if num == ans => println!("right answer"),
            Ok(_) => println!("wrong answer!"),
            Err(_) => println!("Please type a valid num or stop"),
        }
    }
}
