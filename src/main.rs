mod balances;
mod system;

pub struct Runtime {
    balances: balances::Pallet,
    system: system::Pallet, 
}

impl Runtime {
    fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system:: Pallet::new(),
        }
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

    println!("Gud, Gud !!! ");
    println!("Everything is working fine");
}
