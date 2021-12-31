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
## Overview

Each line in the CSV is iterated over by `CsvFileReader` type, which uses `csv::Reader` 
to iterate and apply line by line. Each line is converted into a `Transaction` type 
by using serde::Deserializer.

In the end, all customer balances are iterated over by using the `iter` function of 
`TransactionEngine` type and outputing using `serde::Serialize` and `csv::Writer`.

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

A very few test cases have been written which can be run using the following command:

```
cargo test
```

A simple.csv file has been provided that can be run using:

```
cargo run -- simple.csv
```