// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

// Implement the contract structure
#[near]
impl Contract {}
