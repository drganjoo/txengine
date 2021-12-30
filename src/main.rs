use std::{env};
use std::path::Path;

mod payment;
mod readers;

use payment::Transaction;
use readers::{CsvFileReader};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn process_reader<T>(transcactions : T)
    where
    T : Iterator<Item = Transaction> 
{
    for t in transcactions {
        
    }
}

fn filename_from_args() -> Result<String> {
    if let Some(file_name) = env::args().nth(1) {
        return Ok(file_name);
    }

    return Err("Missing file name to process".into());
}

fn main() -> Result<()> {
    let mut reader = CsvFileReader::new(&filename_from_args()?)?;
    process_reader(reader.iter());

    return Ok(());
}
