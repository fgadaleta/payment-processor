use crate::account::Account;
use crate::transaction::{ Tx, TxType };
use serde::{ Deserialize };
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct Bank {
    // accounts by client_id
    accounts: HashMap<u16, Account>,
    // transaction vault (tx_id, Tx)
    transactions: HashMap<u32, Tx>,
    // client_id, Vec<Tx>
    disputed_transactions: HashMap<u16, Vec<Tx>>,
}


impl Bank {
    pub fn init() -> Self {
        
        Bank {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            disputed_transactions: HashMap::new()
        }
    }

    pub fn open_account(&mut self) {
        let last_client_id = self.accounts
                                        .iter()
                                        .max_by(|a, b| a.0.cmp(&b.0))
                                        .map(|(k, _v)| k)
                                        .unwrap();
        let client_id = *last_client_id + 1;
        let account = Account::new(0.0);
        self.accounts.insert(client_id, account);
    }

    // create account with client_id if it does not exist already
    // Return account 
    pub fn create_account(&mut self, client_id: u16) -> Account {

        if self.accounts.contains_key(&client_id) {
            self.accounts.get(&client_id).unwrap().to_owned()
        }

        else {
            let new_account = Account::new(0.0);
            self.accounts.insert(client_id, new_account);
            new_account
        }
    }

    // loads transactions from a pool and process in order of arrival
    // pub fn load_transactions(&self, tx_pool: Vec<Tx>) {
    //     for tx in tx_pool.iter() {
    //         let res = self.process(tx.clone()).unwrap();
    //     }
    // }

    pub fn process(&mut self, tx: &Tx) -> Result<(), std::io::Error> {
        // TODO get transaction type and dispatch to type of operation 
        // extract transaction details
        let tx_type = tx.tx_type;
        let tx_id = tx.tx_id;
        let amount = tx.amount;
        let client_id = tx.client_id;
        
        // add this tx to transaction vault
        self.transactions.insert(tx_id, tx.to_owned());
    
        // get account from client_id
        let account = match self.accounts.get(&client_id) {
            Some(acc) => {
                // TODO account with client_id exists, process transaction
                acc.to_owned()
            },

            None => {
                // cannot fail by design
                self.create_account(client_id.to_owned())
            }
        };

        match tx_type {
            TxType::Deposit => {
                account.deposit(amount.unwrap());    
            },

            TxType::Withdrawal => {
                let _status = account.withdrawal(amount.unwrap());
                // TODO log insufficient funds message
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
                account.dispute(dt_amount);
            },

            TxType::Resolve => {
                // get disputed transaction amount 
                let dt_amount = self.transactions.get(&tx_id).unwrap().amount.unwrap();
                account.resolve(dt_amount);

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