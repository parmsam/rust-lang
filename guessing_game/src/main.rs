use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Inform user about the game
    println!("Guess the number!");
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Ask for user's guess
        println!("Please input your guess.");

        // Initialize mutable `guess` variable
        let mut guess = String::new();

        // Read user input, handle potential errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        // Print user's guess (note: this line has a mistake)
        println!("You guessed: {guess}");
        // Print user's guess (corrected line)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }

}