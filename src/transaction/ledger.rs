//! ClientBalance
//!
//! Type [`ClientBalance`] holds the current balance of a client
//! 
//! |Field|Description|
//! |-|-|
//! |client|The Id of the client|
//! |available|Amount that is available for the client|
//! |held|Amount that has been disputed|
//! |locked|If a chargeback is transacted, the account is locked|
//! |total()|Gives the total amount that is available for the client|

use std::collections::HashMap;
use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize};
use std::fmt;

use super::{ClientId, TransactionId, Transaction, TransactionType};
use super::amount::Amount;

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
            available: Amount::new(0.0),
            held: Amount::new(0.0),
            locked: false,
        }
    }

    pub fn total(&self) -> Amount {
        self.available + self.held
    }

    /// Deposits money to the client account
    /// An Error [`Err(LedgerError::AccountLocked`] is returned in case the account is locked.
    pub fn deposit(&mut self, amount: Amount)  -> crate::Result<()> {
        self.check_locked()?;
        self.available += amount;
        Ok(())
    }

    /// Withdraws money from the client account.
    /// 
    /// An Error [`Err(LedgerError::AccountLocked`] is returned in case the account is locked.
    /// An Error [`Err(LedgerError::InsufficientFunds`] is returned in case the client does not have enough money to withdraw
    pub fn withdrawal(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;
        if self.available < amount {
            return Err(LedgerError::InsufficentFunds { available: self.available, requested: amount }.into());
        }

        self.available -= amount;
        Ok(())
    }

    /// Disputes a given amount from the customer account
    /// 
    /// The requested amount is subtracted from the available and added to held
    /// 
    /// An Error [`Err(LedgerError::AccountLocked`] is returned in case the account is locked.
    pub fn dispute(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;

        // don't know if this check is to be applied or not
        // if self.available < amount {
        //  return Err(LedgerError::InsufficentFunds { available: self.available, requested: amount }.into());
        // }

        self.available -= amount;
        self.held += amount;

        Ok(())
    }

    /// Disputes a given amount from the customer account
    /// 
    /// The requested amount is subtracted from the available and added to held
    /// 
    /// An Error [`Err(LedgerError::AccountLocked`] is returned in case the account is locked.
    pub fn resolve(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;

        if self.held < amount {
            return Err(format!("Insufficient amount {} held to resolve {}", self.held, amount).into());
        }

        self.available += amount;
        self.held -= amount;
    
        Ok(())
    }

    /// Chargeback is a dispute resolution which causes the account to be locked.
    /// Amount from held is subtracted.
    /// 
    /// An Error [`Err(LedgerError::AccountLocked`] is returned in case the account is already locked.
    pub fn chargeback(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;

        // not sure if this is to be done or not that the client cannot chargeback if the
        // held amount is < the amount
        // if self.held < amount {
        //     return Err(format!("Insufficient amount {} held, cannot chargeback {}", self.held, amount).into());
        // }

        self.held -= amount;
        self.locked = true;
        
        Ok(())
    }

    fn check_locked(&self) -> crate::Result<()> {
        if self.locked {
            return Err(LedgerError::AccountLocked.into());
        }
        Ok(())
    }
}

/// Serializer trait for ClientBalance
/// 
/// A custom serializer has been written instead of deriving from Serializer because 
/// the total field is method and not a member field. But we want to write that out as well
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

// todo: write a Deserializer for ClientBalance


#[derive(Debug)]
pub struct ClientLedger {
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

    pub fn get_balance_mut(&mut self) -> &mut ClientBalance {
        &mut self.balance
    }

    pub fn get_balance(&self) -> &ClientBalance {
        &self.balance
    }

    pub fn record_transaction(&mut self, transaction : &Transaction) {
        match transaction.txn_type {
            TransactionType::Deposit { amount } => {
                self.transactions.insert(transaction.tx, amount);
            },
            TransactionType::Withdrawal { amount } => {
                self.transactions.insert(transaction.tx, amount);
            }
            _ => {
                // nothing to record for any other type of transaction
            }
        }
    }

    pub fn get_past_transaction(&self, id : TransactionId) -> Option<&Amount> {
        self.transactions.get(&id)
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction) -> crate::Result<()> {
        match &transaction.txn_type {
            TransactionType::Deposit { amount } => {
                self.balance.deposit(*amount)?;
            },
            TransactionType::Withdrawal { amount } => {
                self.balance.withdrawal(*amount)?;
            },
            TransactionType::Dispute => {
                // If the tx specified by the dispute doesn't exist you can ignore it and 
                // assume this is an error on our partners side.
                if let Some(amount) = self.transactions.get(&transaction.tx) {
                    self.balance.dispute(*amount)?;
                }
            },
            TransactionType::Resolve => {
                // Funds that were previously disputed are no longer disputed. 
                // This means that the clients held funds should decrease by the amount no longer disputed,
                // their available funds should increase by the amount no longer disputed                
                if let Some(amount) = self.transactions.get(&transaction.tx) {
                    self.balance.resolve(*amount)?;
                }
            },
            TransactionType::ChargeBack => {
                // A chargeback is the final state of a dispute and represents the client reversing a transaction. 
                // Funds that were held have now been withdrawn. This means that the clients held funds and total funds 
                // should decrease by the amount previously disputed.
                if let Some(amount) = self.transactions.get(&transaction.tx) {
                    self.balance.chargeback(*amount)?;
                }
            },
        }

        self.record_transaction(transaction);

        Ok(())
    }
}

#[derive(Debug)]
pub enum LedgerError {
    InsufficentFunds { available: Amount, requested: Amount },
    CustomerMissing(ClientId),
    AccountLocked
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LedgerError::InsufficentFunds {available, requested } => {
                write!(f, "Balance {} is less than the requested withdrawal amount of {}", available, requested)
            },
            LedgerError::CustomerMissing(id) => {
                write!(f, "Ledger for customer {} could not be found", id)
            },
            LedgerError::AccountLocked => {
                write!(f, "Customer account is locked and transaction cannot be carried out")
            },
        }
    }
}

impl std::error::Error for LedgerError {}
