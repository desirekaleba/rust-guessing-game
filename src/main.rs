use rand::Rng;
use std::cmp::Ordering;
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
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is {}", secret_number);
}
