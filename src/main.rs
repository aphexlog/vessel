//use rand::random;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    //number between 1-10
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {}", secret_number);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    while guess != secret_number {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => println!("You win!"),
        }
    }
}
