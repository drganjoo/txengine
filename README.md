# Transaction Processing Engine

## Compiling the program

```
cargo build
```

## Running the program

The program expects one argument, the csv file that has the transactions in it:

```
cargo run -- simple.csv
```

## Sample Project

Given a CSV representing a series of transactions, this sample processes the payments crediting and debiting accounts. After processing the complete set of payments output the client account balances

The input file is the only argument to the binary.

A precision of four places past the decimal is guaranteed through out the program.

### Input

The input will be a CSV file with the columns 

type, client, tx, and amount. 

The type is a string, the client column is a valid u16 client ID, the tx is a valid u32 transaction ID, and the amount is a decimal value with a precision of up to four places past the decimal.

For example:

type, client, tx, amount,   
deposit, 1, 1, 1.0    
deposit, 2, 2, 2.0    
deposit, 1, 3, 2.0    
withdrawal, 1, 4, 1.5    
withdrawal, 2, 5, 3.0    

Output

The output should be a list of client IDs (client), available amounts (available), held amounts (held), total amounts (total), and whether the account is locked (locked). Columns are defined as:

**available**: The total funds that are available for trading, staking, withdrawal, etc. This should be equal to the total - held amounts

**held**: The total funds that are held for dispute. This should be equal to total available amounts

**total**: The total funds that are available or held. This should be equal to available + held

**locked**: Whether the account is locked. An account is locked if a charge back occurs

For example:

client, available, held, total, locked     
1, 1.5, 0.0, 1.5, false      
2, 2.0, 0.0, 2.0, false     


### Types of Transactions

**Deposit**: A deposit is a credit to the client's asset account

**Withdrawal**: A withdraw is a debit to the client's asset account

**Dispute**: A dispute represents a client's claim that a transaction was erroneous and should be reversed. The transaction shouldn't be reversed yet but the associated funds should be held. 

**Resolve**: A resolve represents a resolution to a dispute, releasing the associated held funds. Funds that were previously disputed are no longer disputed.

**Chargeback**: A chargeback is the final state of a dispute and represents the client reversing a transaction. Funds that were held have now been withdrawn.

## Solution Overview

From the engine's prespective, it does not matter whether data is coming from a CSV file, a database or from the network. All
it needs is an iterative way of going over the data.

For this reason, `main::process_reader`, takes an iterator of incoming transactions and applies each of them to the `TransactionEngine`.

Each line in the CSV is represented by the type `Transaction`. The rest of the program does not deal with individual csv lines.

`CsvFileReader` type processes the csv and it provides an `iter()` function to get an iterator that returns `Iterator<Type = Transaction>` type. Internally, it uses `csv::Reader` to iterate and apply line by line. Each line is converted into a `Transaction` type by using serde::Deserializer.

The transaction engine keeps all customer balances using `CustomerLedger` type. The `iter` method provides an `Iterator<Type=CustomerLedger` to get the customer balances. To write the ouput, `serde::Serialize` and `csv::Writer` are used.

### TransactionEngine

This type is used for processing transactions. It uses a ledger to maintain the running
balance of each customer.

### ClientLedger

This type maintains:

1) a map of all past deposit / withdrawal transactions to easily lookup 
disputes and resolutions.

2) an instance of ClientBalance to keep the current balance of the customer

### ClientBalance

This is used for keeping the current balance of the customer

## Testing

A few test cases have been written which can be run using the following command:

```
cargo test
```

A simple.csv file has been provided that can be run using:

```
cargo run -- simple.csv
```