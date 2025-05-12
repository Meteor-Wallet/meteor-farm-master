use crate::*;

impl MeteorFarmMasterContract {
    pub fn internat_is_owner(&self, account_id: &AccountId) -> bool {
        self.owner_id == *account_id
    }

    pub fn internal_is_manager(&self, account_id: &AccountId) -> bool {
        self.manager_id.is_some() && *self.manager_id.get().unwrap() == *account_id
    }

    pub fn internal_is_owner_or_manager(&self, account_id: &AccountId) -> bool {
        self.internat_is_owner(account_id) || self.internal_is_manager(account_id)
    }

    pub fn internal_assert_owner(&self) {
        assert!(
            self.internat_is_owner(&env::predecessor_account_id()),
            "Only owner can call this method"
        );
    }

    pub fn internal_assert_manager(&self) {
        assert!(
            self.internal_is_manager(&env::predecessor_account_id()),
            "Only manager can call this method"
        );
    }

    pub fn internal_assert_owner_or_manager(&self) {
        assert!(
            self.internal_is_owner_or_manager(&env::predecessor_account_id()),
            "Only owner or manager can call this method"
        );
    }
}
