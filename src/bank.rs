use crate::account::{ Account};
use crate::transaction::{ Tx, TxType };
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{ Mutex, MutexGuard };

#[derive(Deserialize, Debug)]
pub struct Bank {
    // accounts by client_id
    accounts: Mutex<HashMap<u16, Account>>,
    // transaction vault (tx_id, Tx)
    transactions: HashMap<u32, Tx>,
    // client_id, Vec<Tx>
    disputed_transactions: HashMap<u16, Vec<Tx>>,
}


impl Bank {
    pub fn init() -> Self {
        
        Bank {
            accounts: Mutex::new(HashMap::new()),
            transactions: HashMap::new(),
            disputed_transactions: HashMap::new()
        }
    }

    // create account with client_id if it does not exist already
    pub fn create_account(&mut self, client_id: u16) {
        let mut accounts = self.accounts.lock().unwrap();
        
        if !accounts.contains_key(&client_id) {
            let new_account = Account::new(0.0);
            accounts.insert(client_id, new_account);
        }
    }

    pub fn get_accounts(&self) -> MutexGuard<HashMap<u16, Account>>{
        self.accounts.lock().unwrap()
    }

    pub fn process(&mut self, tx: &Tx) -> Result<(), std::io::Error> {
        // TODO get transaction type and dispatch to type of operation 
        // extract transaction details
        let tx_type = tx.tx_type;
        let tx_id = tx.tx;
        let amount = tx.amount;
        let client_id = tx.client;
        
        // add this tx to transaction vault
        self.transactions.insert(tx_id, tx.to_owned());

        self.create_account(client_id);

        match tx_type {
            TxType::Deposit => {
                // println!("Depositing to client {} amount {}", &client_id, &amount.unwrap());
                let account = self.accounts.lock().unwrap().get(&client_id).unwrap().deposit(amount.unwrap());
                // update account in vault               
                self.accounts.lock().unwrap().insert(client_id, account);
            },

            TxType::Withdrawal => {
                // println!("Withdrawaling from client {} amount {}", &client_id, &amount.unwrap());
                let account = self.accounts.lock().unwrap().get(&client_id).unwrap().withdrawal(amount.unwrap());
                // update account in vault               
                self.accounts.lock().unwrap().insert(client_id, account);
            },

            TxType::Dispute => {
                // if client_id is not yet disputed, create empty vector and push this Tx
                if !self.disputed_transactions.contains_key(&client_id) {
                    self.disputed_transactions.insert(client_id, Vec::new());
                }

                // if client_id is already disputed, push new Tx to existing vector
                self.disputed_transactions.get_mut(&client_id).unwrap().push(tx.to_owned());
                
                // get disputed transaction amount 
                let dt_amount = self.transactions.get(&tx_id).unwrap().amount.unwrap();
                // reflect dispute into account balances
                self.accounts.lock().unwrap().get(&client_id).unwrap().dispute(dt_amount);
            },

            TxType::Resolve => {
                // get disputed transaction amount 
                let dt_amount = self.transactions.get(&tx_id).unwrap().amount.unwrap();
                self.accounts.lock().unwrap().get(&client_id).unwrap().resolve(dt_amount);

                // remove tx from disputed_transactions and client_id if empty (not really necessary)
                let new_size = self.disputed_transactions
                                                        .get_mut(&client_id)
                                                        .unwrap()
                                                        .iter()
                                                        .position(|x| *x == tx.to_owned())
                                                        .unwrap();
                                                        
                // we can also keep client_id in the hashmap in case there are other disputed tx later 
                if new_size == 0 {
                    self.disputed_transactions.remove(&client_id);
                }
            }

            _ => unimplemented!()
        }

        Ok(())
    }
}