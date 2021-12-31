use txnengine::transaction::{TransactionEngine, Transaction, TransactionType};
use txnengine::transaction::amount::Amount;

#[test]
fn balance_matches() -> txnengine::Result<()> {
    let mut engine = TransactionEngine::new();
    engine.apply(Transaction::new(1, 1, TransactionType::Deposit{ amount: Amount::new(10.0) }))?;
    engine.apply(Transaction::new(1, 1, TransactionType::Deposit{ amount: Amount::new(20.0) }))?;

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

        if let Err(e) = res;
        assert!(res == Err(e));
    }

    Ok(())
}
