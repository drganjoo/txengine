//! `txengine::TransactionEngine`
//! 
//! Applies given transcations to customer accounts. A ledger is maintained
//! for each customer, which keeps a track of all transactions that have
//! been applied to the account and the current balance
use std::collections::HashMap;
use amount::Amount;
use core::str::FromStr;
use std::fmt::Debug;
use serde::de::{self, Deserializer, Visitor, MapAccess};
use serde::{Deserialize};
use ledger::{ClientBalance, ClientLedger};

pub type ClientId = u16;
pub type TransactionId = u32;

pub mod ledger;
pub mod amount;

#[derive(Debug)]
pub struct TransactionEngine {
    ledger: HashMap<ClientId, ClientLedger>
}

/// `TransactionEngine` is used for keeping all customer accounts
/// and is the main entry point for the package.
/// 
/// It provides an `iter` function to iterate over all client accounts
impl TransactionEngine {
    pub fn new() -> Self {
        TransactionEngine {
            ledger : HashMap::new()
        }
    }

    /// Given a transaction it applies it to the given client
    /// In case a client account is not found, a new one is created
    /// 
    /// Certain errors can be returned from this, e.g.
    ///     LedgerError::InsufficentFund
    ///     LedgerError::AccountLocked
    pub fn apply(&mut self, transaction : Transaction) -> crate::Result<()> {
        let mut client_ledger = self.ledger.get_mut(&transaction.client);

        if let None = client_ledger {
            let new_customer = ClientLedger::new(transaction.client);
            self.ledger.insert(transaction.client, new_customer);

            client_ledger = self.ledger.get_mut(&transaction.client);
        }

        let ledger = client_ledger.ok_or("Customer ledger not found")?;
        ledger.apply_transaction(&transaction)
    }

    /// Provides an itereator over all customer accounts. There is no
    /// particular order in which the accounts are returned
    pub fn iter(&self) -> ClientIterator<'_> {
        ClientIterator {
            iter : self.ledger.iter()
        }
    }

    /// `get_ledger` can be used to get the ledger of a particular client
    pub fn get_ledger(&self, client : ClientId) -> Option<&ClientLedger> {
        self.ledger.get(&client)
    }
}


pub struct ClientIterator<'a> {
    iter : std::collections::hash_map::Iter<'a, ClientId, ClientLedger>
}

impl<'a> Iterator for ClientIterator<'a> {
    type Item = &'a ClientBalance;
    
    fn next(&mut self) -> Option<Self::Item> { 
        let (_, client_ledger) = &self.iter.next()?;
        Some(&client_ledger.get_balance())
    }
}


/// `TransactionType` represents the particular transactions that
/// can be applied to an ClientAccount
#[derive(Debug)]
pub enum TransactionType {
    Deposit { amount: Amount },
    Withdrawal { amount: Amount },
    Dispute, 
    Resolve,
    ChargeBack,
}

/// `Transaction` represents the details of a particular transaction
/// Although we could have kept amount here as an Option<f32> and keep
/// that as None for Dispute, Resolve and ChargeBack but keeping the
/// transaction type separate kind of gaurantees us that no one will
/// ever set the amount for these type transactions
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

impl<'a> Deserialize<'a> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'a> 
    { 
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { 
            Type,
            Client, 
            Tx, 
            Amount 
        }

        struct TransactionVisitor;

        impl<'a> Visitor<'a> for TransactionVisitor {
            type Value = Transaction;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
                formatter.write_str("map")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Transaction, V::Error>
            where
                V: MapAccess<'a>,
            {
                // define all field options and check afterwards to make
                // suree all are present in a given record
                let mut transaction_field : Option<&str> = None;
                let mut client_field : Option<ClientId> = None;
                let mut tx_id_field : Option<TransactionId> = None;
                let mut amount_field : Option<Amount> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            transaction_field = map.next_value()?;
                        },
                        Field::Client => {
                            client_field = parse_next(&mut map)?;
                        },
                        Field::Tx => {
                            tx_id_field = parse_next(&mut map)?;
                        },
                        Field::Amount => {
                            amount_field = parse_next(&mut map)?;
                        },
                    }
                }
                
                // each transaction must have a transaction type, client and transaction ID
                let txn_type = transaction_field.ok_or(de::Error::missing_field("type"))?;
                let client = client_field.ok_or(de::Error::missing_field("client"))?;
                let tx_id = tx_id_field.ok_or(de::Error::missing_field("tx"))?;

                let transaction = match txn_type{
                    "withdrawal" | "deposit" => {
                        // a deposit / withdrawal must have the amount field in the incoming
                        // record
                        let amount = amount_field.ok_or(de::Error::missing_field("amount"))?;
                        if *amount < 0.0 {
                            return Err(de::Error::invalid_value(
                                        serde::de::Unexpected::Float(*amount as f64), 
                                        &"a positive number"));
                        }
            
                        if txn_type == "deposit" {
                            Transaction::new(client, tx_id, TransactionType::Deposit { amount: amount })
                        }
                        else {
                            Transaction::new(client, tx_id, TransactionType::Withdrawal { amount: amount })
                        }
                    },
                    "dispute" => {
                        Transaction::new(client, tx_id, TransactionType::Dispute)
                    },
                    "resolve" => {
                        Transaction::new(client, tx_id, TransactionType::Resolve)
                    },
                    "chargeback" => {
                        Transaction::new(client, tx_id, TransactionType::ChargeBack)
                    },
                    invalid_type => {
                        return Err(de::Error::invalid_value(
                            serde::de::Unexpected::Other(invalid_type), &"type of known transaction"));
                    }
                };

                Ok(transaction)
            }
        }

        // define fields that should be present in the map
        const FIELDS : &'static [&'static str] = &["type", "client", "tx", "amount"];
        deserializer.deserialize_struct("Transaction", FIELDS, TransactionVisitor)
    }
}

/// `parse_next` is a generic that is ued for triming and then converting
/// the string part into the given type T
fn parse_next<'a, V, T>(map: &mut V) -> Result<Option<T>, V::Error>
where
    V: MapAccess<'a>,
    T: std::str::FromStr,
    <T as FromStr>::Err : std::fmt::Display
{
    // trim the sting part and in case it is empty return None, this is
    // particular useful for amount field as that is not present in the
    // dispute, resolve and chargeback transactions
    let trimmed_val = map.next_value::<&str>()?.trim();
    if trimmed_val.len() == 0 {
        return Ok(None);
    }

    // try to convert the value and in case it cannot be converted
    // return a deserialization error
    let parsed_val = trimmed_val.parse::<T>()
        .map_err(|e| de::Error::invalid_value(
            serde::de::Unexpected::Other(&format!("Cannot parse {} as {}", trimmed_val, e)), 
            &"a positive number"))?;

    Ok(Some(parsed_val))
}

