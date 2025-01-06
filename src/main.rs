mod balances;
mod system;
mod support;

mod types {
    use crate::support;
    pub type AccountID = String;
    pub type Balance = u32;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountID, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header,Extrinsic>;


}   
pub enum RuntimeCall {
    
}


impl system::Config for Runtime {
    type AccountID = types::AccountID;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountID = types::AccountID;
    type Balance = types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Runtime>,
    system: system::Pallet<Runtime>, 
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system:: Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {

        self.system.inc_block_number();

        if (self.system.block_number() != block.header.block_number){
            return Err("Invalid block number");
        }

        for (i, support::Extrinsic{caller, call}) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);

            }
        Ok(())

    }

}





fn main() {
    let mut runtime = Runtime::new();

    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let charlie = "Charlie".to_string();    

    runtime.balances.set_balance(&alice, 100);
    runtime.balances.set_balance(&bob, 100);
    runtime.balances.set_balance(&charlie, 100);

    runtime.system.inc_block_number();
    runtime.system.inc_nonce(&alice);
    assert_eq!(runtime.system.get_nonce(&alice), 1);
    assert_eq!(runtime.system.block_number(), 1);

    let _ = runtime.balances
    .transfer(&alice, &bob, 50)
    .map_err(|e| println!("Error: {}", e));


    println!("{:#?}",runtime);
    println!("Gud, Gud !!! ");
    println!("Everything is working fine");
}
