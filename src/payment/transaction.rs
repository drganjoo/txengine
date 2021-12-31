use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

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

impl<'a> Deserialize<'a> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'a> 
    { 
        // enum Field { TransactionType, Client, Id, Amount }

        struct TransactionVisitor;

        impl<'a> Visitor<'a> for TransactionVisitor {
            type Value = Transaction;
            
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
                formatter.write_str("struct Transaction")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                where
                    V: SeqAccess<'a>
            {
                let transaction_type = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let client = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let tx = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let amount  : &str = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                
                let amount_parsed = amount.trim().parse()
                    .or_else(|e| Err(de::Error::invalid_value(serde::de::Unexpected::Other(amount), &"a floating point value")))?;
                if amount_parsed < 0.0 {
                    return Err(de::Error::invalid_value(
                                serde::de::Unexpected::Float(amount_parsed as f64), 
                                &"a positive number"));
                }
    
                let transaction_amount = Amount::new(amount_parsed);
            
                let transaction = match transaction_type{
                    "deposit" => {
                        Transaction::new(client, tx, TransactionType::Deposit { amount: transaction_amount })
                    },
                    "withdrawal" => {
                        Transaction::new(client, tx, TransactionType::Withdrawal { amount: transaction_amount })
                    },
                    "dispute" => {
                        Transaction::new(client, tx, TransactionType::Dispute)
                    },
                    "resolve" => {
                        Transaction::new(client, tx, TransactionType::Resolve)
                    },
                    "chargeback" => {
                        Transaction::new(client, tx, TransactionType::ChargeBack)
                    },
                    invalid_type => {
                        return Err(de::Error::invalid_value(
                            serde::de::Unexpected::Other(invalid_type), &"type of known transaction"));
                    }
                };

                Ok(transaction)
            }
        }

        const FIELDS : &'static [&'static str] = &["type", "client", "tx", "amount"];
        deserializer.deserialize_struct("Transaction", FIELDS, TransactionVisitor)
    }
}