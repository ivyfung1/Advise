use std::io;

fn main() {

    fn smartppl() {
        println!("You are smart, so you do nothing!");
    }

    fn strongppl() {
        println!("You are strong, so you ignore patty matters!");
    }

    fn stupidppl() {
        println!("You are stupid, so you advice others.");
    }

    fn stubbornppl() {
        println!("You are stubborn, you don't listen.");
    }

    loop{
        // Print a prompt to the user
        println!("Are you 1. smart  2. strong  3. stupid  4. stubborn ?");

         // Create a mutable String to store the user input
         let mut input = String::new();

        // Read the user input and handle any potential errors
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Attempt to parse the input as u8
                match input.trim().parse::<u8>() {
                    Ok(number) if (1..=4).contains(&number) => {
                        // If parsing is successful and the number is between 1 and 4 (inclusive), print the input
                        match number {
                            1 => smartppl(),
                            2 => strongppl(),
                            3 => stupidppl(),
                            4 => stubbornppl(),
                            _ => unreachable!(), // This should never happen
                        }
                        break;
                    }
                    _ => {
                        println!("Invalid input. Please enter a number between 1 and 4.");
                    }
                }
            }
            Err(error) => {
                // If an error occurs, print an error message
                eprintln!("There is an error {}. Please enter your answer again.", error);
                break;
            }
        }
    }
}
