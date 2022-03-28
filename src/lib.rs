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
    pub fn new(
        owner_id: ValidAccountId,
        total_supply: U128,
        spec: String,
        name: String,
        symbol: String,
        icon: Option<String>,
        reference: Option<String>,
        reference_hash: Option<Base64VecU8>,
        decimals: u8,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let token = FungibleToken {
            accounts: LookupMap::new(b"a".to_vec()),
            total_supply: Balance::from(total_supply),
            account_storage_usage: 0,
        };

        let mut this = Contract {
            token: FungibleToken::from(token),
            ft_metadata: FungibleTokenMetadata {
                spec,
                name,
                symbol,
                icon,
                reference,
                reference_hash,
                decimals,
            },
        };

        // Make owner have total supply
        let total_supply_u128: u128 = total_supply.into();
        this.token
            .accounts
            .insert(&owner_id.as_ref(), &total_supply_u128);
        this
    }
}

#[near_bindgen]
impl FungibleTokenCore for Contract {
    fn ft_transfer(&mut self, receiver_id: ValidAccountId, amount: U128, memo: Option<String>) {
        let sender_id = env::predecessor_account_id();
        assert_one_yocto();
        let amount = amount.into();
        self.token
            .internal_transfer(&sender_id, receiver_id.as_ref(), amount, memo);
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
        self.token.total_supply.into()
    }

    fn ft_balance_of(&self, account_id: ValidAccountId) -> U128 {
        self.token
            .accounts
            .get(account_id.as_ref())
            .unwrap_or(0)
            .into()
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

        fn dex() -> ValidAccountId {
            ValidAccountId::try_from("dex.near").unwrap()
        }

        let contract = Contract::new(
            dex(),
            U128::from(1_000_000_000_000_000),
            String::from("0.1.0"),
            String::from("NEAR Test Token"),
            String::from("TEST"),
            None,
            None,
            None,
            24,
        );
    }
}
