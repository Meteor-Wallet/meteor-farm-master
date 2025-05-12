use crate::*;

#[near]
impl MeteorFarmMasterContract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        burrowland_contract_id: AccountId,
        base_token_id: AccountId,
        master_farm_token_id: AccountId,
        slave_farm_token_id: AccountId,
        farm_reward_token_id: AccountId,
        slave_contract_id: AccountId,
        min_master_health_factor: U64,
        min_slave_health_factor: U64,
        target_reserve_ratio: U64,
        min_reserve_ratio: U64,
    ) -> Self {
        assert!(!env::state_exists(), "Contract already initialized");

        Self {
            owner_id,
            manager_id: None,
            burrowland_contract_id,
            burrowland_config: None,
            base_token_id,
            base_token_pyth_info: None,
            token: None,
            master_farm_token_id,
            master_farm_pyth_info: None,
            slave_farm_token_id,
            slave_farm_pyth_info: None,
            farm_reward_token_id,
            farm_reward_token_pyth_info: None,
            slave_contract_id,
            min_master_health_factor,
            min_slave_health_factor,
            target_reserve_ratio,
            min_reserve_ratio,
            base_token_balance: U128(0),
            base_token_reserve: U128(0),
        }
    }
}
