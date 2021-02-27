use serde::{Serialize, Deserialize};

// type of transaction
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
// transaction struct
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Tx {
    pub tx_id: u32,
    pub tx_type: TxType,
    pub client_id: u16,
    pub amount: Option<f32>,
}

impl Tx {
    pub fn new(tx_id: u32, tx_type: TxType, client_id: u16, amount: Option<f32>) -> Self {
        // TODO validate arguments if necessary  

        Tx {
            tx_id,
            tx_type,
            client_id,
            amount
        }
    }




}