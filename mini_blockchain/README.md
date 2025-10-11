# 🔗 Mini Blockchain Ledger

A simple blockchain implementation demonstrating cryptographic hashing, chain validation, and immutable data structures.

## 🎯 Learning Objectives

- **Cryptographic Hashing**: Using SHA-256 to create tamper-proof blocks
- **Data Structures**: Building linked structures via cryptographic hashes
- **Chain Validation**: Detecting tampering through hash verification
- **Module Organization**: Separating concerns across multiple files
- **Interactive CLI**: Menu-driven interface with persistent state
- **Testing**: Writing comprehensive unit tests

## 🎮 How It Works

1. Genesis block created with fixed timestamp
2. User adds transactions to the blockchain
3. Each block is cryptographically linked to the previous block
4. Chain can be validated to detect tampering
5. Hash changes if any block data is modified

## 🚀 Running the Project

```bash
cargo run
```

## 📝 Example Session

```
=== Blockchain CLI ===
1. Add transaction
2. View chain
3. Validate chain
4. Exit
Choose an option: 
1
Enter transaction data:
Alice sends 10 BTC to Bob
Transaction added.

Choose an option: 
2

=== Full Blockchain ===

Block #0
├─ Timestamp: 2025-01-01 00:00:00 UTC
├─ Data: Genesis Block
├─ Previous Hash: genesis
└─ Hash: 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b

Block #1
├─ Timestamp: 2025-10-11 15:30:00 UTC
├─ Data: Alice sends 10 BTC to Bob
├─ Previous Hash: 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
└─ Hash: 9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08

Total blocks: 2

Choose an option: 
3
✓ Blockchain is VALID! All blocks verified.
```

## 🔑 Key Concepts Demonstrated

### Block Structure
```rust
pub struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}
```

### Hash Calculation
```rust
pub fn calculate_hash_with_parts(
    index: u64,
    timestamp: &str,
    data: &str,
    previous_hash: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(timestamp);
    hasher.update(data);
    hasher.update(previous_hash);
    format!("{:x}", hasher.finalize())
}
```

### Chain Validation Using Windows
```rust
pub fn is_valid(&self) -> bool {
    for blocks in self.chain.windows(2) {
        if blocks[1].hash() != blocks[1].calculate_hash() {
            return false;
        }
        if blocks[1].previous_hash() != blocks[0].hash() {
            return false;
        }
        if blocks[1].index() != blocks[0].index() + 1 {
            return false;
        }
    }
    true
}
```

### Interactive CLI Loop
```rust
pub fn execute() {
    println!("=== Blockchain CLI ===");
    let mut blockchain = Blockchain::new();

    loop {
        print_menu();
        
        if !handle_input(&mut blockchain) {
            break;
        }
        
        println!()
    }
}
```

## 💡 What I Learned

1. **Cryptographic Hashing**: Using SHA-256 to create unique, deterministic identifiers
2. **Linked Data Structures**: Connecting blocks through hash references
3. **Iterator Patterns**: Using `.windows(2)` for pairwise validation
4. **Module System**: Organizing code across `block.rs`, `blockchain.rs`, and `cli.rs`
5. **State Management**: Maintaining blockchain state across CLI loop iterations
6. **Borrowing in Practice**: Mutable vs immutable borrows in different contexts
7. **Test Organization**: Using `#[cfg(test)]` for test-only code
8. **Display Trait**: Implementing custom formatting for user-friendly output

## 🔄 Possible Improvements

- [ ] Add file persistence (save/load blockchain from JSON)
- [ ] Implement proof-of-work mining
- [ ] Add transaction signing with public/private keys
- [ ] Support multiple transaction types
- [ ] Build Merkle trees for efficient validation
- [ ] Expose as REST API

## 📚 Relevant Rust Book Chapters

- [Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Chapter 11: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Chapter 13: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

## 🧪 Running Tests

```bash
# Run all tests
cargo test
```

---

**Status**: ✅ Completed | **Difficulty**: Intermediate