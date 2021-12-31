use csv::{Reader, DeserializeRecordsIter};
use std::fs::File;

use txnengine::transaction::{Transaction};

pub struct CsvFileReader {
    reader : Reader<File>,
}

impl CsvFileReader {
    pub fn new(path : &String) -> txnengine::Result<Self> {
        let rdr = csv::Reader::from_path(path)?;
        Ok(
            CsvFileReader {
                reader : rdr,
           }
        )
    }

    pub fn iter(&mut self) -> CsvFileIterator<'_> {
        CsvFileIterator {
            records : self.reader.deserialize(),
        }
    }
}

pub struct CsvFileIterator<'a> {
    records: DeserializeRecordsIter<'a, File, Transaction>,
}

impl<'a> Iterator for CsvFileIterator<'a> {
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        let next_result = self.records.next()?;
        match next_result {
            Ok(value) => Some(value),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}
