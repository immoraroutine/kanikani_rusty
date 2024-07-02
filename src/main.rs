use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    print!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
           
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }

    
}
