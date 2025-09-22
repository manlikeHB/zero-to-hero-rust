# ✅ To-Do List CLI

A simple command-line task manager for tracking your to-do items.

## 🎯 Learning Objectives

- **Vectors**: Using `Vec<T>` for dynamic collections
- **Tuples**: Storing multiple related values together
- **Command Parsing**: Breaking down user input into commands and arguments
- **Mutability**: Working with mutable references
- **Pattern Matching**: Handling different command types
- **State Management**: Maintaining application state across loop iterations

## 📋 Features

- Add new tasks
- List all tasks with status indicators
- Mark tasks as done
- Remove tasks
- Persistent session (in-memory)

## 🚀 Running the Program

```bash
cargo run
```

## 📝 Example Session

```
Choose an action: add/list/done/remove/exit
> add Buy groceries
Task added: Buy groceries

Choose an action: add/list/done/remove/exit
> add Read Rust book
Task added: Read Rust book

Choose an action: add/list/done/remove/exit
> list
1. [ ] Buy groceries
2. [ ] Read Rust book

Choose an action: add/list/done/remove/exit
> done 1
Task 1 marked as done.

Choose an action: add/list/done/remove/exit
> list
1. [X] Buy groceries
2. [ ] Read Rust book

Choose an action: add/list/done/remove/exit
> exit
Exiting...
```

## 🔑 Key Concepts Demonstrated

### Vector of Tuples for State
```rust
let mut list: Vec<(String, bool)> = Vec::new();
// (task_description, is_completed)
```

### Command Parsing
```rust
let binding = get_input().to_lowercase();
let input: Vec<&str> = binding.split_whitespace().collect();

let command = input[0];
let opt = input[1..].join(" "); // Rejoin for multi-word arguments
```

### Mutable Reference Passing
```rust
fn execute(list: &mut Vec<(String, bool)>) -> bool {
    // Modify the list in place
    handle_command(list, command, opt)
}
```

### Index Validation Helper
```rust
fn check_if_valid_index(list_len: usize, opt: &String) -> bool {
    match convert_to_index(opt) {
        Some(val) => list_len > val,
        None => false,
    }
}

fn convert_to_index(opt: &String) -> Option<usize> {
    opt.parse::<usize>().ok().map(|x| x - 1)
}
```

### Marking Tasks as Done
```rust
if command == "done" {
    if check_if_valid_index(list.len(), &opt) {
        match convert_to_index(&opt) {
            Some(i) => {
                let val = list.get_mut(i).unwrap();
                val.1 = true; // Set completion status
                println!("Task {} marked as done.", opt);
            }
            _ => println!("No task found at number {}", opt)
        }
    }
}
```

## 💡 What I Learned

1. **Tuples for Simple Data**: When you don't need a full struct, tuples work great
2. **Vector Manipulation**: Adding, removing, and modifying elements safely
3. **Index Conversion**: Handling user-facing 1-based indexing vs 0-based internal
4. **Option Chaining**: Using `.map()` to transform `Option` values
5. **String Joining**: Using `.join()` to reconstruct multi-word input
6. **Mutable Borrowing**: Getting mutable references with `.get_mut()`

## 🧪 Tests Included

```rust
#[test]
fn test_convert_to_index_valid() {
    let input = "3".to_string();
    assert_eq!(convert_to_index(&input), Some(2));
}

#[test]
fn test_check_if_valid_index_out_of_bounds() {
    let input = "5".to_string();
    assert_eq!(check_if_valid_index(3, &input), false);
}
```

## 🔄 Possible Improvements

- [ ] Add file persistence (save/load from JSON)
- [ ] Task priorities (high/medium/low)
- [ ] Due dates
- [ ] Task categories/tags
- [ ] Search/filter functionality
- [ ] Edit existing tasks
- [ ] Undo functionality
- [ ] Better error handling with `Result`

## 📚 Relevant Rust Book Chapters

- [Chapter 8: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Chapter 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

**Status**: ✅ Completed | **Difficulty**: Beginner