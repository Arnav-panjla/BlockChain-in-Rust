use std::collections::BTreeMap;
pub struct Pallet {
    balances: BTreeMap<String, u128>,
} 

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, who: &String, val: u128) {
        self.balances.insert(who.clone(), val);
    }
    pub fn get_balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }
    pub fn transfer(&mut self, from: &String, to: &String, val: u128) -> Result<(), &'static str> {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);

        let new_from_balance = from_balance
        .checked_sub(val)
        .ok_or("Insuffecient balcance")?;

        let new_to_balance = to_balance
        .checked_add(val)
        .ok_or("Overflow")?;

        self.set_balance(from, new_from_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[test]
fn transfer_balance() {
    let mut pallet = super::Pallet::new();
    pallet.set_balance(&"Alice".to_string(), 100);
    pallet.set_balance(&"Bob".to_string(), 100);
    pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).unwrap();
    assert!(pallet.get_balance(&"Alice".to_string()) == 50);
    assert!(pallet.get_balance(&"Bob".to_string()) == 150);
}

#[test]
fn transfer_balance_overflow() {
    let mut pallet = super::Pallet::new();
    pallet.set_balance(&"Alice".to_string(), 10);
    pallet.set_balance(&"Bob".to_string(), u128::MAX);
    assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).is_err());
}

#[test]
fn transfer_balance_insufficient() {
    let mut pallet = super::Pallet::new();
    pallet.set_balance(&"Alice".to_string(), 10);
    pallet.set_balance(&"Bob".to_string(), 10);
    assert!(pallet.transfer(&"Alice".to_string(), &"Bob".to_string(), 50).is_err());
}