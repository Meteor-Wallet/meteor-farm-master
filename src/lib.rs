use std::hash::Hash;

use burrow_token_pyth_info::BurrowTokenPythInfo;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, json_types::U128, json_types::U64, near, AccountId};

mod burrow_token_pyth_info;
mod permission;
mod setup;
// Define the contract structure
#[near(contract_state)]
pub struct MeteorFarmMasterContract {
    /**
     * Management Accounts
     *
     * owner_id: The account ID of the owner of the contract, this account's private key will be stored in cold wallet
     * and all transactions signed by this account should be trusted.
     * manager_id: The account ID of the manager of the contract, this account's private key will be stored in server running
     * arbitrage bot, this account SHALL NOT be permitted to run any transactions that cause permanent loss of funds.
     */
    owner_id: AccountId,
    manager_id: Option<AccountId>,
    /**
     * Burrowland Contract
     *
     * burrowland_contract_id: The account ID of the burrowland contract, it is an underlying DEX contract that we will be
     * using to stake and borrow tokens.
     * burrowland_config: Most of the burrowland config won't be changed so frequently, so we decide to store a cached
     * config in the contract.
     */
    burrowland_contract_id: AccountId,
    burrowland_config: Option<BurrowConfig>,
    /**
     * Farm Configuration
     *
     * base_token_id: The account ID of the base token contract, this is the token that user will deposit to us.
     * xxxx_token_pyth_info: The pyth info of the token, cached from burrowland contract, this is used to get the price of the token
     * in USD.
     * token: The fungible token contract, this is the liquid staking token that we will give to user.
     * master_farm_token_id: The account ID of the master farm token contract, this is the token that slave contract will borrow
     * and master contract will stake to get the farm bonus.
     * slave_farm_token_id: The account ID of the slave farm token contract, this is the token that master contract will borrow
     * and slave contract will stake to get the farm bonus.
     * farm_reward_token_id: The account ID of the reward token contract, this is the token that we will get from the farm bonus.
     */
    base_token_id: AccountId,
    base_token_pyth_info: Option<BurrowTokenPythInfo>,
    token: Option<FungibleToken>,
    master_farm_token_id: AccountId,
    master_farm_pyth_info: Option<BurrowTokenPythInfo>,
    slave_farm_token_id: AccountId,
    slave_farm_pyth_info: Option<BurrowTokenPythInfo>,
    farm_reward_token_id: AccountId,
    farm_reward_token_pyth_info: Option<BurrowTokenPythInfo>,
    /**
     * Contract Configuration
     *
     * master_contract_id: This contract is the master contract, so we don't need to set this.
     * slave_contract_id: The account ID of the slave contract, the contract will be used to stake and borrow tokens, we need two
     * contracts because burrowland only gives farm bonus based on net liquidity provided.
     * min_master_health_factor: The minimum health factor of the master contract in burrowland, we should never borrow more than
     * this amount, otherwise we will be facing liquidation risk.
     * min_slave_health_factor: The minimum health factor of the slave contract in burrowland, we should never borrow more than
     * this amount, otherwise we will be facing liquidation risk.
     * target_reserve_ratio: The target reserve ratio of how much base_token we should keep in the contract for user's withdrawal
     * purpose.
     * min_reserve_ratio: The minimum reserve ratio of how much base_token we should keep in the contract for user's withdrawal purpose
     */
    slave_contract_id: AccountId,
    min_master_health_factor: U64,
    min_slave_health_factor: U64,
    target_reserve_ratio: U64,
    min_reserve_ratio: U64,
    /**
     * Contract State
     */
    base_token_balance: U128,
    base_token_reserve: U128,
}

#[derive(BorshDeserialize, BorshSerialize, BorshSchema, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BurrowConfig {
    /// The account ID of the pyth oracle contract
    pub pyth_oracle_account_id: String,
    /// The account ID of the ref_exchange contract
    pub ref_exchange_id: String,
    /// The account ID of the booster token contract.
    pub booster_token_id: String,
    /// The number of decimals of the booster fungible token.
    pub booster_decimals: u8,
}
