use blockchainlib::*;
fn main() {
    let block = Block::new(0, 0, vec![0; 16], 0, "Debug".to_string());
    println!("Un-Hashed Block: {:?}", block);
    //Doesn't have Display trait implemented as it's a Vec<u8>,
    // therefor we use debug output {:?}
    println!("Block Hash: {:?}", block.hash());
}
