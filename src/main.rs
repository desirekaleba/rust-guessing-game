use rand::Rng;
use std::io;

fn main() {
    println!("I have got a number between 1 and 100 inclusive.");
    println!("Guess which number it is.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    println!("The secret number is {}", secret_number);
}
