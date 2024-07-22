use std::io;

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn f_to_c_msg(f: f64, c: f64) -> String {
    format!(
        "\n{} degrees Fahrenheit is equal to \x1b[1m{}\x1b[0m degrees Celsius.\nFormula: ({} - 32) * 5 / 9 = {}\n",
        f, c, f, c
    )
}

fn c_to_f_msg(c: f64, f: f64) -> String {
    format!(
        "\n{} degrees Celsius is equal to \x1b[1m{}\x1b[0m degrees Fahrenheit.\nFormula: ({} * 9 / 5) + 32 = {}\n",
        c, f, c, f
    )
}

fn main() {
    println!("This program helps convert between Fahrenheit and Celsius temperatures.");
    println!("Type c for Celsius or f for Fahrenheit, followed by the temperature value.");
    println!("For example, 'c 32' converts 32 degrees Celsius to Fahrenheit.");
    println!("Type 'quit' or 'exit' to quit the tool.");
    loop {
        println!("Please enter the temperature units and value you'd like to convert:");
        // initialize i64 variable to store temperature value
        let mut guess = String::new();

        // Read user input, handle potential errors
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" || guess.trim() == "exit" {
            println!("Quitting temperature conversion tool...");
            break;
        }

        // split the input into a vector of strings
        let temp_input: Vec<&str> = guess.trim().split_whitespace().collect();

        // check if the input is valid
        if temp_input.len() != 2 {
            println!("Invalid input. Please enter the temperature units and value.");
            continue;
        }

        // get the temperature units and value
        let temp_units = temp_input[0];
        let temp_value: f64 = match temp_input[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature value.");
                continue;
            }
        };

        // convert the temperature based on the units
        match temp_units {
            "c" => {
                let fahrenheit = c_to_f(temp_value);
                let x = c_to_f_msg(temp_value, fahrenheit);
                println!("{}", x);
            }
            "f" => {
                let celcius = f_to_c(temp_value);
                let y = f_to_c_msg(temp_value, celcius);
                println!("{}", y);
            }
            _ => {
                println!("Invalid temperature units. Please enter 'c' for Celsius or 'f' for Fahrenheit.");
                continue;
            }
        }
    }

}

