extern crate naivechain;

#[cfg(test)]
mod tests {
    use naivechain::blockchain::*;

    #[test]
    fn test_blocks_eq() {
        let block = Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                               "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
        let another_block = Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                                       "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
        assert_eq!(block, another_block);
    }

    #[test]
    fn test_blocks_ne() {
        let block = Block::new(1, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                               "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
        let another_block = Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                                       "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
        assert_ne!(block, another_block);
    }

    #[test]
    fn test_generate_genesis_block() {
        assert_eq!(Blockchain::generate_genesis_block(),
                   Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                              "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string()));
    }

    #[test]
    fn test_blockchain_genesis_block() {
        assert_eq!(Blockchain::new().genesis_block(), Blockchain::generate_genesis_block());
    }

    #[test]
    fn test_blockchain_calculate_hash_for_block() {
        let block = Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
                               "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
        let hash = Blockchain::calculate_hash_for_block(&block);
        assert_eq!(hash, "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
    }

    #[test]
    fn test_blockchain_generate_new_block() {
        let chain = Blockchain::new();
        let new_block = chain.generate_new_block("new-block".to_string());

        assert_eq!(1, new_block.index);
        assert_eq!(chain.genesis_block().hash, new_block.previous_hash);
        assert_eq!("new-block", new_block.data);
        assert_eq!(Blockchain::calculate_hash_for_block(&new_block), new_block.hash);
    }

    #[test]
    fn test_blockchain_is_valid_new_block() {
        let chain = Blockchain::new();
        let new_block = chain.generate_new_block("test".to_string());

        assert!(chain.is_valid_new_block(&new_block));

        let invalid_block = Block{hash: "a".to_string(), ..new_block.clone()};
        assert!(!chain.is_valid_new_block(&invalid_block));

        let invalid_block = Block{index: 10, ..new_block.clone()};
        assert!(!chain.is_valid_new_block(&invalid_block));

        let invalid_block = Block{previous_hash: "dsf".to_string(), ..new_block};
        assert!(!chain.is_valid_new_block(&invalid_block));
    }

    #[test]
    fn test_blockchain_add_block() {
        let chain = Blockchain::new();
        let new_block = chain.generate_new_block("new-block".to_string());

        chain.add_block(&new_block).expect("Cannot add the new block");

        assert_eq!(new_block, chain.latest_block());
    }

    #[test]
    fn test_blockchain_add_block_invalid() {
        let chain = Blockchain::new();
        let mut new_block = chain.generate_new_block("new-block".to_string());
        new_block.hash = "fdf".to_string();

        assert!(chain.add_block(&new_block).is_err());
    }

    #[test]
    fn test_is_valid_chain() {
        let chain = Blockchain::new();

        let other_chain = Blockchain::new();
        other_chain.add_block(&other_chain.generate_new_block("new-block".to_string())).expect("Cannot add the new block");
        other_chain.add_block(&other_chain.generate_new_block("other-new-block".to_string())).expect("Cannot add the new block");

        assert!(chain.is_valid_chain(&other_chain));
    }

    #[test]
    fn replace_chain() {
        let chain = Blockchain::new();

        let other_chain = Blockchain::new();
        other_chain.add_block(&other_chain.generate_new_block("new-block".to_string())).expect("Cannot add the new block");
        other_chain.add_block(&other_chain.generate_new_block("other-new-block".to_string())).expect("Cannot add the new block");

        chain.replace_chain(&other_chain).expect("Cannot replace the chain");

        assert_eq!(chain.latest_block().data, "other-new-block".to_string());
        assert_eq!(chain.len(), 3);
    }
}