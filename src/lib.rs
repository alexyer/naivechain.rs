extern crate crypto;
extern crate time;

pub mod blockchain {
    use std::cell::RefCell;
    use std::fmt;
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use time;

    #[derive(Debug)]
    pub enum BlockchainError {
        InvalidBlockchain,
        InvalidBlock
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Block {
        pub index: u64,
        pub previous_hash: String,
        pub timestamp: u64,
        pub data: String,
        pub hash: String,
    }

    impl Block {
        pub fn new<S>(index: u64, previous_hash: S, timestamp: u64, data: S, hash: S) -> Block where S: Into<String> {
            Block { index: index, previous_hash: previous_hash.into(), timestamp: timestamp, data: data.into(), hash: hash.into() }
        }
    }

    impl fmt::Display for Block {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<Block: {}>", self.index)
        }
    }

    pub struct Blockchain {
        blockchain: RefCell<Vec<Block>>,
    }

    impl Blockchain {
        pub fn new() -> Blockchain {
            Blockchain { blockchain: RefCell::new(vec![Blockchain::generate_genesis_block()]) }
        }

        pub fn len(&self) -> usize {
            self.blockchain.borrow().len()
        }

        fn calculate_hash(index: u64, previous_hash: &String, timestamp: u64, data: &String) -> String {
            let init_str = format!("{}{}{}{}", index, previous_hash, timestamp, data);
            let mut sha256 = Sha256::new();
            sha256.input_str(&init_str);
            sha256.result_str()
        }

        pub fn calculate_hash_for_block(block: &Block) -> String {
            Blockchain::calculate_hash(block.index, &block.previous_hash, block.timestamp, &block.data)
        }

        pub fn generate_genesis_block() -> Block {
            Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                       "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string())
        }

        pub fn generate_new_block<S>(&self, data: S) -> Block where S: Into<String> {
            let new_index = self.latest_block().index + 1;
            let new_timestamp = time::now_utc().to_timespec().sec as u64;
            let data = data.into();

            let new_hash = Blockchain::calculate_hash(new_index, &self.latest_block().hash, new_timestamp, &data);

            Block::new(new_index, self.latest_block().hash.clone(), new_timestamp, data, new_hash)
        }

        pub fn is_valid_new_block(&self, block: &Block) -> bool {
            self.is_valid_block(&block, &self.latest_block())
        }

        fn is_valid_block(&self, block: &Block, prev_block: &Block) -> bool {
            if prev_block.index + 1 != block.index {
                false
            } else if prev_block.hash != block.previous_hash {
                false
            } else if Blockchain::calculate_hash_for_block(&block) != block.hash {
                false
            } else {
                true
            }
        }

        pub fn add_block(&self, block: &Block) -> Result<Block, BlockchainError> {
            match self.is_valid_new_block(block) {
                true => {
                    self.blockchain.borrow_mut().push(block.clone());
                    Ok(self.latest_block())
                }
                false => Err(BlockchainError::InvalidBlock),
            }
        }

        pub fn genesis_block(&self) -> Block {
            self.blockchain.borrow()[0].clone()
        }

        pub fn latest_block(&self) -> Block {
            let chain = self.blockchain.borrow();
            chain[chain.len() - 1].clone()
        }

        pub fn is_valid_chain(&self, other_chain: &Blockchain) -> bool {
            if self.genesis_block() != other_chain.genesis_block() {
                return false
            }

            let chain = other_chain.blockchain.borrow();

            (1..chain.len()).all(|i| { self.is_valid_block(&chain[i], &chain[i - 1]) })
        }

        pub fn replace_chain(&self, other_chain: &Blockchain) -> Result<&Blockchain, BlockchainError> {
            if other_chain.len() > self.len() && self.is_valid_chain(other_chain) {
                let new_blockchain = other_chain.blockchain.borrow();
                let (_, new_blocks_slice) = new_blockchain.as_slice().split_at(self.len());
                self.blockchain.borrow_mut().extend(new_blocks_slice.to_vec().into_iter());
                Ok(&self)
            } else {
                Err(BlockchainError::InvalidBlockchain)
            }
        }
    }
}