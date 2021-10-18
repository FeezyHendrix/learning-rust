use std::fmt::{self, Debug, Formatter };
use super::*;

pub struct Block {
  pub index: u32,
  pub timestamp: u128,
  pub hash: Hash,
  pub prev_block_hash: Hash,
  pub nonce: u64,
  pub payload: String,
}

impl Debug for Block {
  fn fmt (&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "Block")
  }
}

impl Block {

}
