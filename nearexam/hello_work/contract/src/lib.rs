/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

mod message;
mod storage_imp;

use message::Email;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet, UnorderedMap};
use near_sdk::{ near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,assert_one_yocto, env,json_types::U128, Promise};
use storage_imp::*;

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";
pub type  EmailId = u128;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,PanicOnDefault)]
pub struct Contract {
    message: String,
    senders: LookupMap<AccountId, UnorderedSet<EmailId>>,
    receivers: LookupMap<AccountId,UnorderedSet<EmailId>>,
    emails: UnorderedMap<EmailId, Email>,
    email_count : u128,
    accounts: LookupMap<AccountId, VAccount>,
    donation_account: AccountId,

}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorateKeys {
    SenderKey,
    ReceiverKey,
    Email,
    Account,
    SenderMail {emailId:EmailId},
    ReceiverMail {emailId:EmailId},
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { 
            message: DEFAULT_MESSAGE.to_string(),
            senders : LookupMap::new (StorateKeys::SenderKey),
            receivers: LookupMap::new(StorateKeys::ReceiverKey),
            emails: UnorderedMap::new(StorateKeys::Email),
            email_count : 0,
            accounts: LookupMap::new(StorateKeys::Account),
            donation_account : "tutmt.testnet".parse().unwrap()
        }
    }



    // Public method - returns th&e greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }


    // send message 
    #[payable]
    pub fn send_email(&mut self, receiver:AccountId, title: String, content: String, amount: Option<U128>) {
        assert_one_yocto();
        let sender = env::predecessor_account_id();

        let mut vaccount = self.accounts.get(&sender).unwrap();
        vaccount.used += STORAGE_PER_MAIL * env::storage_byte_cost();
        self.accounts.insert(&sender, &vaccount);


        let current_account = self.email_count;
        let email_id = self.email_count;
        self.email_count = current_account + 1;
        let timestamp = env::block_timestamp();
        let sender = env::predecessor_account_id();

        let title_email = title;
        if !Some(amount).is_none()  {            
            let fee : U128 = amount.unwrap();
            Promise::new(self.donation_account.clone()).transfer(fee.0);
        }
        
        let email = Email {
            title_email,
            content,
            timestamp,
            amount,
        };

       

        self.emails.insert(&current_account, &email);
        if let Some(mut sender_vec) = self.senders.get(&sender) {
            sender_vec.insert(&email_id) ;
            self.senders.insert(&sender, &sender_vec);
        } else {

            let  new_vec_sender :UnorderedSet<EmailId> = UnorderedSet::new(StorateKeys::SenderMail{emailId: email_id});
            self.senders.insert(&sender, &new_vec_sender);
        }
        if let Some(mut receiver_vec) = self.receivers.get(&receiver) {
            receiver_vec.insert(&email_id) ;
            self.receivers.insert(&sender, &receiver_vec);
        } else {

            let  new_vec_receiver :UnorderedSet<EmailId> = UnorderedSet::new(StorateKeys::ReceiverMail{emailId: email_id});
            self.receivers.insert(&sender, &new_vec_receiver);
        }

    }

    pub fn get_email(&self, email_id: U128) -> Email {
        let real_email_id: EmailId = email_id.0;
        self.emails.get(&real_email_id).unwrap()
    }

    pub fn delete_mail(&mut self, email_id: U128) {
        let real_email_id: EmailId = email_id.0;
        let sender = env::predecessor_account_id();
        assert!(!self.senders.get(&sender).unwrap().contains(&real_email_id), "Caller is not sender");
        self.emails.remove(&real_email_id);
    }

    pub fn mail_exist(&self) -> u64 {
        self.emails.keys_as_vector().len()
    }

    pub fn get_mail_receive(&self, receiver: AccountId) -> Vec<Email> {
        let mut email_vec:Vec<Email> = Vec::new();
        if let Some(receiver_vec) = self.receivers.get(&receiver) {
            for index in receiver_vec.iter() {
                let mail = self.emails.get(&index).unwrap();
                email_vec.push(mail);
            }
        }
        return email_vec;
    }

    pub fn get_mail_send(&self, sender: AccountId) -> Vec<Email> {
        let mut email_vec:Vec<Email> = Vec::new();
        if let Some(sender_vec) = self.senders.get(&sender) {
            for index in sender_vec.iter() {
                let mail = self.emails.get(&index).unwrap();
                email_vec.push(mail);
            }
        }
        return email_vec;
    }

    pub fn get_mail_receive_num(&self, receiver: AccountId) -> u64 {
        if let Some(receiver_vec) = self.receivers.get(&receiver) {
           return receiver_vec.len();
        }
        0
    }

    pub fn get_mail_send_num(&self, sender: AccountId) -> u64 {
        if let Some(sender_vec) = self.senders.get(&sender) {
           return sender_vec.len();
        }
        0
    }

    pub fn mail_delete(&self) -> U128 {
        let mail_exist:u128 = self.emails.keys_as_vector().len().into();
        U128(self.email_count - mail_exist)
    }


}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        // let mut contract = Contract::default();
        // contract.set_greeting("howdy".to_string());
        // assert_eq!(
        //     contract.get_greeting(),
        //     "howdy".to_string()
        // );
    }
}
