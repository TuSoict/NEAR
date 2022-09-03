/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */


mod interface_contract_call;
use near_sdk::{Balance, Gas};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, };
use near_sdk::{ near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, env,json_types::U128,PromiseResult};
use interface_contract_call::*;


#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorateKeys {
    DonateKey,
}

pub const XCC_GAS: Gas = Gas(10_000_000_000_000);

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,PanicOnDefault)]
pub struct Contract {
    donation : LookupMap<AccountId, Balance>,
    message_contract_account : AccountId
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { 
            donation : LookupMap::new(StorateKeys::DonateKey),
            message_contract_account : "dev-1662200438205-38513402844676".parse().unwrap()
        }
    }

    #[payable]
    pub fn send_donation(&mut self, receiver: AccountId , title: String, content: String) {
        let fee  = env::attached_deposit();
        let amount = Some(U128::from(fee));
        ext_mail::ext(self.message_contract_account.clone())
        .with_attached_deposit(1)
        .with_static_gas(XCC_GAS)
        .send_email(receiver, title, content,amount ) 
        .then(
            ext_self::ext(env::current_account_id())
            .callback_after_send(env::current_account_id(), fee)
        );
    }


    #[private]
    pub fn callback_after_send(
        &mut self,
        sender_id: AccountId,
        fee: u128,
    ) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
               self.donation.insert(&sender_id, &fee);
            }
            PromiseResult::Failed => env::panic_str("ERR_CALL_FAILED"),
        }
    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
// #[cfg(test)]
// mod tests {
//     use super::*;
// }
