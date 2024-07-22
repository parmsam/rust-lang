use std::io;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacci_sequence(n: u32) -> String {
    let mut sequence = String::new();
    for i in 0..n {
        sequence.push_str(&fibonacci(i).to_string());
        sequence.push_str(" ");
    }
    sequence
}

fn main() {
    println!("Get the nth number of the Fibonacci sequence.");
    println!("Type 'quit' or 'exit' to quit the game.");
    loop {
        println!("Please input the nth number.");
        // Initialize mutable `guess` variable
        let mut guess = String::new();    
        // Read user input, handle potential errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "0" {
            println!("Please enter a number greater than zero!");
            continue;
        }
        // Quit on "quit" or "exit" commands otherwise continue
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    if guess.trim() == "quit" || guess.trim() == "exit" {
                        println!("Quitting game...");
                        break;
                    } else {
                        println!("Please enter a number!");
                        continue;
                    }
                }
        };

        let result = fibonacci(guess - 1);
        let seq = fibonacci_sequence(guess);
        println!("The {}th number of the Fibonacci sequence is: {}", guess, result);
        println!("The sequence is {}", seq);
        println!();

    }
}