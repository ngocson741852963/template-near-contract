#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  count: u32,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
  fn default() -> Self {
    Self { count: 0 }
  }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  // Public method - Get the current count
  pub fn get_number(&self) -> u32 {
    self.count
  }

  // Call this method to increment the count by a given number
  pub fn plus(&mut self, number: u32) {
    self.count += number;
  }

  // Private method
  #[private]
  fn plus_one(&mut self) {
    self.count += 1;
  }
}
