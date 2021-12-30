use csv::{Reader, StringRecordsIter};
use std::path::Path;
use std::fs::File;
use crate::payment::Transaction;

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
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        let next_result = self.records.next()?;
        let response = match next_result {
            Ok(next_record) => {
                match &next_record[0] {
                    "deposit" => {
                        Transaction::Deposit {
                            client: 1, tx : 1, amount: 0.0 
                        }
                    },
                    _ => {
                        None
                    }
                }
            },
            Err(_) => {
                None
            }
        }
    }
}

fn parse_f32(part : &str) -> f32 {
    part.parse().unwrap_or(0.0)
}