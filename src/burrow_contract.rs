use crate::*;

#[ext_contract(ext_burrowland)]
pub trait _BurrowlandContract {
    fn get_config(&self) -> BurrowConfig;
    fn get_token_pyth_info(&self, token_id: AccountId) -> Option<BurrowTokenPythInfo>;
}
