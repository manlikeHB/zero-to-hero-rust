# 🧮 Simple Calculator

A basic command-line calculator supporting the four fundamental arithmetic operations.

## 🎯 Learning Objectives

- **Pattern Matching**: Using `match` extensively for operation selection
- **Error Handling**: Safely parsing user input with `Result`
- **Float Arithmetic**: Working with `f64` for calculations
- **Input Validation**: Handling multiple error cases gracefully
- **Edge Cases**: Dealing with division by zero

## ➕ Supported Operations

- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)

## 🚀 Running the Program

```bash
cargo run
```

## 📝 Example Session

```
Simple calculator, enter 'exit' to quit!
Enter expression (e.g. 5 + 3):
> 10 + 5
Answer: 15

Enter expression (e.g. 5 + 3):
> 20 / 4
Answer: 5

Enter expression (e.g. 5 + 3):
> 10 / 0
Error: Division by zero is not allowed

Enter expression (e.g. 5 + 3):
> 7 * 6
Answer: 42

Enter expression (e.g. 5 + 3):
> exit
Goodbye!
```

## 🔑 Key Concepts Demonstrated

### Tuple Pattern Matching for Parsing
```rust
let (num1, num2) = match (token1.parse::<f64>(), token2.parse::<f64>()) {
    (Ok(x), Ok(y)) => (x, y),
    _ => {
        println!("Please enter valid numbers!");
        return true;
    }
};
```

### Operation Matching
```rust
match op {
    "+" => println!("Answer: {}", num1 + num2),
    "-" => println!("Answer: {}", num1 - num2),
    "/" => {
        if num2 == 0.0 {
            println!("Error: Division by zero is not allowed");
        } else {
            println!("Answer: {}", num1 / num2);
        }
    }
    "*" => println!("Answer: {}", num1 * num2),
    _ => println!("Unsupported operator {}", op),
}
```

### Input Tokenization
```rust
let expr = get_input();
let tokens: Vec<&str> = expr.split(" ").collect();

if tokens.len() < 3 {
    println!("Invalid format. Use: number operator number");
    return true;
}
```

## 💡 What I Learned

1. **Nested Pattern Matching**: Matching on tuples of `Result` values
2. **Early Returns**: Using `return` to exit functions early on errors
3. **Float Comparison**: Checking for `0.0` in division operations
4. **String to Number**: Using `.parse::<f64>()` for type conversion
5. **Graceful Degradation**: Continuing the program loop after errors
6. **User Experience**: Clear error messages for different failure cases

## 🔄 Possible Improvements

- [ ] Support for more operations (modulo, exponentiation)
- [ ] Parentheses and order of operations
- [ ] Expression evaluation (not just single operations)
- [ ] Scientific notation support
- [ ] History of calculations
- [ ] Unit tests for each operation
- [ ] Support for negative numbers
- [ ] Decimal precision control

## 📚 Relevant Rust Book Chapters

- [Chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

**Status**: ✅ Completed | **Difficulty**: Beginner