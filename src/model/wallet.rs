use std::{error::Error, fmt::Display};

#[derive(Debug, Default, Clone)]
pub struct Wallet {
    balance: u64,
}

#[derive(Debug)]
pub struct NotEnoughMoneyErr {}

impl Display for NotEnoughMoneyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to take money from wallet while not having enough")
    }
}

impl Error for NotEnoughMoneyErr {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Wallet {
    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn add_money(&mut self, to_add: u64) -> &Self {
        self.balance = self.balance.checked_add(to_add).unwrap_or(u64::MAX);
        self
    }

    pub fn take_money(&mut self, to_take: u64) -> Result<&Self, NotEnoughMoneyErr> {
        self.balance = self
            .balance
            .checked_sub(to_take)
            .ok_or(NotEnoughMoneyErr {})?;
        Ok(self)
    }

    pub fn pay_to_do<F: FnOnce()>(
        &mut self,
        cost: u64,
        callback: F,
    ) -> Result<(), NotEnoughMoneyErr> {
        self.take_money(cost)?;
        callback();
        Ok(())
    }
}
