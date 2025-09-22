# 📊 Mini CSV Parser

A custom CSV parser built from scratch without external CSV libraries, demonstrating manual file parsing and data structure design.

## 🎯 Learning Objectives

- **File I/O**: Reading files line by line with `BufReader`
- **String Parsing**: Splitting and processing CSV format
- **Iterator Methods**: Using `enumerate()`, `position()`, `iter()`
- **Option Handling**: Working with `Option<T>` for safe access
- **Error Propagation**: Using `?` operator for `Result` types
- **Struct Design**: Creating ergonomic APIs for data access

## 📋 Features

- Parse CSV files into structured data
- Header extraction
- Row-wise access
- Column lookup by name
- Type-safe record iteration
- Automatic data cleaning (trimming whitespace)

## 🚀 Running the Program

```bash
cargo run
```

The program reads `text.csv` and parses it into `Record` structs.

## 📝 Example CSV

```csv
name,age,city
Alice,30,London
Bob,25,Paris
mike, 30
sarah, r, lagos
```

## 🔑 Key Concepts Demonstrated

### Buffered File Reading
```rust
fn from_file(path: &str) -> std::io::Result<Csv> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    
    let mut headers = Vec::<String>::new();
    let mut rows = Vec::<Vec<String>>::new();
    
    for (i, res) in reader.lines().enumerate() {
        let line = res?;
        let cols = line.split(",").map(|x| x.to_string()).collect();
        if i == 0 {
            headers = cols;
        } else {
            rows.push(cols);
        }
    }
    
    Ok(Csv { headers, rows })
}
```

### Column Lookup by Name
```rust
fn get(&self, row: usize, cols: &str) -> Option<&str> {
    let idx = self.headers.iter().position(|x| x == cols)?;
    self.rows.get(row)?.get(idx).map(|s| s.as_str())
}
```

### Type-Safe Record Iteration
```rust
fn iter_records(&self) -> Vec<Record> {
    let mut records = Vec::<Record>::new();
    for i in 1..self.rows.len() {
        let data: Vec<String> = self.rows[i]
            .iter()
            .map(|a| a.trim().to_string())
            .collect();
        
        if data.len() < 3 { continue; }
        
        if let Ok(n) = data[1].parse::<u32>() {
            records.push(Record::new(
                data[0].clone(), 
                n, 
                data[2].clone()
            ));
        }
    }
    records
}
```

### Record Struct
```rust
#[derive(Debug)]
struct Record {
    name: String,
    age: u32,
    city: String,
}
```

## 💡 What I Learned

1. **BufReader**: Efficient line-by-line file reading
2. **Iterator Chaining**: Combining `enumerate()`, `map()`, `collect()`
3. **Option Chaining**: Using `?` in `Option` contexts
4. **Early Return Patterns**: Using `?` for error propagation
5. **Data Validation**: Skipping malformed rows gracefully
6. **String Ownership**: When to use `&str`, `String`, `.clone()`
7. **Position Method**: Finding index of elements in collections

## 🧪 Tests Included

```rust
#[test]
fn test_get() {
    let csv = Csv::from_file("text.csv").unwrap();
    
    assert_eq!(csv.get(0, "name").unwrap(), "Alice");
    assert_eq!(csv.get(0, "city").unwrap(), "London");
    assert!(csv.get(0, "invalid_col").is_none());
    assert!(csv.get(99, "name").is_none());
}

#[test]
fn test_from_file() {
    let csv = Csv::from_file("text.csv").unwrap();
    assert!(!csv.rows.is_empty());
}
```

## 🔄 Possible Improvements

- [ ] Support quoted fields with commas
- [ ] Handle escaped quotes
- [ ] Different delimiter support (tabs, pipes)
- [ ] Generic record type with macros
- [ ] Streaming API for large files
- [ ] Write CSV functionality
- [ ] Better error types with `thiserror`
- [ ] Iterator-based API instead of collecting to Vec
- [ ] Column type inference

## 📚 Relevant Rust Book Chapters

- [Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Chapter 13: Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

---

**Status**: ✅ Completed | **Difficulty**: Intermediate