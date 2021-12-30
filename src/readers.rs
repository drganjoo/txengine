use csv::{Reader, StringRecordsIter, StringRecord};
use std::path::Path;
use std::fs::File;
use crate::payment::{Transaction, Deposit, ClientId, TransactionId};

pub struct CsvFileReader {
    reader : Reader<File>,
}

pub struct CsvFileIterator<'a> {
    records : StringRecordsIter<'a, File>
}

// todo:
// impl Drop for CsvFileReader {
// }

impl<'a> CsvFileReader {
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
            records: self.reader.records()
        }
    }
}

impl<'a> Iterator for CsvFileIterator<'a> {
    type Item = Box<dyn Transaction>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_result = self.records.next()?;
        match next_result {
            Ok(next_record) => {
                let result = match &next_record[0] {
                    "deposit" => {
                        parse_deposit(&next_record)
                    },
                    invalid_type => {
                        Err(format!("An invalid type of record '{}' is presnet in CSV file.", invalid_type).into())
                    }
                };

                match result {
                    Ok(transaction) => {
                        return Some(transaction);
                    },
                    Err(e) => {
                        println!("{}", e);
                        return None;
                    }
                }
            },
            Err(e) => {
                return None;
            }
        }
    }
}

fn parse_deposit(record : &StringRecord) -> crate::Result<Box<Deposit>> {
    if record.len() < 4 {
        return Err("Deposit record does not have enough parts".into());
    }

    let client : ClientId = record[1].trim().parse()?;
    let tx : TransactionId = record[2].trim().parse()?;
    let amount : f32 = record[3].trim().parse()?;

    if amount < 0.0 {
        return Err("Deposit amounts cannot be negative".into());
    }

    Ok(Box::new(Deposit::new(client, tx, amount)))
}