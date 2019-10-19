use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::prelude::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("secret: {}", secret_number);

    println!("guess the number (0 < n <= 100)");

    loop {
        let mut guess = String::new();
        print!("> ");
        io::stdout().flush().expect("WA WA");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number, that didn't count");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("invalid number: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}