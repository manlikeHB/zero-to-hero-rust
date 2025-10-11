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
