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
    // making this humongous to avoid needing to do things like change time or use multiple values
    nonce: u64,
}

struct MiningResult {
    nonce: u64,
    hash: String,
}

impl Block {
    pub fn new(data: String, previous_block_hash: Option<String>, difficulty: u32) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let mining_result =
            Self::mine_and_return_hash(&id, &timestamp, &data, &previous_block_hash, difficulty);
        Self {
            id: id,
            timestamp: timestamp,
            data: data,
            previous_block_hash: previous_block_hash,
            hash: mining_result.hash,
            nonce: mining_result.nonce,
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
        nonce: &u64,
    ) -> String {
        let mut hasher = Sha512::new();
        let mut str = id.to_string();
        str.push_str(&timestamp.to_string());
        str.push_str(&data);
        str.push_str(&previous_block_hash.clone().unwrap_or("".to_string()));
        str.push_str(&nonce.to_string());

        hasher.update(str);

        format!("{:x}", hasher.finalize())
    }

    fn mine_and_return_hash(
        id: &Uuid,
        timestamp: &DateTime<Utc>,
        data: &String,
        previous_block_hash: &Option<String>,
        difficulty: u32,
    ) -> MiningResult {
        println!("Hashing new block");

        let mut nonce: u64 = 0;
        loop {
            // hashing succeeds when first n values are 0, where n is difficulty
            let hash = Self::generate_hash(id, timestamp, data, previous_block_hash, &nonce);
            if (&hash[..difficulty as usize]).chars().all(|c| c == '0') {
                println!("Hash hit! Hash value: {}", hash);
                return MiningResult {
                    nonce: nonce,
                    hash: hash,
                };
            }

            if nonce >= u64::MAX {
                panic!("Reached maximum u64 nonce size");
            }
            nonce += 1;
        }
    }

    pub fn generate_hash_from_block(block: &Block) -> String {
        Self::generate_hash(
            &block.id,
            &block.timestamp,
            &block.data,
            &block.previous_block_hash,
            &block.nonce,
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
        let block = Block::new("data".to_string(), None, 1);
        let new_hash = Block::generate_hash_from_block(&block);

        assert_eq!(new_hash, block.get_hash());
    }
}
