use near_contract_standards::fungible_token::core::FungibleTokenCore;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U128};
use near_sdk::{
    assert_one_yocto, env, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue,
    StorageUsage,
};
use std::num::ParseIntError;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub token: FungibleToken,

    pub ft_metadata: FungibleTokenMetadata,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Contract {
            token: FungibleToken::new(b"t".to_vec()),
            ft_metadata: FungibleTokenMetadata {
                spec: String::default(),
                name: String::default(),
                symbol: String::default(),
                icon: None,
                reference: Option::from(String::default()),
                reference_hash: Option::from(Base64VecU8(vec![])),
                decimals: 0,
            },
        }
    }
}

#[near_bindgen]
impl FungibleTokenCore for Contract {
    fn ft_transfer(&mut self, receiver_id: ValidAccountId, amount: U128, memo: Option<String>) {
        todo!()
    }

    fn ft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        todo!()
    }

    fn ft_total_supply(&self) -> U128 {
        todo!()
    }

    fn ft_balance_of(&self, account_id: ValidAccountId) -> U128 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{test_utils, VMContext};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new().is_view(is_view).build()
    }

    #[test]
    fn test_new() {
        let context = get_context(false);
        testing_env!(context);
        let _contract = Contract::new();
    }
}
