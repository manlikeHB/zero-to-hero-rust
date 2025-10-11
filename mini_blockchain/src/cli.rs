use crate::blckchain::Blockchain;
use std::io;

pub fn get_input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read command");
    buf.trim().to_string()
}

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

fn handle_input(blockchain: &mut Blockchain) -> bool {
    match get_input().parse::<usize>() {
        Ok(1) => {
            add_transaction(blockchain);
            true
        }
        Ok(2) => {
            view_blockchain(blockchain);
            true
        }
        Ok(3) => {
            validate_chain(blockchain);
            true
        }
        Ok(4) => {
            println!("Exiting...");
            false
        }
        _ => {
            println!("Invalid option, please try again.");
            true
        }
    }
}

fn print_menu() {
    println!("1. Add transaction");
    println!("2. View chain");
    println!("3. Validate chain");
    println!("4. Exit");
    println!("Choose an option: ");
}

fn add_transaction(blockchain: &mut Blockchain) {
    println!("Enter transaction data:");

    let data = get_input();

    if data.is_empty() {
        println!("Transaction data cannot be empty.");
    } else {
        blockchain.add_block(data);
        println!("Transaction added.");
    }
}

fn view_blockchain(blockchain: &Blockchain) {
    println!("\n=== Full Blockchain ===\n");
    for block in blockchain.chain() {
        println!("{}\n", block);
    }
    println!("\nTotal blocks: {}", blockchain.chain().len());
}

fn validate_chain(blockchain: &Blockchain) {
    if blockchain.is_valid() {
        println!("✓ Blockchain is VALID! All blocks verified.");
    } else {
        println!("✗ WARNING: Blockchain is INVALID! Tampering detected!");
    }
}
