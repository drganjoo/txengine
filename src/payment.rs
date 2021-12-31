use core::ops::DerefMut;
use core::ops::Add;
use core::ops::{SubAssign, Deref, AddAssign};
use std::collections::HashMap;
use std::fmt;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize};

pub type ClientId = u16;
pub type TransactionId = u32;

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
    client : ClientId,
    tx : TransactionId,
    txn_type : TransactionType
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

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct Amount(f32);

impl Amount {
    pub fn new(init : f32) -> Amount {
        Amount(init)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, fmt : &mut std::fmt::Formatter<'_>) -> fmt::Result { 
        let formatted = format!("{:.4}", **self);
        fmt.write_str(&formatted)?;
        Ok(())
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Amount) -> Self::Output { 
        Amount(*self + *rhs)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        **self += *rhs;
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        **self -= *rhs;
    }
}

impl PartialEq for Amount {
    fn eq(&self, rhs: &Amount) -> bool {
        // two values are same if they match within 4 precision 
        (**self - **rhs).abs() < 0.0001
    }
}

impl Deref for Amount {
    type Target = f32;
    fn deref(&self) -> &Self::Target { 
        let Amount(value) = self;
        value
    }
}

impl DerefMut for Amount {
    fn deref_mut(&mut self) -> &mut Self::Target { 
        let Amount(value) = self;
        value
    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug)]
pub struct ClientBalance {
    client: ClientId,
    available : Amount,
    held : Amount,
    locked : bool
}

impl ClientBalance {
    pub fn new(client : ClientId) -> Self {
        ClientBalance {
            client: client,
            available: Amount(0.0),
            held: Amount(0.0),
            locked: false,
        }
    }

    pub fn total(&self) -> Amount {
        self.available + self.held
    }

    pub fn deposit(&mut self, amount: Amount) {
        self.available += amount;
    }
}

impl Serialize for ClientBalance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ClientBalance", 5)?;

        state.serialize_field("client", &self.client)?;
        state.serialize_field("available", &self.available)?;
        state.serialize_field("held", &self.held)?;
        state.serialize_field("total", &self.total())?;
        state.serialize_field("locked", &self.locked)?;

        state.end()
    }
}

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
            if ledger.balance.locked {
                return Err("Customer account is locked and transaction cannot be applied".into());
            }

            return TransactionEngine::apply_transaction(ledger, &transaction);
        }

        Err("Customer ledger not found".into())
    }

    fn apply_transaction(ledger : &mut ClientLedger, transaction: &Transaction) -> crate::Result<()> {
        let balance = &mut ledger.balance;

        match &transaction.txn_type {
            TransactionType::Deposit { amount } => {
                balance.deposit(*amount);

                ledger.transactions.insert(transaction.tx, *amount);
                println!("{}: deposited: {}, total: {}", transaction.client, amount, balance.total());
            },
            _ => {

            }
            // TransactionType::Withdrawal { amount } => {
            //     if balance.available < amount {
            //         return Err("Balance is less than the requested withdrawal amount".into());
            //     }

            //     balance.available -= amount;
            //     ledger.transactions.insert(transaction.tx, amount);
            //     println!("{}: withdrawal: {}, total: {}", transaction.client, amount, balance.total());
            // },
            // TransactionType::Dispute => {
            //     // If the tx specified by the dispute doesn't exist you can ignore it and 
            //     // assume this is an error on our partners side.
            //     if let Some(amount) = ledger.transactions.get(&transaction.tx) {
            //         // that the clients available funds should decrease by the amount disputed, 
            //         // their held funds should increase by the amount disputed,
            //         balance.available -= *amount;
            //         balance.held += *amount;

            //         println!("{}: dispute amount: {}, total: {}", transaction.client, amount, balance.total());
            //     }
            //     else {
            //         println!("transaction ID {} not found for customer {}", transaction.tx, transaction.client);
            //     }
            // },
            // TransactionType::Resolve => {
            //     // Funds that were previously disputed are no longer disputed. 
            //     // This means that the clients held funds should decrease by the amount no longer disputed,
            //     // their available funds should increase by the amount no longer disputed                
            //     if let Some(amount) = ledger.transactions.get(&transaction.tx) {
            //         balance.available += *amount;
            //         balance.held -= *amount;
            //     }
            // },
            // TransactionType::ChargeBack => {
            //     // A chargeback is the final state of a dispute and represents the client reversing a transaction. 
            //     // Funds that were held have now been withdrawn. This means that the clients held funds and total funds 
            //     // should decrease by the amount previously disputed.
            //     if let Some(amount) = ledger.transactions.get(&transaction.tx) {
            //         balance.held -= *amount;
            //         balance.locked = true;
            //     }
            // },
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

