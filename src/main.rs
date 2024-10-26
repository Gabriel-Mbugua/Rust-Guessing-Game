use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("You are now playing guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let max_attempts: u32 = 10;
    let mut attempts: u32 = 0;

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        match stdin().read_line(&mut guess) {
            Ok(text) => text,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        attempts += 1;
        let attempts_left = max_attempts - attempts;

        if max_attempts == attempts {
            println!("Oh no. You lost. The secret number was {}", secret_number);
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! You have {} attempts left.", { attempts_left })
            }
            Ordering::Greater => {
                println!("Too big! You have {} attempts left.", { attempts_left })
            }
            Ordering::Equal => {
                println!("You won after {} attempts!", attempts);
                break;
            }
        }
    }
}
