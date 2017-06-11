extern crate naivechain;

use naivechain::blockchain::Block;

fn main() {
    let block = Block::new(0, "0".to_string(), 1465154705, "my genesis block!!".to_string(),
    "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string());
    println!("{}", block);
}
