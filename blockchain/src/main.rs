use blockchainlib::*;

fn main() {
    let block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Intial block".to_owned()
    );
    print!("{:?}", &block);
}
