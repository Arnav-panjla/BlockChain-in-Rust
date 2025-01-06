use std::collections::BTreeMap;
use num::traits::{Zero, One};
use num::{CheckedAdd, CheckedSub, Bounded};



pub trait Config {
    type AccountID: Ord + Clone;
    type Balance: Zero + One + CheckedAdd + CheckedSub + Copy + Bounded;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountID, T::Balance>,
} 

impl<T: Config> Pallet<T> 
{
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, who: &T::AccountID, val: T::Balance ) {
        self.balances.insert(who.clone(), val);
    }
    pub fn get_balance(&self, who: &T::AccountID) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
    pub fn transfer(&mut self, from: &T::AccountID, to: &T::AccountID, val: T::Balance) -> Result<(), &'static str> {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);

        let new_from_balance = from_balance
        .checked_sub(&val)
        .ok_or("Insuffecient balcance")?;

        let new_to_balance = to_balance
        .checked_add(&val)
        .ok_or("Overflow")?;

        self.set_balance(from, new_from_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod test {
        
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountID = String;
        type Balance = u32;
    }
    fn transfer_balance() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 100);
        pallet.set_balance(&"Bob".to_string(), 100);
        pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).unwrap();
        assert!(pallet.get_balance(&"Alice".to_string()) == 50);
        assert!(pallet.get_balance(&"Bob".to_string()) == 150);
    }

    #[test]
    fn transfer_balance_overflow() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 10);
        pallet.set_balance(&"Bob".to_string(), u32::MAX);
        assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).is_err());
    }

    #[test]
    fn transfer_balance_insufficient() {
        let mut pallet: super::Pallet<TestConfig> = super::Pallet::new();
        pallet.set_balance(&"Alice".to_string(), 10);
        pallet.set_balance(&"Bob".to_string(), 10);
        assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).is_err());
    }
}