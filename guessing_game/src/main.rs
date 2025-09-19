use rand::*;
use std::io;

fn main() {
    let max = choose_dificulty();

    while play_round(max, generate_secret(max)) {
    } 

      println!("Thanks for playing! Goodbye!");
    
}

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn generate_secret(max: u8) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(0..max).into()
}

fn choose_dificulty() -> u8 {
    println!("Choose a difficulty level: Easy, Medium, Hard");

    let max: u8;
    let difficulty: String = get_input();

    match difficulty.to_lowercase().as_str() {
        "easy" => max = 10,
        "medium" => max = 50,
        "hard" => max = 100,
        _ => { println!("Invalid difficulty level. Defaulting to Easy.");
                max = 10;
    
        }
    };

    max
}

fn play_round(max: u8, val: u32) -> bool {
    let mut guessed: bool = false;
    let mut total_guesses: u8 = 0;

    while !guessed {
        println!("Enter your Guess:");

        let input_val = get_input();

        if input_val.is_empty() {
            println!("Input cannot be empty. Please enter a number between 0 and {}.", max - 1);
            continue;
        }

        if input_val == String::from("exit") {
            println!("Exiting the game. Goodbye!");
            return false;
        }

        let input_u32 = match input_val.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number between 0 and {}.", max - 1);
                continue;
            }
        };

        if val == input_u32 {
            println!("You guessed correctly!");
            guessed = true;

            println!("wanna play again? (y/n)");
            let res = get_input();

            return res.to_lowercase().as_str() == "y";
          
        } else {
            total_guesses += 1;
            println!("Total guesses: {}", total_guesses);

            if input_u32 < val {
                println!("Too low!");
            } else {
                println!("Too high!");
            }
        }

    }

    return false;
}