use std::collections::HashMap;

pub type ClientId = u16;
pub type TransactionId = u32;

// pub enum Transaction {
//     Deposit { client: u16, tx : u32, amount: f32 },
//     Withdraw { client: u16, tx : u32, amount: f32 },
//     Dispute { client: u16, disputed_tx : u32 },
//     Resolve { client: u16, disputed_tx : u32 },
//     ChargeBack { client: u16, disputed_tx : u32 },
// }

struct ClientBalance {
    available : f32,
    held : f32,
    locked : bool
}

pub enum TransactionType {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    ChargeBack,
}

pub trait Transaction {
    fn apply(&self, balance: &mut ClientBalance);
}

impl ClientBalance {
    pub fn new() -> Self {
        ClientBalance {
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }

    pub fn total(&self) -> f32 {
        self.available + self.held
    }
}

pub struct Deposit { 
    client: ClientId, 
    tx : TransactionId, 
    amount: f32 
}

impl Deposit {
    pub fn new(client : ClientId, tx : TransactionId, amount : f32) -> Self{
        Deposit {
            client : client,
            tx: tx,
            amount: amount
        }
    }
}

impl Transaction for Deposit {
    fn apply(&self, balance: &mut ClientBalance) {
        // meaning it should increase the available and total funds of the client account
        // not sure what to do in case of locked accounts?
        balance.available += self.amount;
    }
}



struct ClientLedger {
    // ordered hashmap is what we need here
    transactions : HashMap<TransactionId, Box<dyn Transaction>>
}


struct TransactionEngine {
    ledger: HashMap<ClientId, ClientLedger>
}

impl TransactionEngine {
    // fn deposit(tx : Transaction) {
    //     // meaning it should increase the available and total funds of the client account
    // }
    
    // fn withdraw(tx : Transaction){
    //     // meaning it should decrease the available and total funds of the client account
    //     // If a client does not have sufficient available funds the withdrawal should fail and the total amount of funds should not change
    // }

    // fn dispute(tx: Transaction) {
    //     // This means that the clients available funds should decrease by the amount disputed, their held funds 
    //     // should increase by the amount disputed, while their total funds should remain the same.
    //     // Notice that a dispute does not state the amount disputed. Instead a dispute references the transaction that is disputed by ID

    //     // If the tx specified by the dispute doesn't exist you can ignore it and assume this is an error on our partners side.
    // }

    // fn resolve(tx : Transaction) {
    //     // A resolve represents a resolution to a dispute, releasing the associated held funds. Funds that were 
    //     // previously disputed are no longer disputed. This means that the clients held funds should decrease by 
    //     // the amount no longer disputed, their available funds should increase by the amount no longer disputed, 
    //     // and their total funds should remain the same.

    //     // If the tx specified doesn't exist, or the tx isn't under dispute, you can ignore the resolve 
    //     // and assume this is an error on our partner's side.
    // }

    // fn charge_back(tx: Transaction) {
    //     // This means that the clients held funds and total funds should decrease by the amount previously disputed. 
    //     // If a chargeback occurs the client's account should be immediately frozen.

    //     // Like a resolve, if the tx specified doesn't exist, or the tx isn't under dispute, you can ignore chargeback 
    //     // and assume this is an error on our partner's side.
//    }
}

