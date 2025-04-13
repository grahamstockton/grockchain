use crate::block::Block;

#[derive(Clone, Debug)]
pub struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new(data: String) -> Self {
        Self {
            chain: vec![Self::create_genesis_block(data)],
        }
    }

    pub fn get_latest_block(&self) -> Block {
        self.chain
            .last()
            .expect("BlockChain with 0 elements should be impossible")
            .clone()
    }

    pub fn add_block(&mut self, data: String) {
        let block = Block::new(data, Some(self.get_latest_block().get_hash()));
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for idx in 1..self.chain.len() {
            let hash = self.chain[idx].hash.clone();
            let previous_hash = self.chain[idx].previous_block_hash.clone().unwrap();

            // check hash matches fields
            if !(hash == Block::generate_hash_from_block(&self.chain[idx])) {
                return false;
            }

            // check hash matches previous
            if !(previous_hash == self.chain[idx - 1].hash) {
                return false;
            }
        }

        true
    }

    fn create_genesis_block(data: String) -> Block {
        Block::new(data, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let data_1 = "hello world".to_string();
        let data_2 = "second data".to_string();
        let data_3 = "third data".to_string();
        let mut chain = BlockChain::new(data_1.clone());
        chain.add_block(data_2.clone());
        chain.add_block(data_3.clone());
        assert!(chain.chain.len() == 3);
        assert!(chain.chain[0].data == data_1);
        assert!(chain.chain[1].data == data_2);
        assert!(chain.chain[2].data == data_3);
    }

    #[test]
    fn test_chain_validation_value_changed() {
        let data_1 = "hello world".to_string();
        let data_2 = "second data".to_string();
        let data_3 = "third data".to_string();
        let mut chain = BlockChain::new(data_1.clone());
        chain.add_block(data_2.clone());
        chain.add_block(data_3.clone());

        assert!(chain.is_valid());

        // change a value in the chain and assert the chain is no longer valid
        chain.chain[1].data = "new_data".to_string();
        assert!(!chain.is_valid());
    }

    #[test]
    fn test_chain_validation_hash_changed() {
        let data_1 = "hello world".to_string();
        let data_2 = "second data".to_string();
        let data_3 = "third data".to_string();
        let mut chain = BlockChain::new(data_1.clone());
        chain.add_block(data_2.clone());
        chain.add_block(data_3.clone());

        assert!(chain.is_valid());

        // change a value and hash and assert the chain is no longer valid
        chain.chain[1].data = "new_data".to_string();
        chain.chain[1].hash = Block::generate_hash_from_block(&chain.chain[1]);
        assert!(!chain.is_valid());
    }
}
