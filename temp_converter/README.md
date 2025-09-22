# 🌡️ Temperature Converter

A simple CLI tool for converting between Celsius and Fahrenheit.

## 🎯 Learning Objectives

- **Functions**: Breaking logic into reusable pieces
- **Input Validation**: Ensuring users enter valid data
- **Ownership Basics**: Working with String types
- **Control Flow**: Loop-based program structure
- **Type Conversion**: Working with `f32` for temperature calculations

## 🔄 How It Works

1. User enters a temperature value
2. User selects scale (C or F)
3. Program converts and displays result
4. Option to convert another temperature

## 🚀 Running the Program

```bash
cargo run
```

## 📝 Example Session

```
Enter temperature value
> 100
Scale?: (C for Celsius, F for Fahrenheit)
> C
You entered: C
Temperature: 100°C in Fahrenheit is 212°F
Do you want to convert another temperature? (y/n)
> n
```

## 🔑 Key Concepts Demonstrated

### Input Validation with Loops
```rust
fn read_temperature() -> f32 {
    let temp: f32 = loop {
        println!("Enter temperature value");
        match get_input().parse::<f32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a numeric value.");
                continue;
            }
        };
    };
    temp
}
```

### Temperature Conversion Logic
```rust
fn convert(temp: f32, scale: &str) {
    if scale == "C" {
        let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
        println!("{}°C = {}°F", temp, fahrenheit);
    } else {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("{}°F = {}°C", temp, celsius);
    }
}
```

### Reusable Input Function
```rust
fn get_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
```

## 💡 What I Learned

1. **Function Design**: Creating focused, single-purpose functions
2. **String Ownership**: Understanding when to use `&str` vs `String`
3. **Input Sanitization**: Using `.trim()` and `.to_uppercase()` for robust input
4. **Error Recovery**: Using loops to re-prompt on invalid input
5. **Float Arithmetic**: Working with `f32` for calculations

## 🔄 Possible Improvements

- [ ] Add Kelvin support
- [ ] Batch conversion from file
- [ ] Unit tests for conversion formulas
- [ ] Support for scientific notation input
- [ ] Add precision control (decimal places)

## 📚 Relevant Rust Book Chapters

- [Chapter 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

---

**Status**: ✅ Completed | **Difficulty**: Beginner 