use serde::ser::{Serializer, SerializeStruct};
use serde::{Serialize};

use super::transaction::{ClientId};
use super::amount::Amount;

#[derive(Debug)]
pub struct ClientBalance {
    client: ClientId,
    available : Amount,
    held : Amount,
    locked : bool
}

impl ClientBalance {
    pub fn new(client : ClientId) -> Self {
        ClientBalance {
            client: client,
            available: Amount::new(0.0),
            held: Amount::new(0.0),
            locked: false,
        }
    }

    pub fn total(&self) -> Amount {
        self.available + self.held
    }

    pub fn deposit(&mut self, amount: Amount)  -> crate::Result<()> {
        self.check_locked()?;
        self.available += amount;
        Ok(())
    }

    pub fn withdrawal(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;
        if self.available < amount {
            return Err(format!("Balance {} is less than the requested withdrawal amount of {}", self.available, amount).into());
        }

        self.available -= amount;
        Ok(())
    }

    pub fn dispute(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;

        if self.available < amount {
            return Err(format!("Insufficient amount {} available to dispute {}", self.available, amount).into());
        }

        self.available -= amount;
        self.held += amount;

        Ok(())
    }

    pub fn resolve(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;

        if self.held < amount {
            return Err(format!("Insufficient amount {} held to resolve {}", self.held, amount).into());
        }

        self.available += amount;
        self.held -= amount;
    
        Ok(())
    }


    pub fn chargeback(&mut self, amount: Amount) -> crate::Result<()> {
        self.check_locked()?;

        if self.held < amount {
            return Err(format!("Insufficient amount {} held to chargeback {}", self.held, amount).into());
        }

        self.held -= amount;
        self.locked = true;
        
        Ok(())
    }

    fn check_locked(&self) -> crate::Result<()> {
        if self.locked {
            return Err("Customer account is locked and transaction cannot be applied".into());
        }
        Ok(())
    }
}

impl Serialize for ClientBalance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ClientBalance", 5)?;

        state.serialize_field("client", &self.client)?;
        state.serialize_field("available", &self.available)?;
        state.serialize_field("held", &self.held)?;
        state.serialize_field("total", &self.total())?;
        state.serialize_field("locked", &self.locked)?;

        state.end()
    }
}
