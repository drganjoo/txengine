use std::{env};
use std::io;

mod payment;
mod readers;

use payment::transaction::{Transaction};
use payment::TransactionEngine;
use readers::{CsvFileReader};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

fn process_reader<T>(transcactions : T) -> TransactionEngine
    where
    T : Iterator<Item = Transaction> 
{
    let mut engine = TransactionEngine::new();

    for t in transcactions {
        if let Err(e) = engine.apply(t) {
            eprintln!("Error in applying transaction, {}", e);
        }
    }

    engine
}

fn filename_from_args() -> Result<String> {
    if let Some(file_name) = env::args().nth(1) {
        return Ok(file_name);
    }

    return Err("Missing file name to process".into());
}

fn write_balances(engine : &TransactionEngine) -> Result<()> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    
    for balance in engine.iter() {
        writer.serialize(balance)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut reader = CsvFileReader::new(&filename_from_args()?)?;
    let engine = process_reader(reader.iter());
    write_balances(&engine)?;

    return Ok(());
}
