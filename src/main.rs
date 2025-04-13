use blockchain::BlockChain;

pub mod block;
pub mod blockchain;
fn main() {
    let data_1 = "hello world".to_string();
    let data_2 = "second data".to_string();
    let data_3 = "third data".to_string();
    let mut chain = BlockChain::new(data_1.clone());
    chain.add_block(data_2.clone());
    chain.add_block(data_3.clone());
    println!("{:?}", chain);
}
