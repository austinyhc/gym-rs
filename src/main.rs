use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let max_val = 100;
    let secret_number = rand::thread_rng().gen_range(1..=max_val);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse::<u32>() {
                Ok(num) => num,
                Err(_)  => continue,
            };

         if guess > max_val { continue; }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("Bingo!!");
                break;
            }
        }
    }
}
