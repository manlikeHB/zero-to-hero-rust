# 📇 Contact Book CLI

A persistent contact management system with JSON serialization.

## 🎯 Learning Objectives

- **Structs**: Defining custom data types
- **Serialization**: Using `serde` for JSON conversion
- **File Persistence**: Saving and loading data between sessions
- **Derive Macros**: Using `#[derive]` for automatic trait implementation
- **CRUD Operations**: Create, Read, Update, Delete functionality
- **Error Handling**: Gracefully handling file I/O errors

## 📋 Features

- Add new contacts (name, phone, email)
- List all contacts
- Search contacts by name
- Delete contacts
- Persistent storage (JSON file)
- Automatic save on exit

## 🚀 Running the Program

```bash
cargo run
```

## 📝 Example Session

```
Choose an action: add/list/delete/search/exit
> add john 0908213 john@best.com
(Contact added)

Choose an action: add/list/delete/search/exit
> add sarah 345432 sarah@it.com
(Contact added)

Choose an action: add/list/delete/search/exit
> list
1. john 0908213 john@best.com
2. sarah 345432 sarah@it.com

Choose an action: add/list/delete/search/exit
> search sa
1. sarah 345432 sarah@it.com

Choose an action: add/list/delete/search/exit
> delete 2
Contact Deleted!

Choose an action: add/list/delete/search/exit
> exit
```

## 🔑 Key Concepts Demonstrated

### Struct with Serde
```rust
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: String,
    email: String,
}
```

### JSON Serialization
```rust
fn save_contact(contact_list: &Vec<Contact>) {
    let json = serde_json::to_string_pretty(contact_list).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
```

### JSON Deserialization with Fallback
```rust
fn load_contact() -> Vec<Contact> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}
```

### Search Functionality
```rust
let name = res[1];
let mut found = false;
for (i, contact) in contact_list.iter().enumerate() {
    if contact.name.contains(name) {
        println!("{}. {} {} {}", i + 1, contact.name, contact.phone, contact.email);
        found = true;
    }
}

if !found {
    println!("No contact found with name containing '{}'", name);
}
```

## 💡 What I Learned

1. **External Crates**: Adding and using `serde` and `serde_json`
2. **Derive Macros**: Using `#[derive(Serialize, Deserialize)]` for automatic trait impl
3. **Pretty Printing JSON**: Using `to_string_pretty()` for readable output
4. **Pattern Matching on File Operations**: Using `if let Ok()` for optional file loading
5. **Error Recovery**: Providing default values when deserialization fails
6. **String Methods**: Using `.contains()` for substring search
7. **Enumerate**: Getting both index and item when iterating

## 🧪 Tests Included

```rust
#[test]
fn test_manage_contact() {
    let mut contact_list = Vec::<Contact>::new();
    
    // Test add
    assert!(manage_contact(&mut contact_list, "add john 090123 john@best.com".to_string()));
    assert_eq!(*contact_list.get(0).unwrap(), get_john_contact());
    
    // Test delete
    assert!(manage_contact(&mut contact_list, "delete 1".to_string()));
    assert_eq!(contact_list.len(), 0);
}
```

## 🔄 Possible Improvements

- [ ] Edit existing contacts
- [ ] Multiple phone numbers per contact
- [ ] Contact groups/categories
- [ ] Export to CSV
- [ ] Import from vCard
- [ ] Better error handling with `Result`
- [ ] Validation for email/phone formats
- [ ] Fuzzy search
- [ ] Sort by different fields

## 📚 Relevant Rust Book Chapters

- [Chapter 5: Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Chapter 10: Generic Types, Traits](https://doc.rust-lang.org/book/ch10-00-generics.html)

## 📦 Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

**Status**: ✅ Completed | **Difficulty**: Intermediate