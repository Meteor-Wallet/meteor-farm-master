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
            manager_id: LazyOption::new(StorageKey::ManagerId, None),
            burrowland_contract_id,
            burrowland_config: LazyOption::new(StorageKey::BurrowlandConfig, None),
            base_token_id,
            base_token_pyth_info: LazyOption::new(StorageKey::BaseTokenPythInfo, None),
            token: LazyOption::new(StorageKey::Token, None),
            master_farm_token_id,
            master_farm_pyth_info: LazyOption::new(StorageKey::MasterFarmPythInfo, None),
            slave_farm_token_id,
            slave_farm_pyth_info: LazyOption::new(StorageKey::SlaveFarmPythInfo, None),
            farm_reward_token_id,
            farm_reward_token_pyth_info: LazyOption::new(StorageKey::FarmRewardPythInfo, None),
            slave_contract_id,
            min_master_health_factor,
            min_slave_health_factor,
            target_reserve_ratio,
            min_reserve_ratio,
            base_token_balance: U128(0),
            base_token_reserve: U128(0),
        }
    }

    /// Open to Public
    pub fn update_cache(&self) -> Promise {
        let promises = vec![
            ext_burrowland::ext(self.burrowland_contract_id.clone())
                .with_static_gas(FIVE_TERA_GAS)
                .get_config(),
            ext_burrowland::ext(self.burrowland_contract_id.clone())
                .with_static_gas(FIVE_TERA_GAS)
                .get_token_pyth_info(self.base_token_id.clone()),
            ext_burrowland::ext(self.burrowland_contract_id.clone())
                .with_static_gas(FIVE_TERA_GAS)
                .get_token_pyth_info(self.master_farm_token_id.clone()),
            ext_burrowland::ext(self.burrowland_contract_id.clone())
                .with_static_gas(FIVE_TERA_GAS)
                .get_token_pyth_info(self.slave_farm_token_id.clone()),
            ext_burrowland::ext(self.burrowland_contract_id.clone())
                .with_static_gas(FIVE_TERA_GAS)
                .get_token_pyth_info(self.farm_reward_token_id.clone()),
        ];

        promises
            .into_iter()
            .reduce(|acc, p| acc.and(p))
            .unwrap()
            .then(Promise::new(env::current_account_id()).function_call(
                "update_cache_callback".to_string(),
                near_sdk::serde_json::to_vec(&()).unwrap(),
                NearToken::from_yoctonear(0),
                TWENTY_TERA_GAS,
            ))
    }

    #[private]
    pub fn update_cache_callback(&mut self) {
        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                self.burrowland_config
                    .set(&near_sdk::serde_json::from_slice(&result).ok().unwrap());
            }
            _ => env::panic_str("Failed to get burrowland config"),
        }

        match env::promise_result(1) {
            PromiseResult::Successful(result) => {
                self.base_token_pyth_info
                    .set(&near_sdk::serde_json::from_slice(&result).ok().unwrap());
            }
            _ => env::panic_str("Failed to get base token pyth info"),
        }

        match env::promise_result(2) {
            PromiseResult::Successful(result) => {
                self.master_farm_pyth_info
                    .set(&near_sdk::serde_json::from_slice(&result).ok().unwrap());
            }
            _ => env::panic_str("Failed to get master farm token pyth info"),
        }

        match env::promise_result(3) {
            PromiseResult::Successful(result) => {
                self.slave_farm_pyth_info
                    .set(&near_sdk::serde_json::from_slice(&result).ok().unwrap());
            }
            _ => env::panic_str("Failed to get slave farm token pyth info"),
        }

        match env::promise_result(4) {
            PromiseResult::Successful(result) => {
                self.farm_reward_token_pyth_info
                    .set(&near_sdk::serde_json::from_slice(&result).ok().unwrap());
            }
            _ => env::panic_str("Failed to get farm reward token pyth info"),
        }
    }
}
