use csv::{Reader, StringRecordsIter, StringRecord};
use std::fs::File;
use crate::payment::{Transaction, TransactionType, ClientId, TransactionId, Amount};

pub struct CsvFileReader {
    reader : Reader<File>,
}

pub struct CsvFileIterator<'a> {
    records : StringRecordsIter<'a, File>,
    line_no : u32
}

// todo:
// impl Drop for CsvFileReader {
// }

impl CsvFileReader {
    pub fn new(path : &String) -> crate::Result<Self> {
        let rdr = csv::Reader::from_path(path)?;
        Ok(
            CsvFileReader {
                reader : rdr,
           }
        )
    }

    pub fn iter(&mut self) -> CsvFileIterator<'_> {
        CsvFileIterator {
            records : self.reader.records(),
            line_no : 1
        }
    }
}

impl<'a> Iterator for CsvFileIterator<'a> {
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        let next_result = self.records.next()?;
        match next_result {
            Ok(record) => {
                self.line_no += 1;

                if record.len() < 2 {
                    eprintln!("Deposit record does not have enough parts");
                    return None;
                }
            
                let result = parse_line(&record);

                match result {
                    Ok(transaction) => {
                        return Some(transaction);
                    },
                    Err(e) => {
                        eprintln!("Line #: {}, {}", self.line_no, e);
                        return None;
                    }
                }
            },
            Err(_) => {
                // the file has ended?
                return None;
            }
        }
    }
}

fn parse_line(record: &StringRecord) -> crate::Result<Transaction> {
    let client : ClientId = record[1].trim().parse()?;
    let tx : TransactionId = record[2].trim().parse()?;

    let result = match &record[0] {
        "deposit" => {
            Transaction::new(client, tx, TransactionType::Deposit { amount: parse_amount(&record)? })
        },
        "withdrawal" => {
            Transaction::new(client, tx, TransactionType::Withdrawal { amount: parse_amount(&record)? })
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
            return Err(format!("An invalid type of record '{}' is presnet in CSV file.", invalid_type).into())
        }
    };

    Ok(result)
}

fn parse_amount(record : &StringRecord) -> crate::Result<Amount> {
    let amount : f32 = record[3].trim().parse()?;
    if amount < 0.0 {
        return Err("Deposit amounts cannot be negative".into());
    }

    Ok(Amount::new(amount))
}