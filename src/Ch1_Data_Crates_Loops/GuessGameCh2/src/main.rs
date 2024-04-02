use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    loop {      // loop to play multiple times
        println!("Please input your guess; ");

        let mut guess = String::new();    
        io::stdin()                         // User input to a empty string
            .read_line(&mut guess)
            .expect("Failed to read line");

        let secret_number = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100
        let guess: u32 = match guess.trim().parse() { //Converting the user input (String) to a number, we also check it is a valid input
            Ok(num) => num,
            Err(_) => continue,
        };
         
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // using the cmp function to compare the guess with the secret number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("The secret number is: {secret_number}\n\n\n");
    }
}
