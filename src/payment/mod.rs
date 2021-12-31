use std::collections::HashMap;

pub mod transaction;
pub mod clientbalance;
pub mod amount;

use amount::Amount;
use clientbalance::ClientBalance;
use transaction::{Transaction, TransactionType, TransactionId, ClientId};

#[derive(Debug)]
struct ClientLedger {
    transactions : HashMap<TransactionId, Amount>,
    balance: ClientBalance
}

impl ClientLedger {
    pub fn new(client: ClientId) -> Self {
        ClientLedger {
            transactions : HashMap::new(),
            balance : ClientBalance::new(client)
        }
    }
}

#[derive(Debug)]
pub struct TransactionEngine {
    ledger: HashMap<ClientId, ClientLedger>
}

impl TransactionEngine {
    pub fn new() -> Self {
        TransactionEngine {
            ledger : HashMap::new()
        }
    }

    pub fn apply(&mut self, transaction : Transaction) -> crate::Result<()> {
        let mut client_ledger = self.ledger.get_mut(&transaction.client);

        if let None = client_ledger {
            let new_customer = ClientLedger::new(transaction.client);
            self.ledger.insert(transaction.client, new_customer);

            client_ledger = self.ledger.get_mut(&transaction.client);
        }

        if let Some(ledger) = client_ledger {
            return TransactionEngine::apply_transaction(ledger, &transaction);
        }

        Err("Customer ledger not found".into())
    }

    fn apply_transaction(ledger : &mut ClientLedger, transaction: &Transaction) -> crate::Result<()> {
        let balance = &mut ledger.balance;

        match &transaction.txn_type {
            TransactionType::Deposit { amount } => {
                balance.deposit(*amount)?;

                ledger.transactions.insert(transaction.tx, *amount);
                println!("{}: deposited: {}, total: {}", transaction.client, amount, balance.total());
            },
            TransactionType::Withdrawal { amount } => {
                balance.withdrawal(*amount)?;

                ledger.transactions.insert(transaction.tx, *amount);
                println!("{}: withdrawal: {}, total: {}", transaction.client, amount, balance.total());
            },
            TransactionType::Dispute => {
                // If the tx specified by the dispute doesn't exist you can ignore it and 
                // assume this is an error on our partners side.
                if let Some(amount) = ledger.transactions.get(&transaction.tx) {
                    balance.dispute(*amount)?;
                    println!("{}: dispute amount: {}, total: {}", transaction.client, amount, balance.total());
                }
                else {
                    println!("transaction ID {} not found for customer {}", transaction.tx, transaction.client);
                }
            },
            TransactionType::Resolve => {
                // Funds that were previously disputed are no longer disputed. 
                // This means that the clients held funds should decrease by the amount no longer disputed,
                // their available funds should increase by the amount no longer disputed                
                if let Some(amount) = ledger.transactions.get(&transaction.tx) {
                    balance.resolve(*amount)?;
                }
            },
            TransactionType::ChargeBack => {
                // A chargeback is the final state of a dispute and represents the client reversing a transaction. 
                // Funds that were held have now been withdrawn. This means that the clients held funds and total funds 
                // should decrease by the amount previously disputed.
                if let Some(amount) = ledger.transactions.get(&transaction.tx) {
                    balance.chargeback(*amount)?;
                }
            },
        }

        Ok(())
    }

    pub fn iter(&self) -> ClientIterator<'_> {
        ClientIterator {
            iter : self.ledger.iter()
        }
    }
}


pub struct ClientIterator<'a> {
    iter : std::collections::hash_map::Iter<'a, ClientId, ClientLedger>
}

impl<'a> Iterator for ClientIterator<'a> {
    type Item = &'a ClientBalance;
    
    fn next(&mut self) -> Option<Self::Item> { 
        let (_, client_ledger) = &self.iter.next()?;
        Some(&client_ledger.balance)
    }
}

