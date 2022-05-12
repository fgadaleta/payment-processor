use serde::{Serialize, Deserialize};

// type of transaction
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
// transaction struct
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Tx {
    pub tx_type: TxType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

impl Tx {
    pub fn new(tx: u32, tx_type: TxType, client: u16, amount: Option<f32>) -> Self {
        // TODO validate arguments if necessary

        Tx {
            tx_type,
            client,
            tx,
            amount
        }
    }
}