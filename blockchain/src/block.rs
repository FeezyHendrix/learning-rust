use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
  pub index: u32,
  pub timestamp: u128,
  pub hash: Hash,
  pub prev_block_hash: Hash,
  pub nonce: u64,
  pub payload: String,
}

impl Debug for Block {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "Block")
  }
}

impl Block {
  pub fn new(
    index: u32,
    timestamp: u128,
    prev_block_hash: Hash,
    nonce: u64,
    payload: String,
  ) -> Self {
    Block {
      index,
      timestamp,
      prev_block_hash,
      nonce,
      payload,
      hash: vec![0; 32]
    }
  }
}
