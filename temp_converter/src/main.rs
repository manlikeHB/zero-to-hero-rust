use std::io;

fn main() {
    while get_temp() {}
}

fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn get_temp() -> bool {
    let temp = read_temperature();

    let scale = read_scale();
    if scale.is_empty() {
        return true;
    }

    convert(temp, &scale);

    println!("Do you want to convert another temperature? (y/n)");
    let again = get_input().trim().to_lowercase();
    again == "y"
}

fn read_temperature() -> f32 {
    let temp: f32 = loop {
        println!("Enter temperature value");
        match get_input().parse::<f32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a numeric value for temperature.");
                continue;
            }
        };
    };
    temp
}

fn read_scale() -> String {
    loop {
        println!("Scale?: (C for Celsius, F for Fahrenheit)");
        let scale = get_input().trim().to_uppercase();
        println!("You entered: {}", scale);
        if scale == "C" || scale == "F" {
            return scale;
        } else {
            println!("Invalid scale entered. Please enter 'C' or 'F'.");
        }
    }
}

fn convert(temp: f32, scale: &str) {
    if scale == "C" {
        let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
        println!("Temperature: {}°C in Fahrenheit is {}°F", temp, fahrenheit);
    } else {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("Temperature: {}°F in Celsius is {}°C", temp, celsius);
    }
}
