use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome, to the guessing game!");
    println!("Please choose a number between 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
            Ordering::Greater => println!("Too large"),
        }
    }
}
