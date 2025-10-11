use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    pub fn genesis() -> Self {
        Self::new(
            0,
            "2025-01-01 00:00:00 UTC".to_string(),
            "Genesis Block".to_string(),
            "genesis".to_string(),
        )
    }

    pub fn new(index: u64, timestamp: String, data: String, previous_hash: String) -> Self {
        let hash = Self::calculate_hash_with_parts(index, &timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn previous_hash(&self) -> &str {
        &self.previous_hash
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn calculate_hash(&self) -> String {
        Self::calculate_hash_with_parts(
            self.index(),
            self.timestamp(),
            self.data(),
            self.previous_hash(),
        )
    }

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
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block #{}\n\
             ├─ Timestamp: {}\n\
             ├─ Data: {}\n\
             ├─ Previous Hash: {}\n\
             └─ Hash: {}",
            self.index, self.timestamp, self.data, self.previous_hash, self.hash
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_creates_valid_hash() {
        let block = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Test data".to_string(),
            "previous_hash_123".to_string(),
        );

        // Hash should be calculated correctly
        assert_eq!(block.hash(), block.calculate_hash());
        assert_eq!(block.index(), 1);
        assert_eq!(block.data(), "Test data");
        assert_eq!(block.previous_hash(), "previous_hash_123");
    }

    #[test]
    fn test_genesis_block_properties() {
        let genesis = Block::genesis();

        assert_eq!(genesis.index(), 0);
        assert_eq!(genesis.timestamp(), "2025-01-01 00:00:00 UTC");
        assert_eq!(genesis.data(), "Genesis Block");
        assert_eq!(genesis.previous_hash(), "genesis");

        // Genesis hash should be deterministic
        let genesis2 = Block::genesis();
        assert_eq!(genesis.hash(), genesis2.hash());
    }

    #[test]
    fn test_hash_is_deterministic() {
        let block1 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash".to_string(),
        );

        let block2 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash".to_string(),
        );

        // Same inputs should produce same hash
        assert_eq!(block1.hash(), block2.hash());
    }

    #[test]
    fn test_hash_changes_with_data() {
        let block1 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Data A".to_string(),
            "prev_hash".to_string(),
        );

        let block2 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Data B".to_string(), // Different data
            "prev_hash".to_string(),
        );

        // Different data should produce different hash
        assert_ne!(block1.hash(), block2.hash());
    }

    #[test]
    fn test_hash_changes_with_timestamp() {
        let block1 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash".to_string(),
        );

        let block2 = Block::new(
            1,
            "2025-10-10 13:00:00 UTC".to_string(), // Different timestamp
            "Same data".to_string(),
            "prev_hash".to_string(),
        );

        assert_ne!(block1.hash(), block2.hash());
    }

    #[test]
    fn test_hash_changes_with_index() {
        let block1 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash".to_string(),
        );

        let block2 = Block::new(
            2, // Different index
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash".to_string(),
        );

        assert_ne!(block1.hash(), block2.hash());
    }

    #[test]
    fn test_hash_changes_with_previous_hash() {
        let block1 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash_A".to_string(),
        );

        let block2 = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Same data".to_string(),
            "prev_hash_B".to_string(), // Different previous hash
        );

        assert_ne!(block1.hash(), block2.hash());
    }

    #[test]
    fn test_calculate_hash_matches_stored_hash() {
        let block = Block::new(
            5,
            "2025-10-10 12:00:00 UTC".to_string(),
            "Transaction data".to_string(),
            "abc123".to_string(),
        );

        // Recalculated hash should match stored hash
        assert_eq!(block.hash(), block.calculate_hash());
    }

    #[test]
    fn test_calculate_hash_with_parts_static_function() {
        let hash1 =
            Block::calculate_hash_with_parts(1, "2025-10-10 12:00:00 UTC", "Data", "prev_hash");

        let hash2 =
            Block::calculate_hash_with_parts(1, "2025-10-10 12:00:00 UTC", "Data", "prev_hash");

        // Same inputs should produce same hash
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_getters_return_correct_values() {
        let block = Block::new(
            42,
            "2025-12-25 00:00:00 UTC".to_string(),
            "Christmas transaction".to_string(),
            "santa_hash".to_string(),
        );

        assert_eq!(block.index(), 42);
        assert_eq!(block.timestamp(), "2025-12-25 00:00:00 UTC");
        assert_eq!(block.data(), "Christmas transaction");
        assert_eq!(block.previous_hash(), "santa_hash");
        assert!(!block.hash().is_empty());
    }

    #[test]
    fn test_empty_data_still_produces_valid_hash() {
        let block = Block::new(
            1,
            "2025-10-10 12:00:00 UTC".to_string(),
            String::new(), // Empty data
            "prev_hash".to_string(),
        );

        assert_eq!(block.data(), "");
        assert_eq!(block.hash().len(), 64);
        assert_eq!(block.hash(), block.calculate_hash());
    }
}
