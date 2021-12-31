use txnengine::transaction::{TransactionEngine, Transaction, TransactionType};
//use txnengine::transaction::ledger::LedgerError;
use txnengine::transaction::amount::Amount;

#[test]
fn balance_matches() -> txnengine::Result<()> {
    let mut engine = TransactionEngine::new();
    engine.apply(Transaction::new(1, 1, TransactionType::Deposit{ amount: Amount::new(10.0) }))?;
    engine.apply(Transaction::new(1, 2, TransactionType::Deposit{ amount: Amount::new(20.0) }))?;

    {
        let ledger = engine.get_ledger(1).ok_or(String::from("Ledger not found"))?;
        assert_eq!(ledger.get_balance().total(), 30.0);
    }

    engine.apply(Transaction::new(1, 3, TransactionType::Withdrawal{ amount: Amount::new(15.0) }))?;
    {
        let ledger = engine.get_ledger(1).ok_or(String::from("Ledger not found"))?;
        assert_eq!(ledger.get_balance().total(), 15.0);
    }

    let res = engine.apply(Transaction::new(1, 4, TransactionType::Withdrawal{ amount: Amount::new(16.0) }));
    {
        let ledger = engine.get_ledger(1).ok_or(String::from("Ledger not found"))?;
        assert_eq!(ledger.get_balance().total(), 15.0);

        match res {
            Ok(_) => {
                panic!("Should be an error");
            }
            Err(_) => {
            }
        }
    }

    Ok(())
}


#[test]
fn txn_not_found() -> txnengine::Result<()> {
    let mut engine = TransactionEngine::new();
    engine.apply(Transaction::new(1, 1, TransactionType::Deposit{ amount: Amount::new(10.0) }))?;
    engine.apply(Transaction::new(1, 2, TransactionType::Deposit{ amount: Amount::new(20.0) }))?;

    {
        let ledger = engine.get_ledger(1).ok_or(String::from("Ledger not found"))?;
        assert_eq!(ledger.get_balance().total(), 30.0);
    }

    engine.apply(Transaction::new(1, 1, TransactionType::Withdrawal{ amount: Amount::new(15.0) }))?;
    {
        let ledger = engine.get_ledger(1).ok_or(String::from("Ledger not found"))?;
        assert_eq!(ledger.get_balance().total(), 15.0);
    }

    let res = engine.apply(Transaction::new(1, 1, TransactionType::Withdrawal{ amount: Amount::new(16.0) }));
    {
        let ledger = engine.get_ledger(1).ok_or(String::from("Ledger not found"))?;
        assert_eq!(ledger.get_balance().total(), 15.0);

        match res {
            Ok(_) => {
                panic!("Should be an error");
            }
            Err(_) => {
            }
        }
    }

    Ok(())
}
