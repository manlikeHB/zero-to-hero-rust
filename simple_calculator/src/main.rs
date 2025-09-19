use std::io;

fn main() {
    println!("Simple calculator, enter 'exit' to quit!");
    while calculate() {}
}

fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn calculate() -> bool {
    println!("Enter expression (e.g. 5 + 3):");

    let expr = get_input();
    let tokens: Vec<&str> = expr.split(" ").collect();

    if expr.to_lowercase() == "exit" {
        println!("Goodbye!");
        return false;
    }

    if tokens.len() < 3 {
        println!("Invalid format. Use: number operator number");
        return true;
    }

    let token1 = tokens[0];
    let op = tokens[1];
    let token2 = tokens[2];

    let (num1, num2) = match (token1.parse::<f64>(), token2.parse::<f64>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            println!("Please enter valid numbers!");
            return true;
        }
    };

    match op {
        "+" => {
            println!("Answer: {}", num1 + num2);
        }
        "-" => {
            println!("Answer: {}", num1 - num2);
        }
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed");
            } else {
                println!("Answer: {}", num1 / num2);
            }
        }
        "*" => {
            println!("Answer: {}", num1 * num2);
        }
        _ => {
            println!("Unsupported operator {}", op);
        }
    }

    return true;
}
