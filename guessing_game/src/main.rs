use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Generate the random number to guess
    let secret_number = rand::thread_rng().gen_range(1..=100); // between 1 and 100
    println!("The secret number is: {secret_number}");
    loop { //Loop until user guessed correctly
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!"); // read user input
        let guess: u32 = match guess.trim().parse() { //cast string to number
            Ok(num) => num,
            Err(_) => {
                println!("Please insert a number!");
                continue;
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // compare guessed and secret random number
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
