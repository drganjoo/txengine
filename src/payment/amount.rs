use core::str::FromStr;
use std::fmt;
use core::ops::{Add, SubAssign, Deref, DerefMut, AddAssign};
use serde::{Serialize};
use serde::ser::{Serializer};

#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct Amount(f32);

impl Amount {
    pub fn new(init : f32) -> Amount {
        Amount(init)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, fmt : &mut std::fmt::Formatter<'_>) -> fmt::Result { 
        let formatted = format!("{:.4}", **self);
        fmt.write_str(&formatted)?;
        Ok(())
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Amount) -> Self::Output { 
        Amount(*self + *rhs)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        **self += *rhs;
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        **self -= *rhs;
    }
}

impl PartialEq for Amount {
    fn eq(&self, rhs: &Amount) -> bool {
        // two values are same if they match within 4 precision 
        (**self - **rhs).abs() < 0.0001
    }
}

impl Deref for Amount {
    type Target = f32;
    fn deref(&self) -> &Self::Target { 
        let Amount(value) = self;
        value
    }
}

impl DerefMut for Amount {
    fn deref_mut(&mut self) -> &mut Self::Target { 
        let Amount(value) = self;
        value
    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Amount {
    type Err = std::num::ParseFloatError;

    fn from_str(value: &str) -> Result<Self, Self::Err> { 
        let amount = value.parse::<f32>()?;
        Ok(Amount::new(amount))
    }
}