use std::{env};
use std::io;

mod readers;

use txnengine::transaction::{Transaction, TransactionEngine};
use readers::{CsvFileReader};

/// `process_reader` takes an iterator over Transaction. It does not
/// matter where the transactions are coming from.assert_eq!
/// 
/// Returns the TransactionEngine that holds the ending balances
/// of all customers after processing the iterator
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

fn filename_from_args() -> txnengine::Result<String> {
    if let Some(file_name) = env::args().nth(1) {
        return Ok(file_name);
    }

    return Err("Missing file name to process".into());
}

/// `write_balances` iterates over all custmers and serializes the 
///  output to the standard output
fn write_balances(engine : &TransactionEngine) -> txnengine::Result<()> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    
    for balance in engine.iter() {
        writer.serialize(balance)?;
    }

    Ok(())
}

/// The filename to process is passed as an argument.
/// 
/// It uses the CsvReader to read get an iterator over Transaction,
/// and applies each transaction onto the TransactionEngine
/// 
fn main() -> txnengine::Result<()> {
    let mut reader = CsvFileReader::new(&filename_from_args()?)?;
    let engine = process_reader(reader.iter());
    write_balances(&engine)?;

    Ok(())
}
