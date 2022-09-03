use near_sdk::{ext_contract, AccountId};
use near_sdk::json_types::U128;

#[ext_contract(ext_self)]
pub trait ContractActions {
    fn callback_after_send(
        sender_id: AccountId,
        amount: u128
    );
}

#[ext_contract(ext_mail)]
pub trait Mail {
     fn send_email(&mut self, receiver:AccountId, title: String, content: String, amount: Option<U128>);
}