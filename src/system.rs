use::std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,

}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> u32 {
        self.block_number
    }
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }
    pub fn inc_nonce(&mut self , who: &String) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(),nonce+1);
    }
    pub fn get_nonce(&self, who:&String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(),1);

    }
}