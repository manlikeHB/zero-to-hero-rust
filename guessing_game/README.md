# 🎲 Guessing Game

A classic number guessing game with difficulty levels - my first Rust project!

## 🎯 Learning Objectives

- **Rust Basics**: Variables, functions, control flow
- **User Input**: Reading from stdin and handling input
- **Random Numbers**: Using external crates (`rand`)
- **Pattern Matching**: Using `match` for game logic
- **Error Handling**: Parsing user input safely
- **Loops**: Game loop and replay functionality

## 🎮 How It Works

1. Player selects difficulty (Easy: 0-9, Medium: 0-49, Hard: 0-99)
2. Program generates a random secret number
3. Player guesses until correct
4. Feedback provided (too high/too low)
5. Option to play again after winning

## 🚀 Running the Game

```bash
cargo run
```

## 📝 Example Session

```
Choose a difficulty level: Easy, Medium, Hard
> easy
Enter your Guess:
> 5
Too low!
Total guesses: 1
Enter your Guess:
> 8
You guessed correctly!
wanna play again? (y/n)
> n
Thanks for playing! Goodbye!
```

## 🔑 Key Concepts Demonstrated

### Using External Crates
```rust
use rand::*;

fn generate_secret(max: u8) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(0..max).into()
}
```

### Pattern Matching
```rust
match difficulty.to_lowercase().as_str() {
    "easy" => max = 10,
    "medium" => max = 50,
    "hard" => max = 100,
    _ => {
        println!("Invalid difficulty level. Defaulting to Easy.");
        max = 10;
    }
}
```

### Error Handling with Parse
```rust
let input_u32 = match input_val.parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input. Please enter a valid number.");
        continue;
    }
};
```

## 💡 What I Learned

1. **Cargo Workflow**: Adding dependencies, running projects
2. **String Handling**: `.trim()`, `.to_lowercase()`, `.as_str()` - understanding when to use each
3. **Type Conversion**: Parsing strings to numbers, handling failures
4. **Game Loop Pattern**: Structuring interactive CLI applications
5. **Input Validation**: Making programs robust against unexpected input

## 🔄 Possible Improvements

- [ ] Add maximum guess limit for each difficulty
- [ ] Track high scores
- [ ] Add hints (e.g., "getting warmer/colder")
- [ ] Save game statistics to file
- [ ] Better error messages with colored output

## 📚 Relevant Rust Book Chapters

- [Chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)

---

**Status**: ✅ Completed | **Difficulty**: Beginner 