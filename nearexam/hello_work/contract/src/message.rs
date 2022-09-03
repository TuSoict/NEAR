use near_sdk::borsh::{self,BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Email {
    pub title_email: String,
    pub content:String,
    pub timestamp:u64,
    pub amount:Option<U128>
}