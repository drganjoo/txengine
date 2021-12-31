use csv::{Reader, DeserializeRecordsIter};
use std::fs::File;

use txnengine::transaction::{Transaction};

/// `CsvFileReader` is used for reading from a csv based transaction
/// file. The `iter` method returns an iterator that provides an iterator
/// over all transactions found in the file
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

    /// returns an iterator that provides Transaction records
    pub fn iter(&mut self) -> CsvFileIterator<'_> {
        // initialize the Csv file reader and iterate over when
        // someone calls next on it
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

    /// returns None when there are no more records in the file
    /// 
    /// In case of error, it prints to the error stream
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
