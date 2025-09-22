# 📊 Word Counter CLI

A CLI tool that analyzes text files and provides statistics including word frequency analysis.

## 🎯 Learning Objectives

- **File I/O**: Reading files from disk
- **HashMap**: Using collections for frequency counting
- **Iterators**: Processing text with iterator methods
- **String Operations**: Splitting, trimming, case conversion
- **Path Handling**: Working with `std::path::Path`

## 📈 Features

- Line count
- Word count
- Character count
- Top 5 most frequent words

## 🚀 Running the Program

```bash
cargo run
```

Then enter the path to a text file when prompted.

## 📝 Example Output

```
Enter file path:
> text.txt

Lines: 3
Words: 89
Chars: 571

Top 5 words:
the: 8
of: 6
code: 5
to: 4
a: 4
```

## 🔑 Key Concepts Demonstrated

### HashMap for Frequency Counting
```rust
let mut top_words: HashMap<String, usize> = HashMap::new();

for word in &words {
    let word = word.to_lowercase();
    *top_words.entry(word).or_insert(0) += 1;
}
```

### Sorting by Frequency
```rust
let mut top_vec: Vec<(String, usize)> = top_words.into_iter().collect();
top_vec.sort_by(|a, b| b.1.cmp(&a.1));

for (word, count) in top_vec.iter().take(5) {
    println!("{}: {}", word, count);
}
```

### File Reading with Error Handling
```rust
fn read_file_content(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            println!("err: {}", err);
            panic!("Failed!!")
        }
    }
}
```

### Iterator-Based Processing
```rust
let words = content.split_whitespace().collect::<Vec<&str>>();
let lines = content.lines().count();
let chars = content.chars().collect::<Vec<char>>().len();
```

## 💡 What I Learned

1. **Collections**: Using `HashMap` for counting and aggregation
2. **Ownership with Collections**: Understanding how data moves between `HashMap` and `Vec`
3. **Iterator Adapters**: Chaining `.iter()`, `.take()`, `.collect()`
4. **Entry API**: Using `.entry().or_insert()` for elegant counting
5. **Path Types**: Difference between `&str`, `String`, and `&Path`

## 🔄 Possible Improvements

- [ ] Better error handling with `Result` instead of `panic!`
- [ ] Configurable top N words
- [ ] Ignore common stop words (the, a, an, etc.)
- [ ] Support multiple file formats
- [ ] Export results to JSON/CSV
- [ ] Add unit tests

## 📚 Relevant Rust Book Chapters

- [Chapter 8: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Chapter 12: I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

---

**Status**: ✅ Completed | **Difficulty**: Beginner