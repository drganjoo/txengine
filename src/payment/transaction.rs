pub type ClientId = u16;
pub type TransactionId = u32;

use super::amount::Amount;

#[derive(Debug)]
pub enum TransactionType {
    Deposit { amount: Amount },
    Withdrawal { amount: Amount },
    Dispute, 
    Resolve,
    ChargeBack,
}

#[derive(Debug)]
pub struct Transaction {
    pub client : ClientId,
    pub tx : TransactionId,
    pub txn_type : TransactionType
}

impl Transaction {
    pub fn new(client : ClientId, id : TransactionId, transaction_type : TransactionType) -> Self {
        Transaction {
            client : client,
            tx : id,
            txn_type : transaction_type
        }
    }
}

