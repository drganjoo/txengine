# Transaction Processing Engine

## Overview

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

