use chrono::{DateTime, Utc};
use sha2::{Digest, Sha512};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Block {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub previous_block_hash: Option<String>,
    pub hash: String,
}

impl Block {
    pub fn new(data: String, previous_block_hash: Option<String>) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let hash = Self::generate_hash(&id, &timestamp, &data, &previous_block_hash);
        Self {
            id: id,
            timestamp: timestamp,
            data: data,
            previous_block_hash: previous_block_hash,
            hash: hash,
        }
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn generate_hash(
        id: &Uuid,
        timestamp: &DateTime<Utc>,
        data: &String,
        previous_block_hash: &Option<String>,
    ) -> String {
        let mut hasher = Sha512::new();
        let mut str = id.to_string();
        str.push_str(&timestamp.to_string());
        str.push_str(&data);
        str.push_str(&previous_block_hash.clone().unwrap_or("".to_string()));

        hasher.update(str);

        format!("{:x}", hasher.finalize())
    }

    pub fn generate_hash_from_block(block: &Block) -> String {
        Self::generate_hash(
            &block.id,
            &block.timestamp,
            &block.data,
            &block.previous_block_hash,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_hash_same_value() {
        // create a block (thus creating hash), then create hash from block
        // assert that both hash values are the same
        let block = Block::new("data".to_string(), None);
        let new_hash = Block::generate_hash_from_block(&block);

        assert_eq!(new_hash, block.get_hash());
    }
}
