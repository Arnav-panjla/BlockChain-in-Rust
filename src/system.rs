use::std::collections::BTreeMap;
use std::ops::{AddAssign};

use num::{CheckedAdd, CheckedSub};
use num::traits::{Zero, One};



pub trait Config {
    type AccountID: Ord + Clone;
    type BlockNumber: Default + CheckedAdd + CheckedSub + Copy + AddAssign + Zero + One;
    type Nonce: Default + Clone + Copy + Zero + One + AddAssign;
    
}
#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountID, T::Nonce>,
}

impl<T:Config> Pallet<T> {
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }
    pub fn inc_nonce(&mut self , who: &T::AccountID) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who.clone(),nonce + T::Nonce::one());
    }
    pub fn get_nonce(&self, who:&T::AccountID) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}


#[cfg(test)]
mod test {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountID = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(),1);

    }
}