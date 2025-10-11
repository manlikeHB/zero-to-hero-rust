use crate::block::Block;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
        }
    }

    pub fn chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Blockchain should never be empty")
    }

    pub fn add_block(&mut self, data: String) {
        let latest_block = self.get_latest_block();
        let new_block = Block::new(
            latest_block.index() + 1,
            Utc::now().to_string(),
            data,
            latest_block.hash().to_string(),
        );
        self.chain.push(new_block);
    }

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

    #[cfg(test)]
    pub fn temper_with_block(&mut self, index: usize, prev_hash: String) {
        if let Some(block) = self.chain.get_mut(index) {
            *block = Block::new(
                index as u64,
                block.timestamp().to_string(),
                block.data().to_string(),
                prev_hash,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_blockchain() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("first tx".to_string());
        blockchain.add_block("second tx".to_string());
        blockchain.add_block("third tx".to_string());

        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_tampered_genesis_link() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("first tx".to_string());

        // Break link between genesis and block 1
        blockchain.temper_with_block(1, "fake_genesis_hash".to_string());

        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_tampered_middle_link() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("first tx".to_string());
        blockchain.add_block("second tx".to_string());
        blockchain.add_block("third tx".to_string());

        // Break link between block 1 and block 2
        blockchain.temper_with_block(2, "fake_previous_hash".to_string());

        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_multiple_blocks() {
        let mut blockchain = Blockchain::new();

        for i in 1..=10 {
            blockchain.add_block(format!("Transaction {}", i));
        }

        assert_eq!(blockchain.chain().len(), 11); // genesis + 10
        assert!(blockchain.is_valid());

        // Tamper somewhere in the middle
        blockchain.temper_with_block(5, "broken_link".to_string());
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_chain_integrity_after_valid_additions() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block("tx1".to_string());
        assert!(blockchain.is_valid());

        blockchain.add_block("tx2".to_string());
        assert!(blockchain.is_valid());

        blockchain.add_block("tx3".to_string());
        assert!(blockchain.is_valid());

        // Each addition maintains validity
        for i in 1..blockchain.chain().len() {
            let current = &blockchain.chain()[i];
            let previous = &blockchain.chain()[i - 1];
            assert_eq!(current.previous_hash(), previous.hash());
        }
    }
}
