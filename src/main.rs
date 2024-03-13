use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !!!");
    println!("Please input your guess. ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you supposed to guess a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
