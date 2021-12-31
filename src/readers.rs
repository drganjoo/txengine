use csv::{Reader, StringRecordsIter, StringRecord};
use std::path::Path;
use std::fs::File;
use crate::payment::{Transaction, Deposit, Withdrawal, ClientId, TransactionId};

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
            records : self.reader.records(),
            line_no : 1
        }
    }
}

impl<'a> Iterator for CsvFileIterator<'a> {
    type Item = Box<dyn Transaction>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_result = self.records.next()?;
        match next_result {
            Ok(next_record) => {
                self.line_no += 1;

                let result : crate::Result<Box<dyn Transaction>> = match &next_record[0] {
                    "deposit" => {
                        parse_deposit(&next_record)
                    },
                    "withdrawal" => {
                        parse_nonref_record(&next_record, Withdrawal::new)
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
                        println!("Line #: {}, {}", self.line_no, e);
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

fn parse_deposit(record : &StringRecord) -> crate::Result<Box<dyn Transaction>> {
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

fn parse_nonref_record<T : 'static>(record : &StringRecord, type_creator : impl Fn(ClientId, TransactionId, f32) -> T) -> crate::Result<Box<dyn Transaction>> 
    where T : Transaction + Sync + Send
{
    if record.len() < 4 {
        return Err("Record does not have enough parts".into());
    }

    let client : ClientId = record[1].trim().parse()?;
    let tx : TransactionId = record[2].trim().parse()?;
    let amount : f32 = record[3].trim().parse()?;

    if amount < 0.0 {
        return Err("Amount cannot be negative".into());
    }

    Ok(Box::new(type_creator(client, tx, amount)))
}
